use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use tinyverse_lib::{
    ListSessionsOptions, SessionStore, SpawnSessionOptions, TmuxClient, UpsertAgentServiceInput,
};

use crate::commands::config::store::TinyverseConfig;

const OPENCODE_PROVIDER_KEY: &str = "opencode";
const SERVER_CONNECT_TIMEOUT: Duration = Duration::from_millis(600);
const SERVER_BOOT_WAIT: Duration = Duration::from_millis(350);
const SERVER_BOOT_RETRIES: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedOpencodeService {
    pub provider_key: String,
    pub tmux_session_name: String,
    pub tmux_pane_id: Option<String>,
    pub hostname: String,
    pub port: u16,
    pub base_url: String,
}

pub fn ensure_managed_opencode_service(
    store: &mut SessionStore,
    config: &TinyverseConfig,
) -> Result<Option<ManagedOpencodeService>> {
    let Some(spec) = service_spec_from_config(config) else {
        debug!("managed opencode service disabled in config");
        return Ok(None);
    };

    debug!(
        "ensuring managed opencode service (bind_host={}, connect_host={}, port={}, tmux_session={})",
        spec.bind_hostname, spec.connect_host, spec.port, spec.tmux_session_name
    );

    let saved = store.find_agent_service(OPENCODE_PROVIDER_KEY)?;

    if let Some(saved) = saved.as_ref()
        && saved.hostname == spec.bind_hostname
        && saved.port == i32::from(spec.port)
        && probe_server(&spec.connect_host, spec.port)
    {
        debug!("managed opencode service already healthy; reusing saved record");
        return Ok(Some(map_saved(saved)));
    }

    let tmux_client = TmuxClient::new();
    if let Some(saved) = saved.as_ref() {
        warn!(
            "managed opencode service record exists but is stale/unreachable; recycling tmux session {}",
            saved.tmux_session_name
        );
        let _ = tmux_client.kill_session(saved.tmux_session_name.clone().into());
    }
    let _ = tmux_client.kill_session(spec.tmux_session_name.clone().into());

    let command = format!(
        "opencode serve --hostname {} --port {}",
        spec.bind_hostname, spec.port
    );
    let mut options = SpawnSessionOptions::new(&spec.tmux_session_name);
    options.agent_command = Some(command);
    let spawned = tmux_client.spawn_session(options).with_context(|| {
        format!(
            "failed to spawn managed opencode server tmux session `{}`",
            spec.tmux_session_name
        )
    })?;
    info!(
        "spawned managed opencode server session {}",
        spec.tmux_session_name
    );

    wait_for_server(&spec.connect_host, spec.port).with_context(|| {
        format!(
            "spawned managed opencode server but cannot connect to {}:{}",
            spec.connect_host, spec.port
        )
    })?;
    info!("managed opencode server became reachable at {}", spec.base_url);

    let stored = store.upsert_agent_service(&UpsertAgentServiceInput {
        provider_key: OPENCODE_PROVIDER_KEY.to_owned(),
        tmux_session_name: spec.tmux_session_name,
        tmux_pane_id: Some(spawned.agent_pane_id),
        hostname: spec.bind_hostname,
        port: spec.port,
        base_url: spec.base_url,
    })?;

    Ok(Some(map_saved(&stored)))
}

pub fn restart_managed_opencode_service(
    store: &mut SessionStore,
    config: &TinyverseConfig,
) -> Result<Option<ManagedOpencodeService>> {
    info!("restarting managed opencode service");
    if let Some(saved) = store.find_agent_service(OPENCODE_PROVIDER_KEY)? {
        debug!(
            "killing prior managed opencode tmux session {}",
            saved.tmux_session_name
        );
        let _ = TmuxClient::new().kill_session(saved.tmux_session_name.into());
        store.delete_agent_service(OPENCODE_PROVIDER_KEY)?;
    }

    ensure_managed_opencode_service(store, config)
}

pub fn lookup_managed_opencode_service(
    store: &mut SessionStore,
) -> Result<Option<ManagedOpencodeService>> {
    Ok(store
        .find_agent_service(OPENCODE_PROVIDER_KEY)?
        .map(|saved| map_saved(&saved)))
}

pub fn service_is_reachable(service: &ManagedOpencodeService) -> bool {
    let connect_host = normalize_connect_host(&service.hostname);
    probe_server(&connect_host, service.port)
}

pub fn tmux_session_is_live(session_name: &str) -> bool {
    let sessions = TmuxClient::new().list_sessions(ListSessionsOptions);
    let Ok(sessions) = sessions else {
        return false;
    };

    sessions.iter().any(|item| item.session_name == session_name)
}

fn map_saved(saved: &tinyverse_lib::StoredAgentService) -> ManagedOpencodeService {
    ManagedOpencodeService {
        provider_key: saved.provider_key.clone(),
        tmux_session_name: saved.tmux_session_name.clone(),
        tmux_pane_id: saved.tmux_pane_id.clone(),
        hostname: saved.hostname.clone(),
        port: saved.port.max(1) as u16,
        base_url: saved.base_url.clone(),
    }
}

#[derive(Debug, Clone)]
struct ServiceSpec {
    bind_hostname: String,
    connect_host: String,
    port: u16,
    tmux_session_name: String,
    base_url: String,
}

fn service_spec_from_config(config: &TinyverseConfig) -> Option<ServiceSpec> {
    if !config.opencode.server.enabled {
        return None;
    }

    let bind_hostname = non_empty_or(config.opencode.server.hostname.trim(), "127.0.0.1");
    let port = config.opencode.server.port.max(1);
    let tmux_session_name = non_empty_or(
        config.opencode.server.tmux_session_name.trim(),
        "tinyverse_opencode_server",
    );
    let connect_host = normalize_connect_host(&bind_hostname);
    let base_url = format!("http://{connect_host}:{port}");

    Some(ServiceSpec {
        bind_hostname,
        connect_host,
        port,
        tmux_session_name,
        base_url,
    })
}

fn non_empty_or(value: &str, fallback: &str) -> String {
    if value.is_empty() {
        fallback.to_owned()
    } else {
        value.to_owned()
    }
}

fn normalize_connect_host(bind_hostname: &str) -> String {
    match bind_hostname.trim() {
        "" => "127.0.0.1".to_owned(),
        "0.0.0.0" => "127.0.0.1".to_owned(),
        "::" => "::1".to_owned(),
        value => value.to_owned(),
    }
}

fn wait_for_server(host: &str, port: u16) -> Result<()> {
    for _ in 0..SERVER_BOOT_RETRIES {
        if probe_server(host, port) {
            return Ok(());
        }

        std::thread::sleep(SERVER_BOOT_WAIT);
    }

    error!(
        "managed opencode server did not become reachable (host={}, port={})",
        host, port
    );

    anyhow::bail!("server did not become reachable in time")
}

fn probe_server(host: &str, port: u16) -> bool {
    resolve_addrs(host, port)
        .into_iter()
        .any(|addr| TcpStream::connect_timeout(&addr, SERVER_CONNECT_TIMEOUT).is_ok())
}

fn resolve_addrs(host: &str, port: u16) -> Vec<SocketAddr> {
    let input = format!("{host}:{port}");
    match input.to_socket_addrs() {
        Ok(iter) => iter.collect(),
        Err(_) => Vec::new(),
    }
}

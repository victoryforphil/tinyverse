diesel::table! {
    tinyverse_sessions (id) {
        id -> Integer,
        session_key -> Text,
        session_name -> Text,
        agent_type -> Text,
        description -> Nullable<Text>,
        status_string -> Text,
        tmux_session_name -> Text,
        tmux_session_id -> Nullable<Text>,
        console_pane_id -> Nullable<Text>,
        agent_pane_id -> Nullable<Text>,
        agent_base_url -> Nullable<Text>,
        agent_session_id -> Nullable<Text>,
        created_at -> Timestamp,
        last_message_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tinyverse_agent_services (id) {
        id -> Integer,
        provider_key -> Text,
        tmux_session_name -> Text,
        tmux_pane_id -> Nullable<Text>,
        hostname -> Text,
        port -> Integer,
        base_url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

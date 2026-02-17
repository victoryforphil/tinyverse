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
        created_at -> Timestamp,
        last_message_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

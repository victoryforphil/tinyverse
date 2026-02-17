//! Reusable Ratatui component primitives and composites.

mod composites;
mod primitives;

pub use composites::card_grid_component::CardGridComponent;
pub use composites::chat::chat_composer::{ChatComposerComponent, ChatComposerProps};
pub use composites::chat::chat_conversation_header::{
    ChatConversationHeaderComponent, ChatConversationHeaderProps, ChatStatusTone,
};
pub use composites::chat::chat_message_list::{
    ChatMessageListComponent, ChatMessageListProps, ChatPalette,
};
pub use composites::chat::chat_types::{ChatMessageEntry, ChatMessageRole};
pub use composites::file_tree::{
    FileTreeBadge, FileTreeComponent, FileTreeLayout, FileTreeProps, FileTreeRow,
    FileTreeRowHitbox, tree_prefix,
};
pub use composites::popup_overlay::{
    PopupAnchor, PopupHit, PopupItem, PopupOverlay, PopupOverlayProps,
};
pub use primitives::code_view::{CodeViewComponent, CodeViewLine, CodeViewMode, CodeViewProps};
pub use primitives::diff_view::{
    DiffLine, DiffLineKind, DiffLineNumberMode, DiffViewComponent, DiffViewProps,
};
pub use primitives::footer_bar::{FooterBar, FooterBarProps};
pub use primitives::key_hint_bar::{KeyBind, KeyHintBar};
pub use primitives::labeled_field::LabeledField;
pub use primitives::loading_spinner::LoadingSpinner;
pub use primitives::modal_overlay::{ModalOverlay, ModalOverlayLayout, ModalOverlayProps};
pub use primitives::pane_block_component::PaneBlockComponent;
pub use primitives::section_header::SectionHeader;
pub use primitives::status_pill::StatusPill;

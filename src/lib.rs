pub mod hotkeys_provider;
pub mod types;
pub mod use_hotkeys;
pub mod macros;

pub use use_hotkeys::{use_hotkeys, use_hotkeys_scoped};

pub use hotkeys_provider::{use_hotkeys_context, HotkeysContext, HotkeysProvider};

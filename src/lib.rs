pub mod hotkeys_provider;
pub mod types;
pub mod use_hotkeys;
pub mod macros;
pub mod prelude;

pub use use_hotkeys::{use_hotkeys_scoped, use_hotkeys_ref_scoped};

pub use hotkeys_provider::{use_hotkeys_context, HotkeysContext, HotkeysProvider};

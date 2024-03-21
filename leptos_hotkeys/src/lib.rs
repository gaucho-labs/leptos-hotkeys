mod hotkeys_provider;
mod macros;
mod types;
mod use_hotkeys;

pub use hotkeys_provider::{provide_hotkeys_context, use_hotkeys_context, HotkeysContext};
pub use types::{Hotkey, KeyboardModifiers};
pub use use_hotkeys::{use_hotkeys_ref_scoped, use_hotkeys_scoped};

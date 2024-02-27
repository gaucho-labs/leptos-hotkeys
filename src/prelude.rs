use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(feature = "hydrate", feature= "csr"))] {
        pub use crate::use_hotkeys::{use_hotkeys_ref_scoped, use_hotkeys_scoped};
        pub use crate::hotkeys_provider::{use_hotkeys_context, HotkeysContext, HotkeysProvider};
        pub use crate::scopes;
        pub use crate::{use_hotkeys, use_hotkeys_ref};
        pub use std::collections::HashSet;
    }
}

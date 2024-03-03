use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(feature = "hydrate", feature= "csr"))] {
        use leptos::{ev::DOMEventResponder, html::ElementDescriptor, *};
        use leptos_dom::NodeRef;
        use web_sys::KeyboardEvent;
        use crate::hotkeys_provider::use_hotkeys_context;
        use crate::types::{Hotkey, KeyboardModifiers};

        use std::collections::{HashMap, HashSet};

        fn parse_key(key_combination: &str) -> Hotkey {
            let parts = key_combination
                .split('+')
                .map(str::trim)
                .collect::<Vec<&str>>();

            let mut modifiers = KeyboardModifiers::default();
            let mut keys = Vec::new();

            for part in parts {
                match part.to_lowercase().as_str() {
                    "ctrl" => modifiers.ctrl = true,
                    "alt" => modifiers.alt = true,
                    "meta" => modifiers.meta = true,
                    "shift" => modifiers.shift = true,
                    key => keys.push(key.to_lowercase().to_string()),
                }
            }

            Hotkey { modifiers, keys }
        }

        fn is_hotkey_match(hotkey: &Hotkey, pressed_keyset: &mut HashMap<String, KeyboardEvent>) -> bool {
            let mut modifiers_match = true;

            if hotkey.modifiers.ctrl {
                modifiers_match &= pressed_keyset.contains_key("control");
            }

            if hotkey.modifiers.shift {
                modifiers_match &= pressed_keyset.contains_key("shift");
            }

            if hotkey.modifiers.meta {
                modifiers_match &= pressed_keyset.contains_key("meta");
            }

            if hotkey.modifiers.alt {
                modifiers_match &= pressed_keyset.contains_key("alt");
            }

            let keys_match = hotkey.keys.iter().all(|key| {
                if let Some(event) = pressed_keyset.get_mut(key) {
                    if modifiers_match {
                        event.prevent_default();
                        return true;
                    }
                } else {
                    false
                }
            });

            modifiers_match && keys_match
        }

        pub fn use_hotkeys_scoped(
            key_combination: String,
            on_triggered: Callback<()>,
            scopes: Vec<String>,
        ) {
            let parsed_keys: HashSet<Hotkey> = key_combination
                .split(',')
                .map(|key_combo| parse_key(key_combo))
                .collect();

            let hotkeys_context = use_hotkeys_context();
            let pressed_keys = hotkeys_context.pressed_keys;

            create_effect(move |_| {
                let active_scopes = hotkeys_context.active_scopes.get();

                //intersection should be O(min(scopes, active_scopes))
                let within_scope = &scopes.iter().any(|scope| active_scopes.contains(scope));

                if *within_scope {
                    let mut pressed_keyset = pressed_keys.get();
                    if parsed_keys
                        .iter()
                        .any(|hotkey| is_hotkey_match(hotkey, &mut pressed_keyset))
                    {
                        Callable::call(&on_triggered, ());
                    }
                }
            });
        }

        pub fn use_hotkeys_ref_scoped<T>(
            key_combination: String,
            on_triggered: Callback<()>,
            scopes: Vec<String>,
        ) -> NodeRef<T>
        where
            T: ElementDescriptor + 'static + Clone,
        {
            let node_ref = create_node_ref::<T>();

            create_effect(move |_| {
                let parsed_keys: HashSet<Hotkey> = key_combination
                    .split(',')
                    .map(|key_combo| parse_key(key_combo))
                    .collect();
                let scopes = scopes.clone();
                if let Some(element) = node_ref.get() {
                    let keydown_closure = move |_event: KeyboardEvent| {
                        let hotkeys_context = use_hotkeys_context();
                        let active_scopes = hotkeys_context.active_scopes.get();
                        let mut pressed_keys = hotkeys_context.pressed_keys.get();
                        let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

                        if within_scope {
                            if parsed_keys
                                .iter()
                                .any(|hotkey| is_hotkey_match(hotkey, &mut pressed_keys))
                            {
                                Callable::call(&on_triggered, ());
                            }
                        }
                    };

                    let _ = element.add(ev::keypress, keydown_closure);
                }
            });

            node_ref
        }
    }
}

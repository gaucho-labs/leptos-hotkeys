use leptos::{html::ElementDescriptor, *};

pub fn use_hotkeys_scoped(
    #[cfg_attr(feature = "ssr", allow(unused_variables))] key_combination: String,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] on_triggered: Callback<()>,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] scopes: Vec<String>,
) {
    #[cfg(not(feature = "ssr"))]
    {
        use crate::hotkey::{is_hotkey_match, is_last_key_match};
        use crate::{use_hotkeys_context, Hotkey};
        use std::collections::HashSet;

        let parsed_keys: HashSet<Hotkey> = key_combination.split(',').map(Hotkey::new).collect();

        let hotkeys_context = use_hotkeys_context();
        let pressed_keys = hotkeys_context.pressed_keys;

        create_effect(move |_| {
            let active_scopes = hotkeys_context.active_scopes.get();
            let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

            if !within_scope {
                return;
            }

            let mut pressed_keyset = pressed_keys.get();
            if !is_last_key_match(&pressed_keyset, &parsed_keys) {
                return;
            }

            if let Some(matching_hotkey) = parsed_keys
                .iter()
                .find(|hotkey| is_hotkey_match(hotkey, &mut pressed_keyset.keys))
            {
                if cfg!(feature = "debug") {
                    let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                    web_sys::console::log_2(
                        &wasm_bindgen::JsValue::from_str(&message),
                        &wasm_bindgen::JsValue::from_str("color: #39FF14;"),
                    );
                }
                Callable::call(&on_triggered, ());
            }
        });
    }
}

pub fn use_hotkeys_ref<T>(
    #[cfg_attr(feature = "ssr", allow(unused_variables))] node_ref: NodeRef<T>,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] key_combination: String,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] on_triggered: Callback<()>,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] scopes: Vec<String>,
) where
    T: ElementDescriptor + 'static + Clone,
{
    #[cfg(not(feature = "ssr"))]
    create_effect(move |_| {
        use crate::hotkey::{is_hotkey_match, is_last_key_match};
        use crate::{use_hotkeys_context, Hotkey};
        use leptos::ev::DOMEventResponder;
        use std::collections::HashSet;

        let parsed_keys: HashSet<Hotkey> = key_combination.split(',').map(Hotkey::new).collect();
        let scopes = scopes.clone();
        if let Some(element) = node_ref.get() {
            let keydown_closure = move |_event: web_sys::KeyboardEvent| {
                let hotkeys_context = use_hotkeys_context();
                let active_scopes = hotkeys_context.active_scopes.get();
                let mut pressed_keys = hotkeys_context.pressed_keys.get();
                let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

                if !within_scope {
                    return;
                }

                if !is_last_key_match(&pressed_keys, &parsed_keys) {
                    return;
                }

                if let Some(matching_hotkey) = parsed_keys
                    .iter()
                    .find(|hotkey| is_hotkey_match(hotkey, &mut pressed_keys.keys))
                {
                    if cfg!(feature = "debug") {
                        let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                        web_sys::console::log_2(
                            &wasm_bindgen::JsValue::from_str(&message),
                            &wasm_bindgen::JsValue::from_str("color: #39FF14;"),
                        );
                    }
                    Callable::call(&on_triggered, ());
                }
            };

            // needs `leptos::ev::DOMEventResponder`
            let _ = element.add(ev::keydown, keydown_closure);
        }
    });
}

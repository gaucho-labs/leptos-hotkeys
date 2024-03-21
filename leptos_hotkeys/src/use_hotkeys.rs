use wasm_keys::{
    hotkey::Hotkey,
    wasm_key::WasmKey,
    Fluid
};

use leptos::{html::ElementDescriptor, *};

#[cfg_attr(feature = "ssr", allow(dead_code))]
fn is_hotkey_match(
    hotkey: &Hotkey,
    pressed_keyset: &mut std::collections::HashMap<WasmKey, web_sys::KeyboardEvent>,
) -> bool {
    let pressed_keys = pressed_keyset.keys().cloned().collect();

   if *hotkey == Hotkey::from_keys(pressed_keys) {
        hotkey.keys.iter().all(|(key, _count)| {
            return if let Some(event) = pressed_keyset.get_mut(key) {
                event.prevent_default();
                true
            } else {
                false
            }
        })
    } else {
       false
   }
}

pub fn use_hotkeys_scoped(
    #[cfg_attr(feature = "ssr", allow(unused_variables))] key_combination: String,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] on_triggered: Callback<()>,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] scopes: Vec<String>,
) {
    #[cfg(not(feature = "ssr"))]
    {
        use crate::hotkeys_provider::use_hotkeys_context;
        use std::collections::HashSet;

        let parsed_keys: HashSet<Hotkey> = key_combination.split(',').map(Hotkey::from_str).collect();

        let hotkeys_context = use_hotkeys_context();
        let pressed_keys = hotkeys_context.pressed_keys;

        create_effect(move |_| {
            let active_scopes = hotkeys_context.active_scopes.get();
            let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

            if within_scope {
                let mut pressed_keyset = pressed_keys.get();
                if let Some(matching_hotkey) = parsed_keys
                    .iter()
                    .find(|hotkey| is_hotkey_match(hotkey, &mut pressed_keyset))
                {
                    if cfg!(feature = "debug") {
                        /*let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                        web_sys::console::log_2(
                            &wasm_bindgen::JsValue::from_str(&message),
                            &wasm_bindgen::JsValue::from_str("color: #39FF14;"),
                        );*/
                    }
                    Callable::call(&on_triggered, ());
                }
            }
        });
    }
}

pub fn use_hotkeys_ref_scoped<T>(
    #[cfg_attr(feature = "ssr", allow(unused_variables))] node_ref: NodeRef<T>,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] key_combination: String,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] on_triggered: Callback<()>,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] scopes: Vec<String>,
) where
    T: ElementDescriptor + 'static + Clone,
{
    #[cfg(not(feature = "ssr"))]
    create_effect(move |_| {
        use crate::hotkeys_provider::use_hotkeys_context;
        use leptos::ev::DOMEventResponder;
        use std::collections::HashSet;

        let parsed_keys: HashSet<Hotkey> = key_combination.split(',').map(Hotkey::from_str).collect();
        let scopes = scopes.clone();
        if let Some(element) = node_ref.get() {
            let keydown_closure = move |_event: web_sys::KeyboardEvent| {
                let hotkeys_context = use_hotkeys_context();
                let active_scopes = hotkeys_context.active_scopes.get();
                let mut pressed_keys = hotkeys_context.pressed_keys.get();
                let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

                if within_scope {
                    if let Some(matching_hotkey) = parsed_keys
                        .iter()
                        .find(|hotkey| is_hotkey_match(hotkey, &mut pressed_keys))
                    {
                        if cfg!(feature = "debug") {
                        /*    let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                            web_sys::console::log_2(
                                &wasm_bindgen::JsValue::from_str(&message),
                                &wasm_bindgen::JsValue::from_str("color: #39FF14;"),
                            );

                         */
                        }
                        Callable::call(&on_triggered, ());
                    }
                }
            };

            // needs `leptos::ev::DOMEventResponder`
            let _ = element.add(ev::keypress, keydown_closure);
        }
    });
}

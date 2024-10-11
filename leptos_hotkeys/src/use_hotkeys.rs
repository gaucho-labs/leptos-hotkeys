#[cfg(not(feature = "ssr"))]
use std::{cell::RefCell, rc::Rc};

use leptos::{callback::Callback, html::ElementType, prelude::*};

#[cfg(not(feature = "ssr"))]
use leptos::tachys::renderer::RemoveEventHandler;

use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::Element;

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

        Effect::new(move |_| {
            let keys_pressed = hotkeys_context.keys_pressed.get();
            let active_scopes = hotkeys_context.active_scopes.get_untracked();
            let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

            if !within_scope {
                return;
            }

            if !is_last_key_match(&parsed_keys, &keys_pressed) {
                return;
            }

            if let Some(matching_hotkey) = parsed_keys
                .iter()
                .find(|hotkey| is_hotkey_match(hotkey, &keys_pressed.key_map))
            {
                if cfg!(feature = "debug") {
                    let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                    web_sys::console::log_2(
                        &wasm_bindgen::JsValue::from_str(&message),
                        &wasm_bindgen::JsValue::from_str("color: #39FF14;"),
                    );
                }
                let _ = &on_triggered.run(());
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
    T: ElementType + 'static,
    T::Output: JsCast + Clone + 'static,
{
    #[cfg(not(feature = "ssr"))]
    {
        let remove_event_handler: Rc<RefCell<Option<RemoveEventHandler<Element>>>> =
            Rc::new(RefCell::new(None));
        let remove_event_handler_clone = Rc::clone(&remove_event_handler);

        Effect::new(move |_| {
            use crate::hotkey::{is_hotkey_match, is_last_key_match};
            use crate::{use_hotkeys_context, Hotkey};
            use std::collections::HashSet;

            let hotkeys_context = use_hotkeys_context();
            let parsed_keys: HashSet<Hotkey> =
                key_combination.split(',').map(Hotkey::new).collect();
            let scopes = scopes.clone();
            if let Some(element) = node_ref.get() {
                let keyup_closure = move |_event: web_sys::KeyboardEvent| {
                    let pressed_keys = hotkeys_context.keys_pressed.get();
                    let active_scopes = hotkeys_context.active_scopes.get_untracked();
                    let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

                    if !within_scope {
                        return;
                    }

                    if !is_last_key_match(&parsed_keys, &pressed_keys) {
                        return;
                    }

                    if let Some(matching_hotkey) = parsed_keys
                        .iter()
                        .find(|hotkey| is_hotkey_match(hotkey, &pressed_keys.key_map))
                    {
                        if cfg!(feature = "debug") {
                            let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                            web_sys::console::log_2(
                                &wasm_bindgen::JsValue::from_str(&message),
                                &wasm_bindgen::JsValue::from_str("color: #39FF14;"),
                            );
                        }
                        let _ = &on_triggered.run(());
                    }
                };

                let mut maybe_handler = remove_event_handler_clone.borrow_mut();
                if maybe_handler.is_none() {
                    let handler = element
                        .unchecked_into::<web_sys::Element>()
                        .on(leptos::ev::keyup, keyup_closure);
                    *maybe_handler = Some(handler);
                }
            }
        });
    }
}

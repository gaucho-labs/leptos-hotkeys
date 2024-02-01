use crate::types::{Hotkey, KeyboardModifiers};
use leptos::{ev::DOMEventResponder, html::ElementDescriptor, *};
use leptos_dom::NodeRef;
use web_sys::KeyboardEvent;
use crate::use_hotkeys_context;

use std::collections::HashSet;
use std::sync::Arc;


fn parse_key(key_combination: &str) -> Hotkey {
    let parts = key_combination.split('+').collect::<Vec<&str>>();

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

    Hotkey {
        modifiers,
        keys,
        description: "".to_string(), // todo! when we introduce scopes, we'll add this feature in
    }
}

fn is_hotkey_match(hotkey: &Hotkey, pressed_keyset: &HashSet<String>) -> bool {
    let mut modifiers_match = true;

    if hotkey.modifiers.ctrl {
        modifiers_match &= pressed_keyset.contains("Control");
    }

    if hotkey.modifiers.shift {
        modifiers_match &= pressed_keyset.contains("Shift");
    }

    if hotkey.modifiers.meta {
        modifiers_match &= pressed_keyset.contains("Meta");
    }

    if hotkey.modifiers.alt {
        modifiers_match &= pressed_keyset.contains("Alt");
    }

    let keys_match = hotkey
        .keys
        .iter()
        .all(|key| pressed_keyset.contains(key));

    modifiers_match && keys_match
}

pub fn use_hotkeys_scoped(key_combination: String, on_triggered: Callback<()>, scopes: Vec<String>) {
    let parsed_keys: HashSet<Hotkey> = key_combination
        .split(',')
        .map(|key_combo| parse_key(key_combo))
        .collect();

    let hotkeys_context = use_hotkeys_context();
    let pressed_keys = hotkeys_context.pressed_keys.clone();

    create_effect(move |_| {
        let active_scopes = hotkeys_context.active_scopes.get();
        
        //intersection should be O(min(scopes, active_scopes))
        let within_scope = &scopes
            .iter()
            .any(|scope| active_scopes.contains(scope));

        if *within_scope {
            let pressed_keyset = pressed_keys.get();
            if parsed_keys.iter().any(|hotkey| is_hotkey_match(hotkey, &pressed_keyset)) {
                on_triggered.call(());
                logging::log!("matched!");
            }
        } else {
            logging::log!("out of scope!");
        }
    });

}

pub fn use_hotkeys_ref<T>(
    key_combination: &'static str,
    on_triggered: Callback<()>,
) -> NodeRef<T>
where T: ElementDescriptor + 'static + Clone
{
    let node_ref = create_node_ref::<T>();
    let parsed_keys: Arc<HashSet<Hotkey>> = Arc::new(
        key_combination
        .split(',')
        .map(|key_combo| parse_key(key_combo))
        .collect());


    let hotkeys_context = Arc::new(use_hotkeys_context());
    create_effect(move |_| {
        if let Some(element) = node_ref.get() {
            // arc should be a little cheaper plain clone
            let pressed_keys = hotkeys_context.clone().pressed_keys;
            let parsed_keys = parsed_keys.clone();

            let keydown_closure = move |event: KeyboardEvent| {
                let pressed_keyset = pressed_keys.get();

                if parsed_keys
                    .iter()
                    .any(|hotkey| is_hotkey_match(hotkey, &pressed_keyset))
                {
                    on_triggered.call(());
                }
            };

            let _ = element.add(leptos::ev::keydown, keydown_closure);
        }
    });

    node_ref
}

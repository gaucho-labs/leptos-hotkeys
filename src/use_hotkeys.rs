use crate::hotkeys_provider::use_hotkeys_context;
use crate::types::{Hotkey, KeyboardModifiers};
use leptos::{ev::DOMEventResponder, html::ElementDescriptor, *};
use leptos_dom::NodeRef;
use web_sys::KeyboardEvent;

use std::collections::{HashMap, HashSet};

/// Parses a key combination string and constructs a `Hotkey` struct match.
///
/// The `key_combination` parameter is a string representing a combination of keys, 
/// separated by the '+' character. Each key can be accompanied by modifiers such as 
/// "ctrl", "alt", "meta", or "shift". The function parses this string and constructs 
/// a `Hotkey` struct containing the modifiers and keys.
///
/// # Arguments
///
/// * `key_combination` - A string representing a combination of keys and modifiers.
///
/// # Returns
///
/// A `Hotkey` struct representing the parsed key combination.
///
/// # Examples
///
/// ```
/// use crate::{parse_key, KeyboardModifiers, Hotkey};
///
/// let hotkey = parse_key("Ctrl+Shift+A");
/// assert_eq!(
///     hotkey,
///     Hotkey {
///         modifiers: KeyboardModifiers {
///             ctrl: true,
///             alt: false,
///             meta: false,
///             shift: true,
///         },
///         keys: vec!["a".to_string()],
///     }
/// );
/// ```
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

/// Determines if the given hotkey matches the set of currently pressed keys.
///
/// # Arguments
///
/// * `hotkey` - A reference to the `Hotkey` struct representing the hotkey to be matched.
/// * `pressed_keyset` - A mutable reference to a `HashMap<String, KeyboardEvent>` representing the set of currently pressed keys.
///
/// # Returns
///
/// A boolean value indicating whether the given `hotkey` matches the set of currently pressed keys.
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
            event.prevent_default();
            true
        } else {
            false
        }
    });

    modifiers_match && keys_match
}

/// Handles the usage of hotkeys within specified scopes.
///
/// This function takes a `key_combination` as a comma-separated string representing the hotkey combination,
/// a callback `on_triggered` to be executed when the hotkey is triggered,
/// and a vector `scopes` containing the scopes within which the hotkeys should be active.
///
/// # Arguments
///
/// * `key_combination` - A string representing the hotkey combination.
/// * `on_triggered` - A callback to be executed when the hotkey is triggered.
/// * `scopes` - A vector of strings containing the scopes within which the hotkeys should be active.
///
/// # Example
///
/// ```
/// use_hotkeys_scoped("Ctrl+Shift+A", Callback::new( move |_| { logging::log!("Hotkey triggered!") }), vec!["Editor".to_string(), "Global".to_string()]);
/// ```
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

/// Attaches a hotkey listener to the provided element reference within the specified scopes.
///
/// # Arguments
///
/// * `key_combination` - A string representing the combination of keys to listen for, separated by commas.
/// * `on_triggered` - A callback function to be invoked when the hotkey combination is triggered.
/// * `scopes` - A vector of strings representing the scopes in which the hotkey should be active.
///
/// # Returns
///
/// A reference to the node on which the hotkey listener is attached.
///
/// # Type Parameters
///
/// * `T` - The type of element reference.
///
/// # Constraints
///
/// * `T` must implement `ElementDescriptor`, `'static`, and `Clone`.
///
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

use crate::types::Hotkey;
use leptos::html::div;
use leptos::web_sys::KeyboardEvent;
use leptos::*;
use std::collections::HashSet;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

// Defining a hotkey context structure
#[derive(Clone)]
pub struct HotkeysContext {
    pub(crate) pressed_keys: RwSignal<HashSet<String>>,

    active_scopes: RwSignal<Vec<String>>,
    enable_scope: Callback<String>,
    disable_scope: Callback<String>,
    toggle_scope: Callback<String>,

    bound_hotkeys: RwSignal<Vec<Hotkey>>,
    add_hotkey: Callback<Hotkey>,
    remove_hotkey: Callback<Hotkey>,
}

pub fn use_hotkeys_context() -> HotkeysContext {
    use_context::<HotkeysContext>().expect("expected hotkeys context")
}

#[component]
pub fn HotkeysProvider(
    #[prop(default=vec!["*".to_string()])] initially_active_scopes: Vec<String>,

    children: Children,
) -> impl IntoView {
    let pressed_keys: RwSignal<HashSet<String>> = RwSignal::new(HashSet::new());
    let active_scopes: RwSignal<Vec<String>> = RwSignal::new(initially_active_scopes);

    let enable_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if !scopes.contains(&scope) {
                scopes.push(scope);
            }
        })
    });

    let disable_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            scopes.retain(|s| *s != scope);
        })
    });

    let toggle_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if scopes.contains(&scope) {
                scopes.retain(|s| *s != scope);
            } else {
                scopes.push(scope);
            }
        })
    });

    let bound_hotkeys: RwSignal<Vec<Hotkey>> = RwSignal::new(vec![]);

    let add_hotkey = Callback::new(move |hotkey: Hotkey| {
        bound_hotkeys.update(|keys| keys.push(hotkey));
    });

    let remove_hotkey = Callback::new(move |hotkey: Hotkey| {
        bound_hotkeys.update(|keys| keys.retain(|k| *k != hotkey));
    });

    provide_context(HotkeysContext {
        pressed_keys,
        active_scopes,
        enable_scope,
        disable_scope,
        toggle_scope,
        bound_hotkeys,
        add_hotkey,
        remove_hotkey,
    });

    logging::log!("logging outside of view in hotkeysprovider");

    div()
        .on_mount(move |_| {
            logging::log!("mounted");
            let keydown_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                logging::log!("keydown: {}", event.key());
                pressed_keys.update(|keys| {
                    keys.insert(event.key());
                });
            }) as Box<dyn Fn(_)>);
            let keyup_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                logging::log!("keyup: {}", event.key());
                pressed_keys.update(|keys| {
                    keys.remove(&event.key());
                });
            }) as Box<dyn Fn(_)>);
            document()
                .add_event_listener_with_callback(
                    "keydown",
                    keydown_listener.as_ref().unchecked_ref(),
                )
                .expect("Failed to add keydown event listener");
            document()
                .add_event_listener_with_callback("keyup", keyup_listener.as_ref().unchecked_ref())
                .expect("Failed to add keyup event listener");
            on_cleanup(move || {
                document()
                    .remove_event_listener_with_callback(
                        "keydown",
                        keydown_listener.as_ref().unchecked_ref(),
                    )
                    .expect("Failed to remove keydown event listener");
                document()
                    .remove_event_listener_with_callback(
                        "keyup",
                        keyup_listener.as_ref().unchecked_ref(),
                    )
                    .expect("Failed to remove keyup event listener");
                keydown_listener.forget();
                keyup_listener.forget();
            });
        })
        .child(children())
}

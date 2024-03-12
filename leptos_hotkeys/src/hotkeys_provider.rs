use leptos::*;
use std::collections::HashSet;

#[cfg(not(feature = "ssr"))]
use std::collections::HashMap;

#[cfg(not(feature = "ssr"))]
use leptos::html::ElementDescriptor;

#[cfg(not(feature = "ssr"))]
use wasm_bindgen::closure::Closure;

#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;

#[cfg(not(feature = "ssr"))]
use web_sys::{EventTarget, KeyboardEvent};

// Defining a hotkey context structure
#[derive(Clone, Copy)]
pub struct HotkeysContext {
    #[cfg(not(feature = "ssr"))]
    pub(crate) pressed_keys: RwSignal<HashMap<String, KeyboardEvent>>,

    #[cfg(not(feature = "ssr"))]
    pub active_ref_target: RwSignal<Option<EventTarget>>,

    #[cfg(not(feature = "ssr"))]
    pub set_ref_target: Callback<Option<EventTarget>>,
    pub active_scopes: RwSignal<HashSet<String>>,
    pub enable_scope: Callback<String>,
    pub disable_scope: Callback<String>,
    pub toggle_scope: Callback<String>,
}

pub fn provide_hotkeys_context<T>(
    node_ref: NodeRef<T>,
    allow_blur_event: bool,
    initially_active_scopes: HashSet<String>,
) -> HotkeysContext
where
    T: ElementDescriptor + 'static + Clone,
{
    #[cfg(not(feature = "ssr"))]
    let active_ref_target: RwSignal<Option<EventTarget>> = RwSignal::new(None);

    #[cfg(not(feature = "ssr"))]
    let set_ref_target = Callback::new(move |target: Option<EventTarget>| {
        active_ref_target.set(target);
    });

    #[cfg(not(feature = "ssr"))]
    let pressed_keys: RwSignal<HashMap<String, KeyboardEvent>> = RwSignal::new(HashMap::new());

    let active_scopes: RwSignal<HashSet<String>> = RwSignal::new(initially_active_scopes);

    let enable_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if !scopes.contains(&scope) {
                if cfg!(feature = "debug") {
                    logging::log!("inserting scope {}", &scope);
                }
                scopes.insert(scope);
            }
        });
    });

    let disable_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if cfg!(feature = "debug") {
                logging::log!("removing scope {}", &scope);
            }
            scopes.remove(&scope);
        })
    });

    let toggle_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if scopes.contains(&scope) {
                if cfg!(feature = "debug") {
                    logging::log!("removing scope {}", &scope);
                }
                scopes.remove(&scope);
            } else {
                if cfg!(feature = "debug") {
                    logging::log!("inserting scope {}", &scope);
                }
                scopes.insert(scope);
            }
        })
    });

    #[cfg(feature = "debug")]
    if cfg!(not(feature = "ssr")) {
        create_effect(move |_| {
            let pressed_keys_list =
                move || pressed_keys.get().keys().cloned().collect::<Vec<String>>();
            logging::log!("keys pressed: {:?}", pressed_keys_list());
        });
    }

    node_ref.on_load(move |_| {
        let blur_listener = Closure::wrap(Box::new(move || {
            if cfg!(feature = "debug") {
                logging::log!("Window lost focus");
            }
            pressed_keys.set_untracked(HashMap::new());
        }) as Box<dyn Fn()>);

        let keydown_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            pressed_keys.update(|keys| {
                keys.insert(event.key().to_lowercase(), event);
            });
        }) as Box<dyn Fn(_)>);
        let keyup_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            pressed_keys.update(|keys| {
                keys.remove(&event.key().to_lowercase());
            });
        }) as Box<dyn Fn(_)>);

        if !allow_blur_event {
            window()
                .add_event_listener_with_callback("blur", blur_listener.as_ref().unchecked_ref())
                .expect("Failed to add blur event listener");
        }

        document()
            .add_event_listener_with_callback("keydown", keydown_listener.as_ref().unchecked_ref())
            .expect("Failed to add keydown event listener");
        document()
            .add_event_listener_with_callback("keyup", keyup_listener.as_ref().unchecked_ref())
            .expect("Failed to add keyup event listener");

        on_cleanup(move || {
            if !allow_blur_event {
                window()
                    .remove_event_listener_with_callback(
                        "blur",
                        blur_listener.as_ref().unchecked_ref(),
                    )
                    .expect("Failed to remove blur event listener");
                blur_listener.forget();
            }

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
    });

    let hotkeys_context = HotkeysContext {
        #[cfg(not(feature = "ssr"))]
        pressed_keys,

        #[cfg(not(feature = "ssr"))]
        active_ref_target,

        #[cfg(not(feature = "ssr"))]
        set_ref_target,

        active_scopes,
        enable_scope,
        disable_scope,
        toggle_scope,
    };

    provide_context(hotkeys_context);
    hotkeys_context
}

pub fn use_hotkeys_context() -> HotkeysContext {
    use_context::<HotkeysContext>().expect("expected hotkeys context")
}

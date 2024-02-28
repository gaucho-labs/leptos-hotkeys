use crate::scopes;
use cfg_if::cfg_if;
use leptos::*;
use std::collections::HashSet;
cfg_if! {
    if #[cfg(any(feature = "hydrate", feature= "csr"))] {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        use web_sys::{
            EventTarget, KeyboardEvent
        };
        use leptos::html::div;
        use std::collections::HashMap;
    }
}

// Defining a hotkey context structure
cfg_if! {
    if #[cfg(any(feature = "hydrate", feature= "csr"))] {
        #[derive(Clone)]
        pub struct HotkeysContext {
            pub(crate) pressed_keys: RwSignal<HashMap<String, KeyboardEvent>>,
            pub active_ref_target: RwSignal<Option<EventTarget>>,
            pub set_ref_target: Callback<Option<EventTarget>>,
            pub active_scopes: RwSignal<HashSet<String>>,
            pub enable_scope: Callback<String>,
            pub disable_scope: Callback<String>,
            pub toggle_scope: Callback<String>,
        }
    } else {
        #[derive(Clone)]
        pub struct HotkeysContext {
            pub active_scopes: RwSignal<String>,
            pub enable_scope: Callback<String>,
            pub disable_scope: Callback<String>,
            pub toggle_scope: Callback<String>,
        }
    }
}

pub fn use_hotkeys_context() -> HotkeysContext {
    use_context::<HotkeysContext>().expect("expected hotkeys context")
}

#[allow(unused_variables)]
#[component]
pub fn HotkeysProvider(
    /// when a blur event occurs, the pressed_keys reset, defaults to `false`
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event
    #[prop(default = false)]
    allow_blur_event: bool,

    #[prop(default={
        scopes!()
    })]
    initially_active_scopes: HashSet<String>,

    children: Children,
) -> impl IntoView {
    cfg_if! {
        if #[cfg(any(feature = "hydrate", feature= "csr"))] {
            let active_ref_target: RwSignal<Option<EventTarget>> = RwSignal::new(None);
            let set_ref_target = Callback::new(move |target: Option<EventTarget>| {
                active_ref_target.set(target);
            });

            let pressed_keys: RwSignal<HashMap<String, KeyboardEvent>> = RwSignal::new(HashMap::new());

            let active_scopes: RwSignal<HashSet<String>> = RwSignal::new(initially_active_scopes);

            let enable_scope = Callback::new(move |scope: String| {
                active_scopes.update(|scopes| {
                    if !scopes.contains(&scope) {
                        scopes.insert(scope);
                    }
                })
            });

            let disable_scope = Callback::new(move |scope: String| {
                active_scopes.update(|scopes| {
                    scopes.remove(&scope);
                })
            });

            let toggle_scope = Callback::new(move |scope: String| {
                active_scopes.update(|scopes| {
                    if scopes.contains(&scope) {
                        scopes.remove(&scope);
                    } else {
                        scopes.insert(scope);
                    }
                })
            });

            provide_context(HotkeysContext {
                pressed_keys,
                active_ref_target,
                set_ref_target,
                active_scopes,
                enable_scope,
                disable_scope,
                toggle_scope,
            });


    div()
        .on_mount(move |_| {
            let blur_listener = Closure::wrap(Box::new(move || {
                // todo! add tracing
                // logging::log!("Window lost focus");
                pressed_keys.set(HashMap::new());
            }) as Box<dyn Fn()>);

            let keydown_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                // todo! add tracing
                // logging::log!("key pressed: {}", event.key().to_lowercase());
                pressed_keys.update(|keys| {
                    keys.insert(event.key().to_lowercase(), event);
                });
            }) as Box<dyn Fn(_)>);
            let keyup_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                // todo! add tracing
                // logging::log!("key up: {}", event.key().to_lowercase())
                pressed_keys.update(|keys| {
                    keys.remove(&event.key().to_lowercase());
                });
            }) as Box<dyn Fn(_)>);

            if !allow_blur_event {
                window()
                    .add_event_listener_with_callback(
                        "blur",
                        blur_listener.as_ref().unchecked_ref(),
                    )
                    .expect("Failed to add blur event listener");
            }

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
        })
        .child(children())
        } else {
            view! {
                <></>
            }
        }
    }
}

use leptos::{callback::Callback, html::ElementType, prelude::*};
#[cfg(not(feature = "ssr"))]
use send_wrapper::SendWrapper;
use std::collections::{BTreeMap, HashSet};

use wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
pub struct HotkeysContext {
    #[cfg(not(feature = "ssr"))]
    pub(crate) keys_pressed: RwSignal<SendWrapper<KeyPresses>>,

    #[cfg(not(feature = "ssr"))]
    pub active_ref_target: RwSignal<Option<SendWrapper<web_sys::EventTarget>>>,

    #[cfg(not(feature = "ssr"))]
    pub set_ref_target: Callback<Option<SendWrapper<web_sys::EventTarget>>>,

    pub active_scopes: RwSignal<HashSet<String>>,
    pub enable_scope: Callback<String>,
    pub disable_scope: Callback<String>,
    pub toggle_scope: Callback<String>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct KeyPresses {
    pub key_map: BTreeMap<String, web_sys::KeyboardEvent>,
    pub last_key: Option<String>,
}

pub fn provide_hotkeys_context<T>(
    #[cfg_attr(feature = "ssr", allow(unused_variables))] node_ref: NodeRef<T>,
    #[cfg_attr(feature = "ssr", allow(unused_variables))] allow_blur_event: bool,
    initially_active_scopes: HashSet<String>,
) -> HotkeysContext
where
    T: ElementType + 'static,
    T::Output: JsCast + Clone + 'static,
{
    #[cfg(not(feature = "ssr"))]
    let active_ref_target: RwSignal<Option<SendWrapper<web_sys::EventTarget>>> =
        RwSignal::new(None);

    #[cfg(not(feature = "ssr"))]
    let set_ref_target = Callback::new(move |target: Option<SendWrapper<web_sys::EventTarget>>| {
        active_ref_target.set(target);
    });

    #[cfg(not(feature = "ssr"))]
    let keys_pressed: RwSignal<SendWrapper<KeyPresses>> =
        RwSignal::new(SendWrapper::new(KeyPresses::default()));

    let active_scopes: RwSignal<HashSet<String>> = RwSignal::new(initially_active_scopes);

    let enable_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if !scopes.contains(&scope) {
                if cfg!(feature = "debug") {
                    leptos::logging::log!("inserting scope {}", &scope);
                }
                scopes.insert(scope);
            }
        });
    });

    let disable_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if cfg!(feature = "debug") {
                leptos::logging::log!("removing scope {}", &scope);
            }
            scopes.remove(&scope);
        })
    });

    let toggle_scope = Callback::new(move |scope: String| {
        active_scopes.update(|scopes| {
            if scopes.contains(&scope) {
                if cfg!(feature = "debug") {
                    leptos::logging::log!("removing scope {}", &scope);
                }
                scopes.remove(&scope);
            } else {
                if cfg!(feature = "debug") {
                    leptos::logging::log!("inserting scope {}", &scope);
                }
                scopes.insert(scope);
            }
        })
    });

    #[cfg(all(feature = "debug", not(feature = "ssr")))]
    Effect::new(move |_| {
        let keys_pressed_list = move || {
            keys_pressed
                .get()
                .key_map
                .keys()
                .cloned()
                .collect::<Vec<String>>()
        };
        leptos::logging::log!("keys pressed: {:?}", keys_pressed_list());
    });

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        if node_ref.get().is_some() {
            let blur_listener =
                SendWrapper::new(wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    if cfg!(feature = "debug") {
                        leptos::logging::log!("Window lost focus");
                    }
                    *keys_pressed.write_untracked() = SendWrapper::new(KeyPresses::default());
                })
                    as Box<dyn Fn()>));

            let keydown_listener = SendWrapper::new(wasm_bindgen::closure::Closure::wrap(
                Box::new(move |event: web_sys::KeyboardEvent| {
                    keys_pressed.update(|keys| {
                        let key = clean_key(&event);
                        keys.key_map.insert(key.clone(), event);
                        keys.last_key = Some(key);
                    });
                }) as Box<dyn Fn(_)>,
            ));
            let keyup_listener = SendWrapper::new(wasm_bindgen::closure::Closure::wrap(Box::new(
                move |event: web_sys::KeyboardEvent| {
                    keys_pressed.update(|keys| {
                        let key = clean_key(&event);
                        keys.key_map.remove(&key);
                        keys.last_key = None;
                    });
                },
            )
                as Box<dyn Fn(_)>));

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
                    blur_listener.take().forget();
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
                keydown_listener.take().forget();
                keyup_listener.take().forget();
            });
        }
    });

    let hotkeys_context = HotkeysContext {
        #[cfg(not(feature = "ssr"))]
        keys_pressed,

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

#[cfg(not(feature = "ssr"))]
fn clean_key(event: &web_sys::KeyboardEvent) -> String {
    if cfg!(feature = "use_key") {
        match event.key().as_str() {
            " " => "spacebar".to_string(),
            key => key.to_lowercase(),
        }
    } else {
        event.code().to_lowercase()
    }
}

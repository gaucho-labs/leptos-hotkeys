use crate::{scopes, use_hotkeys};
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

            provide_context(HotkeysContext {
                pressed_keys,
                active_ref_target,
                set_ref_target,
                active_scopes,
                enable_scope,
                disable_scope,
                toggle_scope,
            });

            if cfg!(feature = "debug") {
                create_effect(move |_| {
                    let pressed_keys_list = move || pressed_keys.get().keys().cloned().collect::<Vec<String>>();
                    logging::log!("keys pressed: {:?}", pressed_keys_list());
                });
            }


            div()
                .on_mount(move |_| {
            let blur_listener = Closure::wrap(Box::new(move || {
                if cfg!(feature = "debug") {
                    logging::log!("Window lost focus");
                }
                pressed_keys.set(HashMap::new());
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
                .child(
                    view!{
                    <div>
                        {children()}

                            if cfg!(feature = "debug") {
                                    use crate::use_hotkeys::use_hotkeys_scoped;

                                    let HotkeysContext { pressed_keys, active_scopes, .. } = use_hotkeys_context();

                                    let have_pressed_keys = move || pressed_keys.get().len() > 0;
                                    let pressed_keys_list = move || pressed_keys.get().keys().cloned().collect::<Vec<String>>();

                                    let active_scopes_list = move || active_scopes.get().iter().cloned().collect::<Vec<String>>();

                                    let show_tracer = create_rw_signal(true);

                                    use_hotkeys!(("control+w") => move |_| {
                                        show_tracer.update(move |s| {
                                            *s = match s {
                                                true => false,
                                                false => true,
                                            };
                                        })
                                    });

                                    view! {
                                        <style>{include_str!("./styles.css")}</style>

                                        <div class="fixed bottom-4 transform flex justify-center w-full">
                                        <Show
                                            when=move || show_tracer.get()
                                            fallback=move || view! { <p>"Press `control+W` to toggle the hotkey dev tool"</p> }
                                        >
                                         <div
                                         class="px-4 py-2 w-3/4 lg:w-2/5 bg-purple-300"
                                         >
                                            <div class="flex space-x-2 items-center">
                                                <p>"Active scopes: " </p>
                                                <div class="flex space-x-4">
                                                    <For
                                                        each=move || active_scopes_list()
                                                        key=|s| s.clone()
                                                        children=move |s| {
                                                            view! {
                                                                <div>
                                                                    {format!("{}", s)}
                                                                </div>
                                                            }
                                                        }
                                                    />
                                                </div>
                                            </div>

                                            <div class="flex space-x-2 text-xl items-center">
                                            <p>Currently pressed: </p>
                                                <For
                                                    each=move || pressed_keys_list()
                                                    key=|key| key.clone()
                                                    children=move |key| {
                                                        view! {
                                                            <div>
                                                                {format!("{}", key)}
                                                            </div>
                                                        }
                                                    }
                                                />
                                            </div>
                                            </div>
                                        </Show>
                                    </div>
                                }
                            }
                        </div>
                    }
                )
        }
    }
}

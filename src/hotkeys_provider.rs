use crate::scopes;
use cfg_if::cfg_if;
use leptos::html::div;
use leptos::*;
use std::collections::{HashMap, HashSet};

cfg_if! {
    if #[cfg(any(feature = "hydrate", feature= "csr"))] {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        use web_sys::{
            EventTarget, KeyboardEvent
        };
    }
}

// Defining a hotkey context structure
cfg_if! {
    if #[cfg(any(feature = "hydrate", feature= "csr"))] {
/// A Structure used to store the currently pressed keys and any declared scopes.
///
/// `HotkeysContext` is accessed by calling the `use_hotkeys_context` function
/// which returns HotkeysContext via leptos `use_context`. This will fail
/// if `HotkeysProvider` has not been provided at the root view of the `App()` component, See
/// `HotkeysProvider`. This struct can then be used to call Callbacks like 'toggle_scope':
///
/// ```rust
/// // <HotkeysProvider> must be used to have access to HotkeysContext
/// let HotkeysContext { toggle_scope, .. } = use_hotkeys_context();
/// // global hotkeys
/// use_hotkeys!(("s") => move |_| {
///     toggle_scope("scope_a".to_string());
///     toggle_scope("scope_b".to_string());
///
///     if current_scope.get() == "scope_a" {
///         current_scope.set("scope_b")
///     } else {
///         current_scope.set("scope_a")
///     }
/// });
/// ```
        #[derive(Clone)]
        pub struct HotkeysContext {
            pub(crate) pressed_keys: RwSignal<HashMap<String, KeyboardEvent>>,
            /// This is where events are stored when `_ref` is changed
            pub active_ref_target: RwSignal<Option<EventTarget>>,
            /// This is the callback that triggers a change in `active_ref_target`
            pub set_ref_target: Callback<Option<EventTarget>>,
            /// This hashset allows `use_hotkeys` functions to check if a keypress scope matches
            pub active_scopes: RwSignal<HashSet<String>>,
            /// This callback inserts a scope into `active_scopes`
            pub enable_scope: Callback<String>,
            /// This callback removes a scope from `active_scopes`
            pub disable_scope: Callback<String>,
            /// This callback checks if scope exists and then adds/remove from `active_scopes`
            pub toggle_scope: Callback<String>,
        }
    } else {
        #[derive(Clone)]
        pub struct HotkeysContext {
            pub active_scopes: RwSignal<HashSet<String>>,
            pub enable_scope: Callback<String>,
            pub disable_scope: Callback<String>,
            pub toggle_scope: Callback<String>,
        }
    }
}

pub fn use_hotkeys_context() -> HotkeysContext {
    use_context::<HotkeysContext>().expect("expected hotkeys context")
}

/// This is the leptos component that provides HotkeysContext to leptos apps.
///
/// In order for `use_hotkeys` to be used and `HotkeysContext` to be accessible,
/// `HotkeysProvider` component must be added to the root of the `App()` component:
///
/// ```rust
/// #[component]
/// pub fn App() -> impl IntoView {
///     provide_meta_context();
///
///     view! {
///         <Stylesheet id="leptos" href="/pkg/demo.css"/>
///         <HotkeysProvider initially_active_scopes=scopes!("scope_a")>
///             <ThemeProvider>
///                 <Router>
///                     <Routes>
///                         <Route path="/" view=HomePage/>
///                         <Route path="/:else" view=ErrorPage/>
///                     </Routes>
///                 </Router>
///             </ThemeProvider>
///         </HotkeysProvider>
///     }
/// }
/// ```
///
/// As can be seen, we have declared initial scope to be `"scopes_a"`. `HotkeysProvider`
/// defaults to a scope of `"*"`. The `scopes!` macro infers a context of 
/// `"*"` AND `"scopes_a"`. See `scopes!` macro for more info.
#[component]
pub fn HotkeysProvider(
    /// when a blur event occurs, the pressed_keys reset, defaults to `false`
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event
    #[prop(default = false)]
    allow_blur_event: bool,

    /// This sets the initial scopes to be other than the default scope of `"*"`
    #[prop(default={
        scopes!()
    })]
    initially_active_scopes: HashSet<String>,

    /// The inner components of this components
    children: Children,
) -> impl IntoView {
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
                logging::log!("Window lost focus");
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
        .child(children())
}

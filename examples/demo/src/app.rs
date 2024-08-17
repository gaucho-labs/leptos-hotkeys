use leptos::*;
use leptos_hotkeys::{
    provide_hotkeys_context, scopes, use_hotkeys, use_hotkeys_context, use_hotkeys_ref,
    HotkeysContext,
};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Button(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <a href=href target="_blank">
            {children()}
        </a>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let main_ref = create_node_ref::<html::Main>();
    provide_hotkeys_context(main_ref, false, scopes!("scope_a"));

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css" />
        <main _ref=main_ref>
            <Router>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/:else" view=ErrorPage />
                </Routes>
            </Router>
        </main>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    const SCOPE_BORDER: &str =
        "border border-1 border-[#1a1a1a] dark:border-[#fdfdfd] p-8 space-y-20 h-full";
    let current_scope = create_rw_signal("scope_a");
    let is_green = create_rw_signal(true);

    // leptos_hotkey specific logic
    fn go_to_link(key: &'static str, link: String, scope: &'static str) {
        use_hotkeys!((*key, scope) => move |_| {
            window().location().set_href(&link).expect("Failed to navigate");
        })
    }

    let (count, set_count) = create_signal(0);

    let HotkeysContext { toggle_scope, .. } = use_hotkeys_context();

    // global hotkeys
    use_hotkeys!(("KEYs") => move |_| {
        toggle_scope("scope_a".to_string());
        toggle_scope("scope_b".to_string());

        if current_scope.get() == "scope_a" {
            current_scope.set("scope_b")
        } else {
            current_scope.set("scope_a")
        }
    });

    go_to_link(
        "controlleft+keyR,controlright+keyR",
        "https://github.com/gaucho-labs/leptos_hotkeys".to_string(),
        "*",
    );

    // scope_a related hotkeys
    use_hotkeys!(("arrowup,arrowright", "scope_a") => move |_| {
        set_count.update(|count| {
            *count += 1;
        })
    });

    use_hotkeys!(("arrowdown,arrowleft", "scope_a") => move |_| {
        set_count.update(|count| {
            *count -= 1;
        })
    });

    use_hotkeys!(("escape", "scope_a") => move |_| {
        set_count.set(0);
    });

    let a_ref = create_node_ref::<html::Div>();
    use_hotkeys_ref!((a_ref, "digit6", "scope_a") => move |_| {
        if is_green.get() {
            is_green.set(false)
        } else {
            is_green.set(true)
        }
    });

    const BANANA: &str = "https://www.youtube.com/watch?v=N982sQcjsZI";

    go_to_link("keyB+metaleft", BANANA.to_string(), "scope_b");

    view! {
        <div class="dark:bg-[#1a1a1a] bg-[#fdfdfd] dark:text-white flex justify-center h-screen py-20 w-full font-robotomono absolute">

            <div class="w-10/12 h-full flex flex-col space-y-20">
                <div class="space-y-2 text-lg">
                    <div class="flex space-x-8 items-end">
                        <Button href="https://github.com/gaucho-labs/leptos-hotkeys">
                            <p class="text-2xl">leptos_hotkeys</p>
                        </Button>
                        <p class="text-sm">press ctrl+R to go to repository</p>
                    </div>
                    <p>a library designed to declaratively pair your keybindings with callbacks.</p>
                </div>
                <div class="flex-1 flex flex-col space-y-20">
                    <div class="flex space-x-8 items-center">

                        <div>
                            <p>Press 's' to toggle between scopes a and b</p>
                            <p>Current scope: {move || { current_scope.get() }}</p>
                        </div>
                    </div>
                    <div class="flex-1 grid grid-col-1 lg:grid-cols-2">
                        <div
                            id="scope_a"
                            class:active=move || current_scope.get() == "scope_a"
                            class:not-active=move || current_scope.get() != "scope_a"
                        >
                            <div class=format!("{}", SCOPE_BORDER)>
                                <p>scope_a</p>
                                <div class="space-y-8">
                                    <p class="text-lg">Current count: {count}</p>
                                    <div class="space-y-2">
                                        <p>"press 'Arrow Up' to increase the count"</p>
                                        <p>"press 'Arrow Down' to decrease the count"</p>
                                        <p>"press 'Escape' to reset the count"</p>
                                    </div>
                                </div>
                                <div
                                    _ref=a_ref
                                    tabIndex=-1
                                    class:green=move || is_green.get()
                                    class:yellow=move || !is_green.get()
                                >
                                    <p>click on me and press 6</p>
                                </div>
                            </div>
                        </div>
                        <div
                            id="scope_b"
                            class:active=move || current_scope.get() == "scope_b"
                            class:not-active=move || current_scope.get() != "scope_b"
                        >
                            <div class=format!("{}", SCOPE_BORDER)>
                                <p>scope_b</p>
                                <div class="space-y-2">
                                    <p>press 'T' to switch themes</p>
                                    <p>press "Cmd/Super/Win"+ 'B'</p>
                                </div>

                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex space-x-8">
                    <Button href="https://github.com/gaucho-labs/leptos-hotkeys">
                        <p>Source code</p>
                    </Button>
                    <Button href="https://github.com/gaucho-labs/leptos-hotkeys?tab=readme-ov-file#quick-start">
                        <p>Quick start</p>
                    </Button>
                    <Button href="https://crates.io/crate/leptos_hotkeys">
                        <p>Crate</p>
                    </Button>
                </div>

            </div>
        </div>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <div class="h-screen w-full flex flex-col items-center justify-center font-robotomono"
            .to_string()>
            <p class="">Unknown command: {unknown}</p>
        </div>
    }
}

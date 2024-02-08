use leptos::*;
use leptos_hotkeys::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, Theme, ThemeProvider};
use std::collections::HashSet;

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

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>
        <HotkeysProvider initially_active_scopes=scopes!("scope_a")>
            <ThemeProvider>
                <Router>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/:else" view=ErrorPage/>
                    </Routes>
                </Router>
            </ThemeProvider>
        </HotkeysProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    const SCOPE_BORDER: &'static str =
        "border border-1 border-[#1a1a1a] dark:border-[#fdfdfd] p-8 space-y-20 h-full";
    let current_scope = create_rw_signal("scope_a");
    let is_green = create_rw_signal(true);
    let current_theme = use_theme();

    // leptos_hotkey specific logic
    fn go_to_link(key: &'static str, link: String, scope: &'static str) {
        use_hotkeys!((*key, scope) => move |_| {
            window().location().set_href(&link).expect("Failed to navigate");
        })
    }

    let (count, set_count) = create_signal(0);

    let HotkeysContext { toggle_scope, .. } = use_hotkeys_context();

    // global hotkeys
    use_hotkeys!(("s") => move |_| {
        toggle_scope("scope_a".to_string());
        toggle_scope("scope_b".to_string());

        if current_scope.get() == "scope_a" {
            current_scope.set("scope_b")
        } else {
            current_scope.set("scope_a")
        }
    });

    go_to_link(
        "control+R",
        "https://github.com/friendlymatthew/leptos_hotkeys".to_string(),
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

    // scope_b related hotkeys
    use_hotkeys!(("t", "scope_b") => move |_| {
        if current_theme.get() == Theme::Light {
            current_theme.set(Theme::Dark)
        } else {
            current_theme.set(Theme::Light)
        }
    });

    let a_ref = use_hotkeys_ref!(("6", "scope_a") => move |_| {
        if is_green.get() {
            is_green.set(false)
        } else {
            is_green.set(true)
        }
    });

    const GORILLAS: &'static str = "https://www.youtube.com/watch?v=qavePUOut_c";
    const DOGLICKEDTHEOLE: &'static str = "https://www.youtube.com/watch?v=4arBraMyp0Q";
    const LOW: &'static str = "https://www.youtube.com/watch?v=YIZz2PMnEDM";
    const ALASKA: &'static str = "https://www.youtube.com/watch?v=qRODjitiKP8";
    const TAINAN: &'static str = "https://www.youtube.com/watch?v=pWOFFlPmVdk";
    const NORM: &'static str = "https://www.youtube.com/watch?v=ELoXiuDA_sQ";

    go_to_link("G", format!("{}", GORILLAS), "scope_b");
    go_to_link("D", format!("{}", DOGLICKEDTHEOLE), "scope_b");
    go_to_link("L+O+W", format!("{}", LOW), "scope_b");
    go_to_link("A", format!("{}", ALASKA), "scope_b");
    go_to_link("arrowUp", format!("{}", TAINAN), "scope_b");
    go_to_link("arrowDown", format!("{}", NORM), "scope_b");

    view! {
        <main class="dark:bg-[#1a1a1a] bg-[#fdfdfd] dark:text-white flex justify-center h-screen py-20 w-full font-robotomono absolute">

            <div class="w-10/12 h-full flex flex-col space-y-20">
                <div class="space-y-2 text-lg">
                    <div class="flex space-x-8 flex items-end">
                        <Button href="https://github.com/friendlymatthew/leptos-hotkeys">
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
                                    class:green=move || is_green.get() == true
                                    class:yellow=move || is_green.get() == false
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
                                    <p>press 'G' to see gorillas avoiding the rain</p>
                                    <p>
                                        "press 'Arrow Down' to hear Norm tell a story about his friend Drake"
                                    </p>
                                    <p> "press 'D' to hear a southern man talk without an accent"</p>
                                    <p>
                                        "press 'L+O+W' to listen to Pavarotti's rendition of Flo Rida's 'Low'"
                                    </p>
                                    <p>
                                        "press 'A' to watch a man brave the Alaskan winter without a tent"
                                    </p>
                                    <p>"press 'Arrow Up' for a nice gator roll"</p>
                                </div>

                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex space-x-8">
                    <Button href="https://github.com/friendlymatthew/leptos-hotkeys">
                        <p>Source code</p>
                    </Button>
                    <Button href="https://github.com/friendlymatthew/leptos-hotkeys?tab=readme-ov-file#quick-start">
                        <p>Quick start</p>
                    </Button>
                    <Button href="https://crates.io/crate/leptos_hotkeys">
                        <p>Crate</p>
                    </Button>
                </div>

            </div>
        </main>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main class=format!(
            "h-screen w-full flex flex-col items-center justify-center font-robotomono",
        )>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}

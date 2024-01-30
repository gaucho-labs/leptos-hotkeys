use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{
    ThemeProvider,
    use_theme,
    Theme
};
use leptos_hotkeys::{
    HotkeysProvider,
    use_hotkeys_context,
    HotkeysContext,
    use_hotkeys
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>
        <HotkeysProvider initially_active_scopes={vec!["scope_a".to_string()]}>
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
    let hotkeys_context: HotkeysContext = use_hotkeys_context();

    let current_theme = use_theme();
    let (count, set_count) = create_signal(0);

    use_hotkeys(
        "t",
        Callback::new(move |_| {
            if current_theme.get() == Theme::Light {
                current_theme.set(Theme::Dark)
            } else {
                current_theme.set(Theme::Light)
            }
        })
    );

    use_hotkeys(
        "arrowup",
        Callback::new(move |_| {
            set_count.update(|count| {
                *count += 1;
            });
        })
    );

    use_hotkeys(
        "arrowdown",
        Callback::new(move |_| {
            set_count.update(|count| {
                *count -= 1;
            })
        })
    );

    use_hotkeys(
        "Escape",
        Callback::new(move |_| {
            set_count.set(0);
        })
    );

    const REPO: &'static str = "https://github.com/friendlymatthew/leptos_hotkeys#README";
    const GORILLAS: &'static str = "https://www.youtube.com/watch?v=qavePUOut_c";

    fn go_to_link(
        key: &'static str,
        link: String,
    ) {
        use_hotkeys(
            key,
            Callback::new(move |_| {
                window().location().set_href(&link).expect("Failed to navigate");
            })
        );
    }

    let toggle = hotkeys_context.toggle_scope;
    let enable = hotkeys_context.enable_scope;
    let disable = hotkeys_context.disable_scope;

    go_to_link("G+control", format!("{}", GORILLAS));
    go_to_link("R", format!("{}", REPO));

    view! {
        <main class="dark:bg-[#1a1a1a] bg-[#fdfdfd] dark:text-white h-screen py-20 w-full font-robotomono absolute">
            <div class="relative w-full flex justify-end right-4 z-10">
                <p>Press T to toggle between themes</p>
            </div>
        <div class="h-full flex flex-col items-center justify-around">
            <div class="text-center space-y-2">
                <p class="text-3xl">leptos-hotkeys</p>
                <p>a declarative way of using keyboard shortcuts in Leptos</p>
                <p>{"Press R to see how it works"}</p>
            </div>
            <div class="text-center">
                <p class="text-3xl mb-4"> {move || count.get()} </p>
                <p>{"Press up arrow to increment"}</p>
                <p>{"down arrow to decrement "}</p>
                <p>{"esc to reset"}</p>
            </div>
            <div>
                <p>{"Press control+G to see gorillas avoiding the rain"}</p>
            </div>
            <div>
                <a
                    href=REPO
                    target="_blank"
                    rel="noreferrer"
                >
                    {"Press R to "} contribute
                </a>
            </div>
            <div class="relative w-full flex justify-end right-4 z-10">
                <div class="h-full flex flex-col items-center justify-around">
                    <button on:click=move |_| toggle("scope_a".to_string())>"Toggle scope"</button>
                    <button on:click=move |_| enable("scope_a".to_string())>"Enable scope"</button>
                    <button on:click=move |_| disable("scope_a".to_string())>"Disable scope"</button>
                </div>
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

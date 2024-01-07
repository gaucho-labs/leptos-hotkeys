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
        <HotkeysProvider>
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
        "g",
        Callback::new(move |_| {
            if current_theme.get() == Theme::Light {
                current_theme.set(Theme::Dark)
            } else {
                current_theme.set(Theme::Light)
            }
        })
    );

    use_hotkeys(
        "ctrl+k",
        Callback::new(move |_| {
            set_count.update(|count| {
                *count += 1;
            });
        })
    );

    view! {
        <main class="dark:bg-[#1a1a1a] bg-white dark:text-white h-screen py-20 w-full space-y-8 font-robotomono absolute">
            <div class="relative w-full flex justify-end right-4 z-10">
                <p>Press G to toggle between themes</p>
            </div>
            <div class="text-center space-y-2">
                <p class="text-3xl">leptos-hotkeys</p>
                <p>a declarative way of using keyboard shortcuts in Leptos</p>
            </div>
            <div class="text-center">{"Press CTRL+K: "} {move || count.get()}</div>
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

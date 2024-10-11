use crate::{error_template::ErrorTemplate, errors::AppError};
use leptos::{html::Div, prelude::*};
use leptos_hotkeys::{
    provide_hotkeys_context, scopes, use_hotkeys, use_hotkeys_ref, HotkeysContext,
};
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::components::{Route, Router, Routes};
use leptos_router_macro::path;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta
                    name="description"
                    content="leptos_hotkeys SSR demo."
                />
                <link rel="stylesheet" id="leptos" href="/pkg/ssr-demo.css"/>
                <title>"Welcome to Leptos Hotkeys"</title>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let main_ref = NodeRef::<leptos::html::Main>::new();

    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    view! {
        // content for this welcome page
        <Router >
            <main node_ref=main_ref>
                <Routes fallback=|| {
                    let mut errors = Errors::default();
                    errors.insert_with_default_key(AppError::NotFound);
                    view! {
                        <ErrorTemplate errors/>
                    }
                    .into_view()
                }>
                    <Route path=path!("") view=HomePage />

                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);

    use_hotkeys!(("arrowup") => move |_| {
        set_count.update(|c| *c += 1);
    });

    use_hotkeys!(("arrowdown") => move |_| {
        set_count.update(|c| *c -= 1);
    });

    use_hotkeys!(("space") => move |_| {
        leptos::logging::log!("hola")
    });

    let div_ref = NodeRef::<Div>::new();

    use_hotkeys_ref!((div_ref, "5") => move |_| {
        leptos::logging::log!("howdy")
    });

    use_hotkeys!(("controlleft") => move |_| {
        leptos::logging::log!("works either using control left or control right!")
    });

    let giraffe_signal = RwSignal::new(false);

    use_hotkeys!(("space + l") => move |_| {
        giraffe_signal.set(!giraffe_signal.get_untracked());
        leptos::logging::log!("i'm a giraffe");
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div>"Press arrow up and arrow down: " {count}</div>
        <div tabindex=-1 node_ref=div_ref>
            howdy
        </div>
        <Show when=move || giraffe_signal.get()>"I'm a giraffe!"</Show>
    }
}

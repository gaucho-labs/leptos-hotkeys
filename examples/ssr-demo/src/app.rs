use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_hotkeys::{
    provide_hotkeys_context, use_hotkeys, use_hotkeys_ref, HotkeysContext,
};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let main_ref = create_node_ref::<leptos::html::Main>();

    let HotkeysContext { .. } = provide_hotkeys_context(main_ref);

    view! {
        <Stylesheet id="leptos" href="/pkg/ssr-demo.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main _ref=main_ref>
                <Routes>
                    <Route path="" view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);

    use_hotkeys!(("arrowup") => move |_| {
        set_count.update(|c| *c += 1);
    });

    use_hotkeys!(("arrowdown") => move |_| {
        set_count.update(|c| *c -= 1);
    });

    use_hotkeys!(("space") => move |_| {
        logging::log!("hola")
    });

    let div_ref = create_node_ref::<html::Div>();

    use_hotkeys_ref!((div_ref, "5") => move |_| {
        logging::log!("howdy")
    });

    use_hotkeys!(("controlleft") => move |_| {
        logging::log!("works either using control left or control right!")
    });

    let giraffe_signal = create_rw_signal(false);

    use_hotkeys!(("space + l") => move |_| {
        giraffe_signal.set(!giraffe_signal.get());
        logging::log!("i'm a giraffe");
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div>"Press arrow up and arrow down: " {count}</div>
        <div tabIndex=-1 _ref=div_ref>
            howdy
        </div>
        <Show when=move || giraffe_signal.get()>"I'm a giraffe!"</Show>
    }
}

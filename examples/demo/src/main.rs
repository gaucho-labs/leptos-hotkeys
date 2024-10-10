use app::*;
use leptos::prelude::*;
mod app;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App /> }
    });
}

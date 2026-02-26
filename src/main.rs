use dioxus::prelude::*;

mod backend;
mod components;

use crate::components::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        let router = dioxus::server::router(App);
        Ok(router)
    });

    // 2. This handles the Client-side launch
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView {}, // The {} here matches the component call
    #[route("/favorites")]
    Favorites {},
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

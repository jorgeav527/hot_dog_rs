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
        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.20.1/cdn/themes/light.css" }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.20.1/cdn/shoelace-autoloader.js",
            r#type: "module",
        }
        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/swiper@12/swiper-bundle.min.css" }
        document::Script { src: "https://cdn.jsdelivr.net/npm/swiper@12/swiper-bundle.min.js" }
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

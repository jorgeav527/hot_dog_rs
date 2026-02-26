use dioxus::prelude::*;

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We couldn't find: {segments.join(\"/\")}" }
    }
}

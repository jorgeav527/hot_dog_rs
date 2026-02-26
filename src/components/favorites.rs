use dioxus::prelude::*;

use crate::backend;

#[component]
pub fn Favorites() -> Element {
    // Point to the backend module defined in main.rs
    let favorites = use_server_future(backend::list_dogs)?;

    // We clarify the type for the compiler here
    let dogs = match favorites().unwrap() {
        Ok(list) => list,
        Err(e) => return rsx! { "Failed to load favorites: {e}" },
    };

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in dogs {
                    div { key: "{id}", class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}

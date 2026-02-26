use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend;
use crate::components::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src =
        use_signal(|| String::from("https://images.dog.ceo/breeds/leonberg/n02111129_974.jpg"));

    let author = "The Dioxus Team";
    let article = Article {
        content:
            "This is a simple app demonstrating how to fetch dog images and save them to a backend!"
                .to_string(),
    };

    rsx! {
        div { class: "page-container",
            BlogPost { author, article }
            hr {} // Visual separator
            div { id: "dogview",
                img { src: "{img_src}", id: "dogimg" }
            }
            div { id: "buttons",
                StyledButton {
                    content: "skip",
                    extra_class: "skip-id",
                    style: ButtonStyle::Info,
                    // Dioxus automatically converts this closure into an EventHandler
                    onclick: move |_| async move {
                        let url = "https://dog.ceo/api/breeds/image/random";
                        if let Ok(response) = reqwest::get(url).await {
                            if let Ok(data) = response.json::<DogApi>().await {
                                img_src.set(data.message);
                            }
                        }
                    },
                }

                StyledButton {
                    content: "save!",
                    extra_class: "save-id",
                    style: ButtonStyle::Success,
                    onclick: move |_| async move {
                        _ = backend::save_dog(img_src()).await;
                    },
                }
            }
        }
    }
}

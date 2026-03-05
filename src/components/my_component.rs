use chrono::{Datelike, Local, Timelike};
use dioxus::prelude::*;
use std::time::Duration;
use tracing::info;

// 1. Define the data structure for the article
#[derive(PartialEq, Clone, Debug)]
pub struct Article {
    pub content: String,
}

// 2. Define the Props the component will accept
#[derive(Props, PartialEq, Clone, Debug)]
pub struct BlogProps {
    author: String,
    article: Article,
}

fn current_time() -> String {
    // Local::now() gets the time from the browser/OS
    // .format() uses strftime tokens (%H = hour, %M = min, %S = sec)
    Local::now().format("%H:%M:%S").to_string()
}

fn current_timezone() -> String {
    // This returns the offset from UTC (e.g., +05:00)
    Local::now().offset().to_string()
}

async fn do_async_work() {
    // This simulates a network request or a heavy calculation
    async_std::task::sleep(std::time::Duration::from_secs(2)).await;
}

// 3. The Component function
#[component]
pub fn BlogPost(props: BlogProps) -> Element {
    let content = "Dioxus!";
    let author = props.author;
    let article = props.article;
    let mut type_your_name = use_signal(|| "".to_string());
    let mut is_private = use_signal(|| false);
    let mut items = use_signal(|| 100);
    let mut box_width = use_signal(|| 0.0);
    let mut is_box_visible = use_signal(|| false);
    let mut copied = use_signal(|| false);
    let user_name = Some("Alice");
    let mut items_list = use_signal(|| vec!["Hello", "Dioxus"]);
    let mut loading = use_signal(|| false);
    let mut text = use_signal(|| "Complete!");

    rsx! {
        button {
            onclick: move |_| async move {
                // 1. Immediately update UI to "Loading"
                text.set("Loading...");
                loading.set(true);

                do_async_work().await;
                // 3. Update UI to "Complete"
                text.set("Complete!");
                loading.set(false);
            },
            "Status: {text} (Loading: {loading})"
        }
        ShoppingCart {}
        SlButton { variant: "success", size: "small", pill: true,
            "Success Button" // This is the 'children'
        }
        h1 {
            "Welcome to "
            {content}
        }
        MySwiper {}
        div {
            {
                format!(
                    "The time is: {now}, your timezone is {zone}",
                    now = current_time(),
                    zone = current_timezone(),
                )
                    .to_ascii_uppercase()
            }
        }
        ul {
            for item in items_list.iter() {
                li { key: "{item}", "{item}" }
            }
        }
        h3 {
            "Brought to you by {author}"
            {user_name.map(|name| rsx! { "Logged in as: {name} " })}
        }
        label {
            input { r#type: "checkbox", onchange: move |_| is_private.toggle() }
            " Enable Privacy Mode"
        }
        hr {}
        input {
            r#type: if is_private() { "password" } else { "text" },
            value: "{type_your_name}",
            oninput: move |evt| {
                type_your_name.set(evt.value());
                info!("Input changed to: {}", evt.value());
            },
        }
        p { class: "main-content", "{article.content}" }
        p { style: if is_private() { "filter: blur(5px); color: gray;" },
            "Mirroring here too: {type_your_name}"
        }
        div { style: "width: 20px; height: 20px; background-color: red; margin: 10px;" }
        span {
            class: if is_private() { "bg-red-500" },
            class: if is_private() { "border-4 border-blue-500" },
            class: "w-10 h-10 block border border-black",
            "ID"
        }
        button { onclick: move |_| items += 1, "Add one" }
        div {
            style: "border: 2px solid black; padding: 20px; transition: background 0.5s;",
            // Change background color based on the width signal
            background_color: if box_width() > 500.0 { "lightgreen" } else { "lightblue" },

            onresize: move |data| {
                if let Ok(size) = data.get_border_box_size() {
                    box_width.set(size.width);
                    info!("Container width is now: {}px", size.width);
                }
            },
            h2 { "Resize your browser window!" }
            p { "Current Width: {box_width}px" }
            p { "Goal: Get over 500px to turn me green." }
        }
        button {
            onclick: move |_| {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let title = document.title();

                // 2. Access the clipboard
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();

                // 3. Write to clipboard
                let _ = clipboard.write_text(&title);

                copied.set(true);
            },

            if copied() {
                "✅ Copied Title!"
            } else {
                "Copy Title"
            }
        }
        div {
            style: "height: 200px; margin: 50px; display: flex; align_items: center; justify_content: center;",
            class: if is_box_visible() { "opacity-100 scale-100" } else { "opacity-0 scale-50" },

            onvisible: move |_| {
                spawn(async move {
                    is_box_visible.set(true);
                    info!("The secret box appeared after a delay!");
                });
            },

            div { style: "padding: 40px; background: coral; color: white; border-radius: 10px;",
                "🎉 You found me! I only animate once I'm visible."
            }
        }
    }
}

#[component]
fn SlButton(variant: String, size: String, pill: bool, children: Element) -> Element {
    rsx! {
        sl-button { "variant": "{variant}", "size": "{size}", "pill": "{pill}", {children} }
    }
}

#[component]
fn MySwiper() -> Element {
    use_effect(move || {
        document::eval(
            r#"
            new Swiper(".mySwiper", {
                slidesPerView: 3,
                spaceBetween: 30,
                freeMode: true,
                pagination: {
                    el: ".swiper-pagination",
                    clickable: true,
                },
            });
        "#,
        );
    });

    rsx! {
        div { class: "swiper mySwiper",
            div { class: "swiper-wrapper",
                for i in 1..=9 {
                    div { class: "swiper-slide", "Slide {i}" }
                }
            }
            div { class: "swiper-pagination" }
        }
    }
}

#[component]
fn ShoppingCart() -> Element {
    // 1. Setup our Signals
    let mut count = use_signal(|| 1);
    let mut coupon_code = use_signal(|| String::new());
    let total_price = use_memo(move || {
        println!("--- Memo Scope: Recalculating Price ---");
        count() * 10
    });

    println!("--- The Component Scope is Running! ---");

    rsx! {
        div {
            h1 { "Items in cart: {count}" } // <-- READ occurring here!

            // This button triggers a re-render because 'count' is read above.
            button { onclick: move |_| count += 1, "Add Item" }

            // This input changes 'coupon_code', but 'coupon_code' is NOT read in RSX.
            input {
                placeholder: "Enter Coupon...",
                oninput: move |evt| coupon_code.set(evt.value()),
            }
            h2 { "Total: ${total_price}" }
            p { "Check your console to see when I re-render." }
        }
    }
}

use anyhow::Context;
use dioxus::prelude::*;

#[component]
pub fn MainError() -> Element {
    rsx! {
        div { class: "p-4",
            h1 { "Error Handling Lab" }

            // This boundary will catch the error from ThrowsError
            ErrorBoundary {
                handle_error: |error: ErrorContext| rsx! {
                    div { class: "p-4 bg-red-100 border border-red-500 text-red-700",
                        h2 { "Component Crashed!" }
                        // This will show: "Failed to parse name: invalid digit found in string"
                        p { "{error.error().unwrap()}" }
                    }
                },

                // This component will fail and bubble up to the boundary above
                ThrowsError {}
            }
        }
    }
}

#[component]
fn ThrowsError() -> Element {
    let mut name = use_signal(|| "".to_string());

    let parsed_name = use_memo(move || {
        if name().trim().is_empty() {
            Err("Name cannot be empty".to_string())
        } else if name().len() < 3 {
            Err("Name is too short (min 3 chars)".to_string())
        } else {
            Ok(name().clone())
        }
    });

    let mut age = use_signal(|| "".to_string());

    let parsed_age = use_memo(move || {
        age()
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse age: {e}"))
    });

    // This part will NEVER be reached because of the '?' above
    rsx! {
        input {
            value: "{name}",
            placeholder: "Enter name",
            oninput: move |e| name.set(e.value()),
        }

        match parsed_name() {
            Ok(valid_name) => rsx! {
                div { "Valid name: {valid_name}" }
            },
            Err(err) => rsx! {
                div { style: "color: orange;", "Name error: {err}" }
            },
        }

        input {
            value: "{age}",
            placeholder: "Enter age",
            oninput: move |e| age.set(e.value()),
        }

        match parsed_age() {
            Ok(age) => rsx! {
                div { "Valid age: {age}" }
            },

            Err(err) => rsx! {
                div { style: "color: orange;", "Local validation error: {err}" }
            },
        }
    }
}

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonStyle {
    Info,
    Normal,
    Danger,
    Success,
}

impl ButtonStyle {
    // Helper method to convert the enum variant into a CSS class string
    fn to_class(&self) -> &'static str {
        match self {
            ButtonStyle::Info => "btn-info",
            ButtonStyle::Normal => "btn-normal",
            ButtonStyle::Danger => "btn-danger",
            ButtonStyle::Success => "btn-success",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StyledButtonProps {
    content: String,
    style: ButtonStyle,
    #[props(default = String::new())]
    extra_class: String,
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn StyledButton(props: StyledButtonProps) -> Element {
    // We access everything via 'props.xxx'
    if props.content.is_empty() {
        return rsx! {
            b { color: "red", "Error: No Text!" }
        };
    }

    rsx! {
        button {
            class: "{props.style.to_class()} {props.extra_class}",
            onclick: move |evt| props.onclick.call(evt),
            "{props.content}"
        }
    }
}

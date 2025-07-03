use yew::prelude::*;

/// Properties for the `Button` component.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The type of the button. Default is "button" if not specified.
    pub btn_type: Option<String>,
    /// The CSS class(es) to apply to the button for styling.
    pub class: String,
    /// The text message to be displayed on the button.
    pub message: String,
    /// An optional callback function to be invoked when the button is clicked.
    pub onclick: Option<Callback<MouseEvent>>,
}

/// A functional component for rendering a button.
///
/// This component creates a styled button with customizable text, type, and click behavior.
/// The `btn_type` determines the button type attribute (e.g., "button", "submit", "reset").
/// The `class` property allows for custom CSS styling.
/// The `message` property sets the button text.
/// The `onclick` property allows for a custom click handler to be provided.
#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    // Clone the onclick callback if provided
    let onclick = props.onclick.clone();
    // Set button type to "button" if not provided
    let btn_type = props
        .btn_type
        .clone()
        .unwrap_or_else(|| "button".to_string());

    html! {
        <button
            type={btn_type}
            class={format!("button {}", props.class)}
            onclick={onclick}
        >
            { &props.message }
        </button>
    }
}

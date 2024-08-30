use yew::prelude::*;

/// Properties for the `Alert` component.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The type of alert (e.g., "success", "danger").
    pub alert_type: AttrValue,
    /// The message to be displayed in the alert.
    pub message: AttrValue,
}

/// A functional component for displaying alert messages.
///
/// This component renders a styled alert box with a message. The appearance of the alert box is
/// controlled by the `alert_type` property, which determines the alert's style (e.g., success or danger).
/// The `message` property contains the text to be displayed inside the alert box.
#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    html! {
        <div class={format!("alert alert-{}", props.alert_type)} role="alert">
            {props.message.clone()}
        </div>
    }
}

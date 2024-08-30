use yew::prelude::*;

/// Properties for the `Select` component.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The label for the `select` element.
    pub label: AttrValue,
    /// Options for the dropdown. Each option is a tuple containing the value and the label.
    pub options: Vec<(AttrValue, AttrValue)>,
    /// The name attribute for the `select` element.
    pub name: AttrValue,
    /// The currently selected value of the `select` element.
    pub value: AttrValue,
    /// Callback to handle change events on the `select` element.
    pub onchange: Callback<Event>,
}

/// A functional component that renders a `select` dropdown menu.
///
/// This component generates a `select` element with a list of options. It uses the provided `label`
/// for the dropdown's label, and the `options` to populate the dropdown items. The currently selected
/// option is controlled via the `value` property, and changes are propagated via the `onchange` callback.
///
/// # Properties
///
/// - `label`: The label to display for the dropdown.
/// - `options`: A list of tuples representing the options for the dropdown. Each tuple consists of a value
///   and a label for the option.
/// - `name`: The name attribute for the dropdown, used for form submission.
/// - `value`: The current selected value of the dropdown.
/// - `onchange`: A callback that gets invoked when the selected value changes.
#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    // Generate a unique ID for the `select` element, based on the `name` prop
    let html_id = format!("edit-{}", props.name);
    // Clone the current value for use in the select options
    let value = props.value.clone();
    
    html! {
        <>
            // Render the label associated with the dropdown
            <label for={html_id.clone()}>{props.label.clone()}</label>
            <select
                id={html_id} // Set the ID for the select element
                class="form-control" // Apply styling class
                name={props.name.clone()} // Set the name attribute
                onchange={props.onchange.clone()} // Attach the onchange callback
            >
                // Default option when no option is selected
                <option value="" selected={value.is_empty()}>{"Select an option"}</option>
                {
                    // Render each option in the options list
                    props.options.clone().into_iter().map(|(val, label)| {
                        // Determine if this option should be selected
                        let selected = *val == value;
                        html! {
                            <option value={val.clone()} selected={selected}>{label.clone()}</option>
                        }
                    }).collect::<Html>() // Collect the rendered options into Html
                }
            </select>
        </>
    }
}

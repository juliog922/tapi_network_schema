use yew::prelude::*;
use yew_router::prelude::*;
use serde_json::Value;

use crate::{
    Route,
    contexts::node_context::NodesContext,
};

/// Properties for the `ConnectivityService` component.
#[derive(Properties, PartialEq)]
pub struct ConnectivityServiceProps {
    /// JSON value containing the list of connectivity services.
    pub services: Value,
    /// The IP address of the device to be used in route navigation.
    pub device_ip: String,
}

/// A functional component that displays a list of connectivity services.
///
/// This component renders a list of services, each with a name and UUID. It also manages
/// hover and click interactions to provide more details and update the context with the
/// nodes related to each service.
///
/// # Properties
///
/// - `services`: A JSON object containing an array of connectivity services. Each service
///   includes a `value_name`, `uuid`, and `nodes` associated with it.
/// - `device_ip`: The IP address of the device, used for route navigation.
///
/// # Behavior
///
/// - When a service item is hovered over, its UUID is displayed and the item expands to
///   show more details.
/// - When a service item is clicked, the nodes associated with that service are set in
///   the context and the route is updated to show the node schema for the selected service.
#[function_component(ConnectivityService)]
pub fn connectivity_service_list(props: &ConnectivityServiceProps) -> Html {
    // Default empty array to handle cases where the "connectivity_services" field is missing
    let empty_array = vec![];
    // Extract the "connectivity_services" array from the services JSON
    let services = props.services["connectivity_services"]
        .as_array()
        .unwrap_or(&empty_array);

    // State to manage the expanded service item UUID
    let expanded_uuid = use_state(|| None);
    // Access the NodesContext from the application context
    let nodes_context = use_context::<NodesContext>()
        .expect("NodesContext not found");

    html! {
        <div class="services-section">
            <h1 class="services-title">{"Connectivity Services"}</h1>
            <div class="services-container">
                {
                    // Iterate over the services to create the list items
                    services.iter().enumerate().map(|(index, service)| {
                        let value_name = service["value_name"].as_str().unwrap_or("");
                        let uuid = service["uuid"].as_str().unwrap_or("");
                        let nodes = service["nodes"].clone();
                        // Determine if the current service is expanded
                        let is_expanded = *expanded_uuid == Some(uuid.to_string());

                        // Callback for handling mouse enter event
                        let on_mouse_enter = {
                            let expanded_uuid = expanded_uuid.clone();
                            let uuid = uuid.to_string(); // Create a copy of uuid
                            Callback::from(move |_| {
                                expanded_uuid.set(Some(uuid.clone())); // Use uuid.clone() to avoid moving
                            })
                        };

                        // Callback for handling mouse leave event
                        let on_mouse_leave = {
                            let expanded_uuid = expanded_uuid.clone();
                            Callback::from(move |_| {
                                expanded_uuid.set(None);
                            })
                        };

                        // Callback for handling click event
                        let on_click = {
                            let nodes = nodes.clone();
                            let nodes_context = nodes_context.clone();
                            Callback::from(move |_: MouseEvent| {
                                nodes_context.set(Some(nodes.clone())); // Set the nodes context with the new nodes
                            })
                        };

                        // Calculate animation delay based on the index
                        let animation_delay = format!("{}s", index as f32 * 0.3);

                        html! {
                            // Link to the route for viewing the node schema
                            <Link<Route> to={Route::NodeSchema { ip: props.device_ip.clone(), uuid: uuid.to_string() }}>
                                <div class={classes!(
                                    "service-item",
                                    if is_expanded { "expanded" } else { "" }
                                )}
                                    style={format!("animation-delay: {}", animation_delay)}
                                    onmouseenter={on_mouse_enter}
                                    onmouseleave={on_mouse_leave}
                                    onclick={on_click}
                                >
                                    <div class="service-content">
                                        <p class="service-name">{ value_name }</p>
                                        { if is_expanded {
                                            html! { <p class="service-uuid">{ format!("UUID: {}", uuid) }</p> }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                </div>
                            </Link<Route>>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

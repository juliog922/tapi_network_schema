use yew::prelude::*;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Properties, PartialEq)]
pub struct NodesProps {
    /// The JSON data representing nodes and their endpoints.
    pub nodes: Value,
}

/// A Yew functional component that displays nodes and their endpoints.
///
/// This component renders a list of nodes and their associated endpoints. It handles hover, click,
/// and mouseout events to highlight and display details of endpoints. The highlighting is managed
/// based on various attributes of the endpoints such as `link_uuid`, `client_node_edge_point_uuid`,
/// and `connection_end_point_uuid`.
///
/// # Properties
/// 
/// - `nodes`: A `serde_json::Value` representing the nodes to display.
#[function_component(Nodes)]
pub fn nodes(props: &NodesProps) -> Html {
    // State hooks for managing hover data, selected endpoint, and highlighted endpoints
    let hover_data = use_state(|| "Hover over an endpoint to see details".to_string());
    let selected_value = use_state(|| Option::<Value>::None);
    let highlighted_link_uuids = use_state(|| HashSet::<String>::new());
    let highlighted_lower_connections = use_state(|| HashSet::<String>::new());
    let highlighted_connections_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_edge_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_client_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_service_uuid = use_state(|| HashSet::<String>::new());

    // Callback to handle mouseover events and update hover data
    let onmouseover = {
        let hover_data = hover_data.clone();
        Callback::from(move |data: String| {
            hover_data.set(data);
        })
    };

    // Callback to handle mouseout events. Maintains the hover data until another element is hovered.
    let onmouseout = {
        let _hover_data = hover_data.clone();
        Callback::from(move |_| {
            // Maintains the last hover data until another element is hovered over
        })
    };

    // Callback to handle click events on endpoints and manage highlighting
    let on_click = {
        let selected_value = selected_value.clone();
        let highlighted_link_uuids = highlighted_link_uuids.clone();
        let highlighted_lower_connections = highlighted_lower_connections.clone();
        let highlighted_connections_uuid = highlighted_connections_uuid.clone();
        let highlighted_edge_uuid = highlighted_edge_uuid.clone();
        let highlighted_client_uuid = highlighted_client_uuid.clone();
        let highlighted_service_uuid = highlighted_service_uuid.clone();
        let nodes = props.nodes.clone();
        
        Callback::from(move |ep: Value| {
            selected_value.set(Some(ep.clone()));
            let mut new_highlighted_link_uuids: HashSet<String> = HashSet::new();
            let mut new_highlighted_lower_connections: HashSet<String> = HashSet::new();
            let mut new_highlighted_connections_uuid: HashSet<String> = HashSet::new();
            let mut new_highlighted_edge_uuid: HashSet<String> = HashSet::new();
            let mut new_highlighted_client_uuid: HashSet<String> = HashSet::new();
                let mut new_highlighted_service_uuid: HashSet<String> = HashSet::new();

            if let Some(_) = ep["service_interface_point_uuid"].as_str() {
                new_highlighted_service_uuid.insert("service_interface_point_uuid".to_string());
            }

            // Highlight nodes with the same link_uuid as the selected endpoint
            if let Some(link_uuid) = ep["link_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(end_points) = node["end_points"].as_array() {
                            for endpoint in end_points {
                                if endpoint["link_uuid"].as_str() == Some(link_uuid) && endpoint != &ep {
                                    new_highlighted_link_uuids.insert(link_uuid.to_string());
                                }
                            }
                        }
                    }
                }
            }

            // Highlight endpoints connected to the selected endpoint via lower_connection_ keys
            let mut lower_connections = vec![];
            if let Some(ep_obj) = ep.as_object() {
                for (key, value) in ep_obj {
                    if key.starts_with("lower_connection_") {
                        if let Some(connection_uuid) = value.as_str() {
                            lower_connections.push(connection_uuid.to_string());
                        }
                    }
                }
            }

            if let Some(nodes_array) = nodes.as_array() {
                for node in nodes_array {
                    if let Some(end_points) = node["end_points"].as_array() {
                        for endpoint in end_points {
                            if let Some(connection_end_point_uuid) = endpoint["connection_end_point_uuid"].as_str() {
                                if lower_connections.contains(&connection_end_point_uuid.to_string()) {
                                    if !highlighted_link_uuids.contains(connection_end_point_uuid) {
                                        new_highlighted_lower_connections.insert(connection_end_point_uuid.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Highlight endpoints that share the same connection_end_point_uuid
            if let Some(connection_uuid) = ep["connection_end_point_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(end_points) = node["end_points"].as_array() {
                            for endpoint in end_points {
                                if let Some(endpoint_obj) = endpoint.as_object() {
                                    for (key, value) in endpoint_obj {
                                        if key.starts_with("lower_connection_") {
                                            if let Some(lower_uuid) = value.as_str() {
                                                if lower_uuid == connection_uuid {
                                                    new_highlighted_connections_uuid.insert(connection_uuid.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Highlight endpoints with the same client_node_edge_point_uuid as the selected endpoint
            if let Some(client_uuid) = ep["client_node_edge_point_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(end_points) = node["end_points"].as_array() {
                            for endpoint in end_points {
                                if endpoint["node_edge_point_uuid"].as_str() == Some(client_uuid) && endpoint != &ep {
                                    new_highlighted_client_uuid.insert(client_uuid.to_string());
                                }
                            }
                        }
                    }
                }
            }

            // Highlight endpoints with the same node_edge_point_uuid as the selected endpoint
            if let Some(edge_uuid) = ep["node_edge_point_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(end_points) = node["end_points"].as_array() {
                            for endpoint in end_points {
                                if endpoint["client_node_edge_point_uuid"].as_str() == Some(edge_uuid) && endpoint != &ep {
                                    new_highlighted_edge_uuid.insert(edge_uuid.to_string());
                                }
                            }
                        }
                    }
                }
            }

            // Update states with the new highlighted values
            highlighted_link_uuids.set(new_highlighted_link_uuids);
            highlighted_lower_connections.set(new_highlighted_lower_connections);
            highlighted_connections_uuid.set(new_highlighted_connections_uuid);
            highlighted_client_uuid.set(new_highlighted_client_uuid);
            highlighted_edge_uuid.set(new_highlighted_edge_uuid);
            highlighted_service_uuid.set(new_highlighted_service_uuid);
        })
    };

    // Render nodes and their endpoints
    let empty_array: Vec<Value> = vec![];
    let nodes = props.nodes.as_array().unwrap_or(&empty_array);

    // Function to format JSON values as pretty-printed strings
    fn format_json(value: &Value) -> String {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "Error formatting JSON".to_string())
    }

    html! {
        <div class="component-wrapper">
            // Display hover information
            <div class="info-text-wrapper">
                <div class={classes!(
                    "info-text",
                    if hover_data.len() > 100 { "shrink" } else { "" },
                    if *hover_data != "Hover over an endpoint to see details".to_string() { "show-info" } else { "" })}>
                    { format_json(&serde_json::from_str(&hover_data).unwrap_or(Value::Null)) }
                </div>
            </div>
            <div class="new-container-good">
                {
                    for nodes.iter().map(|node| {
                        let empty_array: Vec<Value> = vec![];
                        let end_points = node["end_points"].as_array().unwrap_or(&empty_array);
                        let node_uuid = node["node_uuid"].as_str().unwrap_or("Unknown UUID");

                        html! {
                            <div class="node-item-good">
                                <h2>{ format!("Node UUID: {}", node_uuid) }</h2>
                                <div class="endpoints-container-good">
                                    {
                                        for end_points.iter().enumerate().map(|(_, ep)| {
                                            let ep_json = ep.clone(); // Clone endpoint for comparison
                                            let ep_json_str = ep.to_string(); // Use string representation for hover callback
                                            let onmouseover = onmouseover.reform(move |_| ep_json_str.clone());
                                            let onmouseout = onmouseout.clone();
                                            let on_click = {
                                                let ep_clone = ep_json.clone();
                                                on_click.reform(move |_| ep_clone.clone())
                                            };

                                            // Determine if this endpoint should be highlighted
                                            let is_selected = {
                                                if let Some(ref selected) = *selected_value {
                                                    *selected == ep.clone()
                                                } else {
                                                    false
                                                }
                                            };

                                            let mut service_flag = false;

                                            let is_service = {
                                                if let Some(_) = ep["service_interface_point_uuid"].as_str() {
                                                    service_flag = true;
                                                    highlighted_service_uuid.contains("service_interface_point_uuid") && !is_selected
                                                } else {
                                                    false
                                                }
                                            };

                                            let is_client_highlighted = {
                                                if let Some(edge_uuid) = ep["node_edge_point_uuid"].as_str() {
                                                    highlighted_client_uuid.contains(edge_uuid) && !is_selected
                                                } else {
                                                    false
                                                }
                                            };

                                            let is_edge_highlighted = {
                                                if let Some(client_uuid) = ep["client_node_edge_point_uuid"].as_str() {
                                                    highlighted_edge_uuid.contains(client_uuid) && !is_selected
                                                } else {
                                                    false
                                                }
                                            };

                                            let is_lower_highlighted = {
                                                if let Some(connection_end_point_uuid) = ep["connection_end_point_uuid"].as_str() {
                                                    highlighted_lower_connections.contains(connection_end_point_uuid)
                                                } else {
                                                    false
                                                }
                                            };

                                            let is_connection_highlighted = {
                                                if let Some(ep_obj) = ep.as_object() {
                                                    ep_obj.keys().any(|key| key.starts_with("lower_connection_")) &&
                                                    ep_obj.values().any(|value| {
                                                        if let Some(lower_uuid) = value.as_str() {
                                                            highlighted_connections_uuid.contains(lower_uuid)
                                                        } else {
                                                            false
                                                        }
                                                    }) &&
                                                    !is_selected
                                                } else {
                                                    false
                                                }
                                            };

                                            let is_highlighted = {
                                                if let Some(link_uuid) = ep["link_uuid"].as_str() {
                                                    highlighted_link_uuids.contains(link_uuid) && !is_selected
                                                } else {
                                                    false
                                                }
                                            };

                                            let last_nepu: &str = {
                                                    if let Some(node_edge_point_uuid) = ep["node_edge_point_uuid"].as_str() {
                                                        &node_edge_point_uuid[node_edge_point_uuid.len()-4..]
                                                } else {
                                                    ""
                                                }
                                            };

                                            let qualifier: &str = {
                                                if let Some(protocol_qualifier) = ep["layer_protocol_qualifier"].as_str() {
                                                    if let Some(qualifier_index) = protocol_qualifier.find("QUALIFIER_") {
                                                        &protocol_qualifier[qualifier_index + "QUALIFIER_".len()..]
                                                    } else {
                                                        if let Some(qualifier_index) = protocol_qualifier.find("TYPE_") {
                                                            &protocol_qualifier[qualifier_index + "TYPE_".len()..]
                                                        } else {
                                                            ""
                                                        }
                                                    }
                                                } else {
                                                    ""
                                                }
                                            };

                                            html! {
                                                <div class="endpoint-wrapper">
                                                    <div
                                                        class={classes!(
                                                            "endpoint-square",
                                                            if is_service { "service-highlighted" } else { "" },
                                                            if is_client_highlighted { "client-highlighted" } else { "" },
                                                            if is_edge_highlighted { "client-highlighted" } else { "" },
                                                            if is_lower_highlighted { "lower-highlighted" } else { "" },
                                                            if is_connection_highlighted { "lower-highlighted" } else { "" },
                                                            if is_highlighted { "highlighted" } else { "" },
                                                            if is_selected { "selected" } else { "" },
                                                            if service_flag { "first" } else { "second" }
                                                        )}
                                                        onmouseover={onmouseover}
                                                        onmouseout={onmouseout}
                                                        onclick={on_click}
                                                    >
                                                        {""}
                                                    </div>
                                                    <div class="endpoint-details">{format!("{} /\n{}", last_nepu, qualifier)}</div>
                                                </div>
                                            }
                                        })
                                    }
                                </div>
                            </div>
                        }
                    })
                }
            </div>
            <div class="legend">
                <div class="legend-item">
                    <div class="color-box red"></div>
                    <span>{ "service interface points" }</span>
                </div>
                <div class="legend-item">
                    <div class="color-box yellow"></div>
                    <span>{ "lower-connection" }</span>
                </div>
                <div class="legend-item">
                    <div class="color-box pink"></div>
                    <span>{ "client-connection" }</span>
                </div>
                <div class="legend-item">
                    <div class="color-box brown"></div>
                    <span>{ "link-connection" }</span>
                </div>
                <div class="legend-item">
                    <div class="color-box blue"></div>
                    <span>{ "selected endpoint" }</span>
                </div>
            </div>
        </div>
    }
}

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
    let is_modal_open = use_state(|| false);
    let selected_value = use_state(|| Option::<Value>::None);
    let highlighted_link_uuids = use_state(|| HashSet::<String>::new());
    let highlighted_lower_connections = use_state(|| HashSet::<String>::new());
    let highlighted_connections_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_edge_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_client_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_service_uuid = use_state(|| HashSet::<String>::new());

    // Callback to handle right-click (context menu) events and update hover data
    let oncontextmenu = {
        let hover_data = hover_data.clone();
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |data: String| {
            hover_data.set(data);
            is_modal_open.set(true); // Muestra el pop-up al hacer clic derecho
        })
    };

    // Prevenir que se muestre el menú contextual predeterminado
    let prevent_default_context_menu = Callback::from(|event: MouseEvent| {
        event.prevent_default();
    });

    // Callback para cerrar el pop-up
    let close_modal = {
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |_| {
            is_modal_open.set(false); // Cerrar el pop-up
        })
    };

    // Callback para manejar el clic en los endpoints y gestionar el resaltado
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

            // Ahora iteramos sobre los nodos y luego sobre sus inventories para buscar los endpoints
            if let Some(link_uuid) = ep["link_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(inventories) = node["inventories"].as_array() {
                            for inventory in inventories {
                                if let Some(endpoints) = inventory["endpoints"].as_array() {
                                    for endpoint in endpoints {
                                        // Comparamos el link_uuid para resaltar los endpoints relacionados
                                        if endpoint["link_uuid"].as_str() == Some(link_uuid) && endpoint != &ep {
                                            new_highlighted_link_uuids.insert(link_uuid.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Resaltar los endpoints conectados vía las claves lower_connection_
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
                    if let Some(inventories) = node["inventories"].as_array() {
                        for inventory in inventories {
                            if let Some(endpoints) = inventory["endpoints"].as_array() {
                                for endpoint in endpoints {
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
                }
            }

            // Resaltar endpoints que compartan el mismo connection_end_point_uuid
            if let Some(connection_uuid) = ep["connection_end_point_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(inventories) = node["inventories"].as_array() {
                            for inventory in inventories {
                                if let Some(endpoints) = inventory["endpoints"].as_array() {
                                    for endpoint in endpoints {
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
                }
            }

            // Resaltar endpoints con el mismo client_node_edge_point_uuid
            if let Some(client_uuid) = ep["client_node_edge_point_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(inventories) = node["inventories"].as_array() {
                            for inventory in inventories {
                                if let Some(endpoints) = inventory["endpoints"].as_array() {
                                    for endpoint in endpoints {
                                        if endpoint["node_edge_point_uuid"].as_str() == Some(client_uuid) && endpoint != &ep {
                                            new_highlighted_client_uuid.insert(client_uuid.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Resaltar endpoints con el mismo node_edge_point_uuid
            if let Some(edge_uuid) = ep["node_edge_point_uuid"].as_str() {
                if let Some(nodes_array) = nodes.as_array() {
                    for node in nodes_array {
                        if let Some(inventories) = node["inventories"].as_array() {
                            for inventory in inventories {
                                if let Some(endpoints) = inventory["endpoints"].as_array() {
                                    for endpoint in endpoints {
                                        if endpoint["client_node_edge_point_uuid"].as_str() == Some(edge_uuid) && endpoint != &ep {
                                            new_highlighted_edge_uuid.insert(edge_uuid.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Actualizar los estados con los nuevos valores resaltados
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
            <div class="new-container-good">
                {
                    for nodes.iter().map(|node| {
                        let node_uuid = node["node_uuid"].as_str().unwrap_or("Unknown UUID");
                        let inventories = node["inventories"].as_array().unwrap_or(&empty_array); // Accedemos a los inventarios

                        html! {
                            <div class="node-item-good">
                                <h2>{ format!("Node UUID: {}", node_uuid) }</h2>
                                <div class="inventories-container-good">
                                    {
                                        for inventories.iter().map(|inventory| {
                                            let endpoints = inventory["endpoints"].as_array().unwrap_or(&empty_array); // Accedemos a los endpoints dentro del inventario
                                            let inventory_id = inventory["inventory_id"].as_str().unwrap_or("Unknown Inventory ID");

                                            html! {
                                                <div class="inventory-item-good">
                                                <h3>
                                                    { format!("{}", inventory_id) }
                                                </h3>
                                                    <div class="endpoints-container-good">
                                                        {
                                                            for endpoints.iter().enumerate().map(|(_, ep)| {
                                                                let ep_json = ep.clone();
                                                                let ep_json_str = ep.to_string();
                                                                // Callback para manejar el clic derecho y prevenir el menú contextual
                                                                let oncontextmenu = oncontextmenu.reform(move |_: MouseEvent| ep_json_str.clone());
                                                                let prevent_default_context_menu = prevent_default_context_menu.clone();
                                                                let on_click = {
                                                                    let ep_clone = ep_json.clone();
                                                                    on_click.reform(move |_| ep_clone.clone())
                                                                };
    
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
                                                                            oncontextmenu={prevent_default_context_menu.clone()}
                                                                            oncontextmenu={oncontextmenu.clone()}
                                                                            onclick={on_click}
                                                                        >
                                                                            {""}
                                                                        </div>
                                                                        <div class="endpoint-details">{format!("{} / {}", last_nepu, qualifier)}</div>
                                                                    </div>
                                                                }
                                                            })
                                                        }
                                                        // Modal (pop-up) que aparece con el clic derecho
                                                        if *is_modal_open {
                                                            <div class="modal-overlay" onclick={close_modal.clone()}>
                                                                <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                                                    <button class="close-button" onclick={close_modal.clone()}>{"X"}</button>
                                                                    <div class="modal-body">
                                                                        <h2>{"Endpoint Details:\n"}</h2>
                                                                        <p>{ format_json(&serde_json::from_str(&hover_data).unwrap_or(Value::Null)) }</p>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    </div>
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
        </div>
    }
}


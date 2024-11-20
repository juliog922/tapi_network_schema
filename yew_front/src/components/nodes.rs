use yew::prelude::*;
use serde_json::Value;
use std::collections::HashSet;
use wasm_bindgen::JsCast;

fn get_position(id: &str) -> Option<(f64, f64)> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let element = document.get_element_by_id(id)?;
    let html_element = element.dyn_into::<web_sys::HtmlElement>().ok()?;
    let rect = html_element.get_bounding_client_rect();

    let body = document.body().unwrap();
    let body_rect = body.get_bounding_client_rect();

    // Ajusta las coordenadas al centro del elemento
    let mut x = rect.x() + rect.width() / 2.0;
    let mut y = rect.y() + rect.height() / 2.0;

    // Corrige el desplazamiento por el scroll de la ventana
    let scroll_x = window.scroll_x().ok()?;
    let scroll_y = window.scroll_y().ok()?;
    x += scroll_x;
    y += scroll_y;

    x -= body_rect.x();
    Some((x, y))
}

#[derive(Properties, PartialEq)]
pub struct NodesProps {
    /// The JSON data representing nodes and their endpoints.
    pub nodes: Value,
}

fn highlight_class(
    is_selected: bool,
    is_service: bool,
    is_client_highlighted: bool,
    is_parent_highlighted: bool,
    is_lower_highlighted: bool,
    is_higher_highlighted: bool,
    is_connection_highlighted: bool,
    is_link_highlighted: bool,
) -> &'static str {
    if is_selected {
        "selected"
    } else if is_service {
        "service-highlighted"
    } else if is_client_highlighted {
        "client-highlighted"
    } else if is_parent_highlighted {
        "client-highlighted"
    } else if is_connection_highlighted {
        "connection-highlighted"
    } else if is_lower_highlighted {
        "lower-highlighted"
    } else if is_higher_highlighted {
        "lower-highlighted"
    } else if is_link_highlighted {
        "link-highlighted"
    } else {
        ""
    }
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
    let highlighted_higher_connections = use_state(|| HashSet::<String>::new());
    let highlighted_connections_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_client_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_parent_uuid = use_state(|| HashSet::<String>::new());
    let highlighted_service_uuid = use_state(|| HashSet::<String>::new());

    let positions_pairs_vec = use_state(|| Vec::<((f64, f64), (f64, f64), String)>::new());


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
        let highlighted_higher_connections = highlighted_higher_connections.clone();
        let highlighted_connections_uuid = highlighted_connections_uuid.clone();
        let highlighted_client_uuid = highlighted_client_uuid.clone();
        let highlighted_parent_uuid = highlighted_parent_uuid.clone();
        let highlighted_service_uuid = highlighted_service_uuid.clone();
        let positions_pairs_vec = positions_pairs_vec.clone();
        
        let nodes = props.nodes.clone();
        
        Callback::from(move |ep: Value| {

            let selected_postion = get_position(ep["node_edge_point_uuid"].as_str().unwrap()).unwrap();

            selected_value.set(Some(ep.clone()));
            let mut new_highlighted_link_uuids: HashSet<String> = HashSet::new();
            let mut new_highlighted_lower_connections: HashSet<String> = HashSet::new();
            let mut new_highlighted_higher_connections: HashSet<String> = HashSet::new();
            let mut new_highlighted_connections_uuid: HashSet<String> = HashSet::new();
            let mut new_highlighted_client_uuid: HashSet<String> = HashSet::new();
            let mut new_highlighted_parent_uuid: HashSet<String> = HashSet::new();
            let mut new_highlighted_service_uuid: HashSet<String> = HashSet::new();
            let mut new_positions_pairs_vec: Vec::<((f64, f64), (f64, f64), String)> = Vec::new();

            if let Some(nodes_array) = nodes.as_array() {
                for node in nodes_array {
                    if let Some(inventories) = node["inventories"].as_array() {
                        for inventory in inventories {
                            if let Some(endpoints) = inventory["endpoints"].as_array() {
                                for endpoint in endpoints {

                                    //Link UUID Check
                                    if let Some(link_uuid_guest) = endpoint.get("link_uuid") {
                                        if let Some(link_uuid_home) = ep.get("link_uuid") {
                                            if link_uuid_guest.eq(link_uuid_home) {
                                                
                                                new_positions_pairs_vec.push(
                                                    (selected_postion, 
                                                    get_position(endpoint["node_edge_point_uuid"].as_str().unwrap()).unwrap(),
                                                    "brown".to_string()
                                                    )
                                                );
                                                new_highlighted_link_uuids.insert(link_uuid_guest.to_string());

                                            }
                                        }
                                    }

                                    //Connection UUID and Service interface Point UUID
                                    if let Some(connection_uuid_guest) = endpoint.get("connection_uuid") {
                                        if let Some(connection_uuid_home) = ep.get("connection_uuid") {
                                            if connection_uuid_guest.eq(connection_uuid_home) {

                                                new_positions_pairs_vec.push(
                                                    (selected_postion, 
                                                    get_position(endpoint["node_edge_point_uuid"].as_str().unwrap()).unwrap(),
                                                    "blue".to_string()
                                                    )
                                                );
                                                new_highlighted_connections_uuid.insert(connection_uuid_guest.to_string());

                                                if let Some(_) = endpoint.get("service_interface_point_uuid") {

                                                    new_highlighted_service_uuid.insert(connection_uuid_guest.to_string());

                                                }

                                            }
                                        }
                                    }

                                    //Lower Connection
                                    if let Some(lower_connection_home) = ep.get("lower_connection") {
                                        if let Some(connection_uuid_guest) = endpoint.get("connection_uuid") {
                                            if lower_connection_home == connection_uuid_guest {

                                                new_positions_pairs_vec.push(
                                                    (selected_postion, 
                                                    get_position(endpoint["node_edge_point_uuid"].as_str().unwrap()).unwrap(),
                                                    "#FFD700".to_string()
                                                    )
                                                );
                                                new_highlighted_lower_connections.insert(connection_uuid_guest.to_string());

                                            }
                                        }
                                    }

                                    //Higher Connection
                                    if let Some(connection_uuid_home) = ep.get("connection_uuid") {
                                        if let Some(lower_connection_guest) = endpoint.get("lower_connection") {
                                            if lower_connection_guest == connection_uuid_home {

                                                new_positions_pairs_vec.push(
                                                    (selected_postion, 
                                                    get_position(endpoint["node_edge_point_uuid"].as_str().unwrap()).unwrap(),
                                                    "#FFD700".to_string()
                                                    )
                                                );
                                                new_highlighted_higher_connections.insert(lower_connection_guest.to_string());

                                            }
                                        }
                                    }

                                    //Client Connection
                                    if let Some(client_node_edge_point_uuid_home) = ep.get("client_node_edge_point_uuid") {
                                        if let Some(node_edge_point_uuid_guest) = endpoint.get("node_edge_point_uuid") {
                                            if client_node_edge_point_uuid_home == node_edge_point_uuid_guest {

                                                new_highlighted_client_uuid.insert(node_edge_point_uuid_guest.to_string());

                                            }
                                        }
                                    }

                                    //Parent Connection
                                    if let Some(client_node_edge_point_uuid_guest) = endpoint.get("client_node_edge_point_uuid") {
                                        if let Some(node_edge_point_uuid_home) = ep.get("node_edge_point_uuid") {
                                            if client_node_edge_point_uuid_guest == node_edge_point_uuid_home {

                                                new_highlighted_parent_uuid.insert(client_node_edge_point_uuid_guest.to_string());

                                            }
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
            highlighted_higher_connections.set(new_highlighted_higher_connections);
            highlighted_connections_uuid.set(new_highlighted_connections_uuid);
            highlighted_client_uuid.set(new_highlighted_client_uuid);
            highlighted_parent_uuid.set(new_highlighted_parent_uuid);
            highlighted_service_uuid.set(new_highlighted_service_uuid);
            positions_pairs_vec.set(new_positions_pairs_vec);
            
            })
            
    };

    // Render nodes and their endpoints
    let empty_array: Vec<Value> = vec![];
    let nodes = props.nodes.as_array().unwrap_or(&empty_array);

    // Function to format JSON values as pretty-printed strings
    fn format_json(value: &Value) -> String {
        match value {
            Value::Object(map) => {
                let mut formatted_html = String::new();
                for (key, val) in map {
                    formatted_html.push_str(&format!(
                        "<div><span class='key'>{}</span>: {}</div>",
                        key,
                        format_json(val) // Llamada recursiva para manejar estructuras anidadas
                    ));
                }
                formatted_html
            }
            Value::Array(arr) => {
                let mut formatted_html = String::from("<div>[");
                for item in arr {
                    formatted_html.push_str(&format!("<div>{}</div>", format_json(item)));
                }
                formatted_html.push_str("</div>]");
                formatted_html
            }
            _ => format!("<span class='value'>{}</span>", value.to_string()), // Para valores simples
        }
    }

    html! {
        <div id="este">
        <svg class="line-overlay" xmlns="http://www.w3.org/2000/svg">
                {   
                    // Recorrer `postions_pairs_hashset` y dibujar una línea para cada par de puntos
                    for (*positions_pairs_vec).iter().map(|((x1, y1), (x2, y2), color)| {
                        html! {
                            <line x1={(x1).to_string()} y1={(y1).to_string()}
                                x2={(x2).to_string()} y2={(y2).to_string()}
                                stroke={color.clone()} stroke-width="1" opacity="0.5" />
                        }
                    })
                }
        </svg>
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

                                                            let is_link_highlighted = {
                                                                if let Some(link_uuid) = ep.get("link_uuid") {
                                                                    highlighted_link_uuids.contains(&link_uuid.to_string()) && !is_selected
                                                                } else {
                                                                    false
                                                                }
                                                            };

                                                            let is_connection_highlighted = {
                                                                if let Some(connection_uuid) = ep.get("connection_uuid") {
                                                                    highlighted_connections_uuid.contains(&connection_uuid.to_string()) && !is_selected
                                                                } else {
                                                                    false
                                                                }
                                                            };

                                                            let is_service = {
                                                                if let Some(connection_uuid) = ep.get("connection_uuid") {
                                                                    service_flag = true;
                                                                    highlighted_service_uuid.contains(&connection_uuid.to_string()) && !is_selected
                                                                } else {
                                                                    false
                                                                }
                                                            };

                                                            let is_client_highlighted = {
                                                                if let Some(edge_uuid) = ep.get("node_edge_point_uuid") {
                                                                    highlighted_client_uuid.contains(&edge_uuid.to_string()) && !is_selected
                                                                } else {
                                                                    false
                                                                }
                                                            };

                                                            let is_parent_highlighted = {
                                                                if let Some(client_uuid) = ep.get("client_node_edge_point_uuid") {
                                                                    highlighted_parent_uuid.contains(&client_uuid.to_string()) && !is_selected
                                                                } else {
                                                                    false
                                                                }
                                                            };

                                                            

                                                            let is_higher_highlighted = {
                                                                if let Some(lower_uuid) = ep.get("lower_connection") {
                                                                    highlighted_higher_connections.contains(&lower_uuid.to_string()) && !is_selected
                                                                } else {
                                                                    false
                                                                }
                                                            };

                                                            

                                                            let is_lower_highlighted = {
                                                                if let Some(connection_uuid) = ep.get("connection_uuid") {
                                                                    highlighted_lower_connections.contains(&connection_uuid.to_string()) && !is_selected
                                                                } else {
                                                                    false
                                                                }
                                                            };

                                                            let last_nepu: &str = {
                                                                    if let Some(node_edge_point_uuid) = ep["node_edge_point_uuid"].as_str() {
                                                                        &node_edge_point_uuid[node_edge_point_uuid.len()-5..node_edge_point_uuid.len()-1]
                                                                } else {
                                                                    ""
                                                                }
                                                            };

                                                            let qualifier: &str = {
                                                                if let Some(protocol_qualifier) = ep["layer_protocol_qualifier"].as_str() {
                                                                    if let Some(qualifier_index) = protocol_qualifier.find("QUALIFIER_") {
                                                                        &protocol_qualifier[qualifier_index + "QUALIFIER_".len()..protocol_qualifier.len()-1]
                                                                    } else {
                                                                        if let Some(qualifier_index) = protocol_qualifier.find("TYPE_") {
                                                                            &protocol_qualifier[qualifier_index + "TYPE_".len()..protocol_qualifier.len()-1]
                                                                        } else {
                                                                            ""
                                                                        }
                                                                    }
                                                                } else {
                                                                    ""
                                                                }
                                                            };

                                                            let id: &str = {
                                                                if let Some(node_edge_point_uuid) = ep["node_edge_point_uuid"].as_str() {
                                                                    node_edge_point_uuid
                                                                } else {
                                                                    ""
                                                                }
                                                            };

                                                            html! {
                                                                <div class="endpoint-wrapper">
                                                                    <div
                                                                        id={format!("{}", id)}
                                                                        class={classes!(
                                                                            "endpoint-square",
                                                                            highlight_class(
                                                                                is_selected,
                                                                                is_service,
                                                                                is_client_highlighted,
                                                                                is_parent_highlighted,
                                                                                is_lower_highlighted,
                                                                                is_higher_highlighted,
                                                                                is_connection_highlighted,
                                                                                is_link_highlighted,
                                                                            ),
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
        // Modal (pop-up) que aparece con el clic derecho
        if *is_modal_open {
            <div class="modal-overlay" onclick={close_modal.clone()}>
                <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                    <button class="close-button" onclick={close_modal.clone()}>{"X"}</button>
                    <div class="modal-body">
                        <h2>{"Endpoint Details:\n"}</h2>
                        <div>{ Html::from_html_unchecked(format_json(&serde_json::from_str(&hover_data).unwrap_or(Value::Null)).into()) }</div>
                    </div>
                </div>
            </div>
        } 
        </div>        
    }
}


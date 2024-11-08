use yew::prelude::*;
use yew_router::prelude::*;
use serde_json::Value;

use crate::{
    Route,
    contexts::node_context::NodesContext,
};

#[derive(Properties, PartialEq)]
pub struct ConnectivityServiceProps {
    pub services: Value,
    pub device_ip: String,
}

#[function_component(ConnectivityService)]
pub fn connectivity_service_list(props: &ConnectivityServiceProps) -> Html {
    let empty_array = vec![];
    let services = props.services
        .as_array()
        .unwrap_or(&empty_array);
    let title: &str = if services.is_empty() {
        "Cannot Process Schema"
    } else {
        "Connectivity Services"
    };

    let nodes_context = use_context::<NodesContext>()
        .expect("NodesContext not found");

    html! {
        <div class="services-section">
            <h1 class="services-title">{title}</h1>
            <div class="services-container">
                {
                    services.iter().enumerate().map(|(index, service)| {
                        let value_name = service["value_name"].as_str().unwrap_or("");
                        let uuid = service["uuid"].as_str().unwrap_or("");
                        let nodes = service["nodes"].clone();

                        let on_click = {
                            let nodes = nodes.clone();
                            let nodes_context = nodes_context.clone();
                            Callback::from(move |_: MouseEvent| {
                                nodes_context.set(Some(nodes.clone()));
                            })
                        };

                        let animation_delay = format!("{}s", index as f32 * 0.3);

                        html! {
                            <Link<Route> to={Route::NodeSchema { ip: props.device_ip.clone(), uuid: uuid.to_string() }}>
                                <div class="service-item" style={format!("animation-delay: {}", animation_delay)} onclick={on_click}>
                                    <div class="service-content">
                                        <p class="service-name">{ value_name }</p>
                                        <p class="service-uuid">{ format!("UUID: {}", uuid) }</p>
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

use yew::{prelude::*, platform::spawn_local};
use serde_json::Value;
use yew_router::prelude::*;

use crate::api::connection::get_services;
use crate::components::sidebar::SideBar;
use crate::Route;

/// Properties for the `ServiceSchema` component.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The IP address of the device whose schema is being fetched and displayed.
    pub device_ip: String,
}

#[function_component(ServiceSchema)]
pub fn schema(props: &Props) -> Html {
    let ip = props.device_ip.clone();
    let json_data = use_state(|| None);
    let search_query = use_state(|| String::new());

    // Fetch JSON data on component mount
    {
        let json_clone = json_data.clone();
        let ip_clone = ip.clone();
        use_effect_with((), move |_| {
            let json_clone = json_clone.clone();
            spawn_local(async move {
                match get_services(ip_clone.clone()).await {
                    Ok(fetched_json) => json_clone.set(Some(fetched_json)),
                    Err(_) => json_clone.set(Some(serde_json::json!({"error": "Failed to fetch JSON"}))),
                }
            });
            || ()
        });
    }

    // Filtrar los servicios por el valor de búsqueda
    let filtered_services = {
        if let Some(services) = (*json_data).clone() {
            if let Some(services_array) = services.as_array() {
                let query = (*search_query).clone().to_lowercase();
                services_array
                    .iter()
                    .filter_map(|service| {
                        let uuid = service
                            .get("uuid")
                            .unwrap_or(&Value::default())
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                        let name = service
                            .get("name")
                            .unwrap_or(&Value::default())
                            .as_str()
                            .unwrap_or("")
                            .to_string();

                        if uuid.to_lowercase().contains(&query) || name.to_lowercase().contains(&query) {
                            Some(serde_json::json!({ "uuid": uuid, "name": name }))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    };

    // Mostrar contenido basado en el estado de carga
    let content = if json_data.is_none() {
        html! {
            <div class="loading-section">
                <div class="lds-grid">
                    <div></div>
                    <div></div>
                    <div></div>
                    <div></div>
                    <div></div>
                    <div></div>
                    <div></div>
                    <div></div>
                    <div></div>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>{"UUID"}</th>
                            <th>{"Name"}</th>
                            <th>{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for filtered_services.clone().iter().map(|service| {
                            let uuid = service.get("uuid").unwrap_or(&Value::default()).as_str().unwrap_or("?").to_string();
                            let name = service.get("name").unwrap_or(&Value::default()).as_str().unwrap_or("?").to_string();
                            html! {
                                <tr>
                                    <td>{ uuid.clone() }</td>
                                    <td>{ name.clone() }</td>
                                    <td>
                                        <button class="check-nodes-button">
                                            <Link<Route> to={Route::NodeSchema { ip: ip.clone(), uuid: uuid.clone(), name: name.clone() }} classes="check-nodes-text">
                                                {"Check Nodes Schema"}
                                            </Link<Route>>
                                        </button>
                                    </td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        }
    };

    html! {
        <div id="app">
            <div class="main-services-container">
                <SideBar />
                <div class="search-container">
                    <input
                        type="text"
                        placeholder="Search by UUID or Name..."
                        value={(*search_query).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            search_query.set(value);
                        })}
                    />
                </div>
                { content }
            </div>
        </div>
    }
}

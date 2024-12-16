use yew::{prelude::*, platform::spawn_local};
use yew_router::prelude::*;

use crate::components::sidebar::SideBar;
use crate::Route;
use crate::api::connection::{get_devices, delete_device};

/// A component for displaying a list of devices.
///
/// The `Devices` component fetches and displays a list of devices from an API. 
/// It includes options to view details about each device and delete devices from the list.
///
/// The component handles the following:
/// - Fetching device data from an API on component mount.
/// - Displaying a loading indicator while data is being fetched.
/// - Rendering device information in cards, including IP, port, and user details.
/// - Providing options to view the device's API schema and delete the device.
///
/// # Functions
///
/// - **`use_effect`**: Fetches device data asynchronously when the component mounts.
/// - **`devices_html`**: Renders the device cards based on the fetched data or shows an error message if the data format is incorrect.
///
/// # Components
///
/// - **`SideBar`**: Displays a sidebar for navigation.
/// - **`Link<Route>`**: Provides navigation to different routes within the application.
#[function_component(Devices)]
pub fn devices() -> Html {
    // State to hold the fetched JSON data
    let json_data = use_state(|| None);
    
    // Fetch JSON data on component mount
    {
        let json_clone = json_data.clone();
        use_effect_with((), move |_| {
            let json_clone = json_clone.clone();
            spawn_local(async move {
                match get_devices().await {
                    Ok(fetched_json) => json_clone.set(Some(fetched_json)),
                    Err(_) => json_clone.set(Some(serde_json::json!({"error": "Failed to fetch JSON"}))),
                }
            });
            || ()
        });
    }

    // Generate HTML based on the fetched data
    let devices_html = {
        if let Some(devices) = (*json_data).clone() {
            if let Some(devices_array) = devices.as_array() {
                html! {
                    <div class="devices-container">
                        {
                            devices_array.iter().enumerate().map(|(index, device)| {

                                if let Some(id) = device.get("id") {
                                    let id = id.as_str().unwrap_or("?").to_string();
                                    html! {
                                        
                                        <div class="device-card" style={format!("animation-delay: {}s", index as f32 * 0.3)}>
                                            <div class="device-header">
                                                <span class="device-icon">{"📁"}</span>
                                                <div class="device-info">
                                                    <p>{ format!("ID: {}", id) }</p>
                                                </div>
                                            </div>
                                        <div class="device-actions">
                                            <button class="check-api-button">
                                                <Link<Route> to={Route::ServiceSchema{ip: id.clone()}} classes="check-api-text">{"Check Services List"}</Link<Route>>
                                            </button>
                                            <button onclick={
                                                Callback::from(move |e: MouseEvent| {
                                                    e.prevent_default();
                                                    // Show confirmation dialog
                                                    let is_confirmed = web_sys::window().unwrap()
                                                        .confirm_with_message("Are you sure you want to delete this device?").unwrap();
                                                    if is_confirmed {
                                                        let id_clone = id.clone(); // Clone `ip` to move into async block
                                                        spawn_local(async move {
                                                            match delete_device(&id_clone).await {
                                                                Ok(_) => {
                                                                },
                                                                Err(_) => {},
                                                            }
                                                        });
                                                    }
                                                    web_sys::window().unwrap().location().reload().unwrap()
                                                })
                                            }>{"Delete"}</button>
                                        </div>
                                        </div>
                                    }

                                } else {
                                    let ip = device["ip"].as_str().unwrap_or("?").to_string();
                                    
                                    html! {
                                        <div class="device-card" style={format!("animation-delay: {}s", index as f32 * 0.3)}>
                                            <div class="device-header">
                                                <span class="device-icon">{"📱"}</span>
                                                <div class="device-info">
                                                    <p>{ format!("IP: {}", ip) }</p>
                                                    {
                                                        if let Some(port) = device.get("port") {
                                                            html!(<p>{ format!("Port: {}", port.as_i64().unwrap_or(00000)) }</p>)
                                                        } else {
                                                            html!()
                                                        }
                                                    }
                                                </div>
                                            </div>
                                            <div class="device-actions">
                                                <button class="check-api-button">
                                                    <Link<Route> to={Route::ServiceSchema{ip: ip.clone()}} classes="check-api-text">{"Check tAPI Schema"}</Link<Route>>
                                                </button>
                                                <button onclick={
                                                    Callback::from(move |e: MouseEvent| {
                                                        e.prevent_default();
                                                        // Show confirmation dialog
                                                        let is_confirmed = web_sys::window().unwrap()
                                                            .confirm_with_message("Are you sure you want to delete this device?").unwrap();
                                                        if is_confirmed {
                                                            let ip_clone = ip.clone(); // Clone `ip` to move into async block
                                                            spawn_local(async move {
                                                                match delete_device(&ip_clone).await {
                                                                    Ok(_) => {},
                                                                    Err(_) => {},
                                                                }
                                                            });
                                                        }
                                                        web_sys::window().unwrap().location().reload().unwrap()
                                                    })
                                                }>{"Delete"}</button>
                                            </div>
                                        </div>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
            } else {
                html! {
                    <div>{"Error: Data format is incorrect."}</div>
                }
            }
        } else {
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
        }
    };

    html! {
        <div class="device-page">
            // Render the sidebar component
            <SideBar />
            
            <div class="main-device-container">
                // Render the devices HTML content
                { devices_html }
                <div class="add-device-text">
                    <span>
                        <Link<Route> to={Route::AddDevices}>
                            { "+ New Device 📱" }
                        </Link<Route>>
                    </span>
                </div>
                <div class="add-files-text">
                    <span>
                        <Link<Route> to={Route::UploadFiles}>
                            { "+ New Files 📁" }
                        </Link<Route>>
                    </span>
                </div>
            </div>
        </div>
    }
}

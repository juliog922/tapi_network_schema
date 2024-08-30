use yew::{prelude::*, platform::spawn_local};
use serde_json::Value;
use crate::api::connection::get_schema;

use crate::components::{
    sidebar::SideBar,
    services::ConnectivityService,
};

/// Properties for the `ServiceSchema` component.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The IP address of the device whose schema is being fetched and displayed.
    pub device_ip: String,
}

#[function_component(ServiceSchema)]
pub fn schema(props: &Props) -> Html {
    // State to store the JSON data fetched from the API
    let json_data = use_state(|| None);
    let ip = props.device_ip.clone();
    
    // Fetch JSON data on component mount
    {
        let json_clone = json_data.clone();
        let ip_clone = ip.clone();
        use_effect(move || {
            let json_clone = json_clone.clone();
            spawn_local(async move {
                match get_schema(ip_clone).await {
                    Ok(fetched_json) => {
                        // Set the fetched JSON data in the state
                        json_clone.set(Some(fetched_json));
                    }
                    Err(_) => {
                        // Set an error message in case of failure
                        let error_json: Value = serde_json::json!({"error": "Failed to fetch JSON"});
                        json_clone.set(Some(error_json));
                    }
                }
            });
            || () // Cleanup function (not used in this case)
        });
    }

    html! {
        <div id="app">
            // Render the sidebar component for navigation
            <SideBar />

            // Conditionally render content based on the availability of JSON data
            { 
                if let Some(data) = (*json_data).clone() {
                    // Render the `ConnectivityService` component with the fetched data
                    html! { <ConnectivityService services={data} device_ip={ip}/> }
                } else {
                    // Display a loading indicator while fetching data
                    html! { 
                        <div class="loading-section">
                            <div class="lds-grid">
                                <div></div><div></div><div></div>
                                <div></div><div></div><div></div>
                                <div></div><div></div><div></div>
                            </div>
                        </div> 
                    }
                }
            }
        </div>
    }    
}

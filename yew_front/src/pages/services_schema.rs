use yew::{prelude::*, platform::spawn_local};
use serde_json::Value;
use crate::api::connection::get_schema;

use crate::components::{
    sidebar::SideBar,
    services::ConnectivityService,
};
use crate::contexts::service_context::ServiceContext;

/// Properties for the `ServiceSchema` component.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The IP address of the device whose schema is being fetched and displayed.
    pub device_ip: String,
}

#[function_component(ServiceSchema)]
pub fn schema(props: &Props) -> Html {
    let context = use_context::<ServiceContext>().expect("ServiceContext not found");
    let ip = props.device_ip.clone();
    
    // Fetch JSON data on component mount
    {
        let context_clone = context.clone();
        let ip_clone = ip.clone();
        use_effect(move || {
            let context_clone = context_clone.clone();
            spawn_local(async move {
                // Check if the schema is already cached
                if let Some(cached_response) = context_clone.get(&ip_clone) {
                    context_clone.set(ip_clone.clone(), cached_response);
                } else {
                    match get_schema(ip_clone.clone()).await {
                        Ok(fetched_json) => {
                            // Store the fetched JSON data in the context
                            context_clone.set(ip_clone, fetched_json);
                        }
                        Err(_) => {
                            // Set an error message in case of failure
                            let error_json: Value = serde_json::json!({"error": "Failed to fetch JSON"});
                        }
                    }
                }
            });
            || () // Cleanup function (not used in this case)
        });
    }

    html! {
        <div id="app">
            <SideBar />
            {
                // Retrieve the stored response from the context
                if let Some(data) = context.get(&ip) {
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
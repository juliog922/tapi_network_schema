use yew::prelude::*;
use serde_json::json;

use crate::components::{
    sidebar::SideBar,
    nodes::Nodes,
};
use crate::contexts::node_context::NodesContext;

/// Properties for the `NodeSchema` component.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The IP address of the device associated with this schema.
    pub device_ip: String,

    /// The UUID of the service being displayed in this schema.
    pub service_uuid: String,
}

#[function_component(NodeSchema)]
pub fn node_schema(props: &Props) -> Html {
    // Retrieve the NodesContext from the context provider
    let nodes_context = use_context::<NodesContext>().expect("no context found");

    // Get the nodes data from the context or default to an empty JSON object
    let nodes = nodes_context.get().clone().unwrap_or_else(|| json!({}));

    html! {
        <div id="app">
            // Render the sidebar component for navigation
            <SideBar />

            <div class="new-container-good">
                // Display the UUID of the service
                <div class="service-uuid-title">{props.service_uuid.clone()}</div>

                // Render the `Nodes` component with the nodes data
                <Nodes nodes={nodes} />
            </div>
        </div>
    }
}

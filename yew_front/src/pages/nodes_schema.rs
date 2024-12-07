use yew::prelude::*;

use crate::components::{
    sidebar::SideBar,
    nodes::Nodes,
    footer_legend::FooterLegend,
};

/// Properties for the `NodeSchema` component.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The IP address of the device associated with this schema.
    pub device_ip: String,

    /// The UUID of the service being displayed in this schema.
    pub service_uuid: String,

    /// The UUID of the service being displayed in this schema.
    pub name: String,
}

#[function_component(NodeSchema)]
pub fn node_schema(props: &Props) -> Html {

    html! {
        <div id="app">
            // Render the sidebar component for navigation
            <SideBar />

            <div class="component-wrapper">
                // Display the UUID of the service
                <div class="service-uuid-title">
                    <div class="service-text-container">
                        {props.name.clone()}
                        <br/>
                        {props.service_uuid.clone()}
                    </div>
                </div>

                // Render the `Nodes` component with the nodes data
                <Nodes device_ip={props.device_ip.clone()} service_uuid={props.service_uuid.clone()} />
            </div>

            <FooterLegend />
        </div>
    }
}

use yew::prelude::*;

use crate::components::{footer_legend::FooterLegend, nodes::Nodes, sidebar::SideBar};

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
        <div class="node-page">
            // Render the sidebar component for navigation
            <SideBar />

            // Display the UUID of the service
            <div class="service-uuid-title">
                <div class="service-text-container">
                    {props.name.clone()}
                    <br/>
                    {props.service_uuid.clone()}
                </div>
            </div>

            <Nodes device_ip={props.device_ip.clone()} service_uuid={props.service_uuid.clone()} />

            <FooterLegend />
        </div>
    }
}

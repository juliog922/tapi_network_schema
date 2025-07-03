use crate::components::sidebar::SideBar;
use std::rc::Rc;
use yew::prelude::*;

/// The `Info` component provides a detailed informational page with a rotating viewport
/// for images and corresponding descriptive sections for different aspects of the TAPI schema.
///
/// # Overview
///
/// This component displays a series of slides, each containing an image and a descriptive section.
/// It also includes a sidebar for navigation purposes.
///
/// # Slides Data
///
/// The slides are stored in a `Rc` (reference-counted) vector, which includes tuples of:
/// - A title for the slide,
/// - A description of the slide's content,
/// - The path to the image associated with the slide.
#[function_component(Info)]
pub fn info() -> Html {
    // A reference-counted vector containing slides data
    let slides = Rc::new(vec![
        // Each tuple contains: (Title, Description, Image Path)
        ("TAPI Schema Overview", "Explore the different components of the TAPI schema. Scroll down to navigate through the various elements and their descriptions.", "images/tapi_schema.png"),
        ("1GE DSR Connectivity-Service", "Refers to a specific type of connectivity service in a network that supports 1 Gigabit Ethernet (1GE) over a DSR (Dynamic Service Request) network. This service enables high-speed data transfer and is used for various applications where 1GE connectivity is required.", "images/telecom.png"),
        ("Node", "In the context of TAPI represents a fundamental element in the network topology. A node is any device or system that can process, receive, or transmit data. It can be a router, switch, or any other networking hardware that participates in the communication process.", "images/tapi_schema.png"),
        ("ODU Layer", "ODU (Optical Data Unit) Layer pertains to the optical transport network's layer where data is encapsulated and transported over optical fibers. It is part of the ITU-T G.709 standard, which defines the structure of optical transport units.", "images/tapi_schema.png"),
        ("Endpoint", "Is the terminus of a connection, representing the start or end point where data is either sent or received. In TAPI, an endpoint is typically an interface or port on a node that is used for communication.", "images/tapi_schema.png"),
        ("ODU Link Connection", "Refers to a specific type of connection within the ODU layer that links two ODU entities. It represents the path for transmitting optical data units between these entities.", "images/tapi_schema.png"),
        ("Lower Connection", "Denotes a connection that is part of a hierarchical structure, linking lower layers or levels within the network. It connects to a higher layer or level, providing the underlying transport.", "images/tapi_schema.png"),
        ("DSR (Dynamic Service Request)", "Is a mechanism that allows for the dynamic creation and management of network services. It enables real-time adjustments and provisioning of connectivity services based on current network conditions and requirements.", "images/tapi_schema.png"),
        ("ODU Top Connection", "Refers to a high-capacity connection within the ODU layer. ODU2 and ODU4 support high-speed data transport with a payload capacity of 10Gbps and 100Gbps.", "images/tapi_schema.png"),
        ("ODU0 Top Connection", "Is the lowest capacity connection in the ODU hierarchy, providing a minimal payload capacity. It is used for transporting small amounts of data.", "images/tapi_schema.png"),
        ("DSR/ODU Switch Node", "Refers to a network node that performs switching operations between DSR and ODU layers. It manages traffic and connections between these different layers.", "images/tapi_schema.png"),
        ("ODU/DSR Layer", "Represents a layer in the network where both ODU and DSR protocols are utilized. This layer manages the interactions and transitions between these different protocols, ensuring efficient data transport and service management.", "images/tapi_schema.png"),
        ("1GE Top Connection", "Represents a connection that supports 1 Gigabit Ethernet, typically used for providing Ethernet connectivity at a high level in the network.", "images/tapi_schema.png"),
    ]);

    html! {
        <div class="info-page">
            // Render the sidebar component for navigation
            <SideBar />

            <main class="info-main">
                // Rotating viewport for images associated with each slide
                <div class="info-rotating-viewport">
                    {for slides.iter().enumerate().map(|(i, slide)| html! {
                        <div class={format!("info-face info-face-{}", i + 1)}>
                            <img src={slide.2} alt={slide.0}/>
                        </div>
                    })}
                </div>

                // Sections containing titles and descriptions for each slide
                {for slides.iter().map(|slide| html! {
                    <section class="info-section">
                        <h1 class="info-hero-text">{slide.0}</h1>
                        <p>{slide.1}</p>
                    </section>
                })}
            </main>
        </div>
    }
}

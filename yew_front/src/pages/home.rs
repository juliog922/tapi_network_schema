use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::sidebar::SideBar;

/// Properties for the `ExpandableSection` component.
///
/// # Fields
///
/// - `title`: The title of the expandable section.
/// - `content`: The HTML content to display when the section is expanded.
#[derive(Clone, PartialEq, Properties)]
pub struct SectionProps {
    pub title: String,
    pub content: Html,
}

/// An expandable section component that shows or hides content when clicked.
///
/// # Component Overview
///
/// The `ExpandableSection` component allows for content to be toggled visible or hidden by clicking on the header.
/// - It utilizes `use_node_ref` and `use_visible` hooks to manage visibility and animations based on whether the section is expanded.
/// - The `expanded` state controls whether the section content is displayed.
///
/// # Hooks
///
/// - **`use_node_ref`**: Used to get a reference to the DOM node for visibility detection.
/// - **`use_visible`**: Determines if the section is visible based on the `node_ref` state.
#[function_component(ExpandableSection)]
fn expandable_section(props: &SectionProps) -> Html {
    let node = use_node_ref();
    let visible = use_visible(node.clone(), false);

    // State to control whether the section is expanded
    let expanded = use_state(|| false);

    // Callback to toggle the expanded state when the header is clicked
    let on_click = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    html! {
        <div ref={node}>
            <div class={if visible {"fade-in"} else {"invisible"}}>
                <section class={if *expanded { "expanded" } else { "" }}>
                    <h2 onclick={on_click} class="expandable-header">
                        { props.title.clone() }
                        <span class="expand-icon">{ if *expanded { "▲" } else { "▼" } }</span>
                    </h2>
                    { if *expanded { props.content.clone() } else { html! {} } }
                </section>
            </div>
        </div>
    }
}

/// The main home page component.
///
/// # Component Overview
///
/// The `Home` component serves as the landing page for the application, showcasing the primary features,
/// functionality, and purpose of the tool. It integrates multiple `ExpandableSection` components to provide detailed
/// information about the tool's capabilities and usage.
///
/// # Components
///
/// - **`SideBar`**: Displays a sidebar for navigation purposes.
/// - **`ExpandableSection`**: Used to present different sections like Key Features, How It Works, and Why Use This Tool.
///
/// # Sections
///
/// - **Intro**: A brief introduction to the tool with an image and text.
/// - **Content**: Contains several `ExpandableSection` components to provide detailed information about the tool.
/// - **Closing**: A final section with closing remarks that becomes visible with animation.
#[function_component(Home)]
pub fn home() -> Html {
    let closing_node = use_node_ref();
    let closing_visible = use_visible(closing_node.clone(), false);

    html! {
        <div id="app">
            // Render the sidebar component
            <SideBar />

            <div class="main-content">
                <div class="intro">
                    <div class="text-container">
                        <h1 class="fade-in">{ "TAPI Network Schema Web Tool" }</h1>
                        <section class="overview fade-in">
                            <p>
                                { "Welcome to the tAPI Network Schema Web Tool, your comprehensive solution for visualizing and managing network transport structures with ease. This tool is designed to streamline your interaction with the tAPI (Transport API) framework, making it simpler to understand and manipulate network structures. Here’s an overview of the features and capabilities that our tool offers:" }
                            </p>
                        </section>
                    </div>
                    <img class="fade-in" src="images/telecom.png" alt="telecom"/>
                </div>
                <div class="content">
                    // Render expandable sections for key features, how it works, and why use this tool
                    <ExpandableSection
                        title="Key Features"
                        content={
                            html! {
                                <ul class="fade-in">
                                    <li>{ "Device Management: Easily add, edit, or delete network devices. Specify IP address, port, username, and password to integrate new devices or update existing ones." }</li>
                                    <li>{ "Interactive Structure Visualization: View a graphical representation of your network’s tAPI layout. Navigate through nodes, edge points, connectivity services, and connections." }</li>
                                    <li>{ "Documentation Access: Read comprehensive documentation about the tAPI structure and its components to help you understand and use the tool effectively." }</li>
                                    <li>{ "Device Structure Analysis: Analyze the structure of individual devices, view their detailed configuration, and monitor real-time data to optimize network performance." }</li>
                                </ul>
                            }
                        }
                    />
                    <ExpandableSection
                        title="How It Works"
                        content={
                            html! {
                                <ol class="fade-in">
                                    <li>{ "Setup: Add your network devices through the Device Management section. Enter required details to register each device." }</li>
                                    <li>{ "Visualization: Use the Structure Visualization area to see a graphical representation of your network’s tAPI layout. Explore various elements and their relationships." }</li>
                                    <li>{ "Documentation: Access the Documentation section to read about tAPI structures and best practices. This will help you understand the underlying concepts." }</li>
                                    <li>{ "Device Analysis: Choose a device to analyze its structure and connections. View detailed information and monitor real-time data to manage and optimize your network." }</li>
                                </ol>
                            }
                        }
                    />
                    <ExpandableSection
                        title="Why Use This Tool"
                        content={
                            html! {
                                <ul class="fade-in">
                                    <li>{ "Enhanced Clarity: Simplifies the complex structure of tAPI for easier understanding and management." }</li>
                                    <li>{ "Streamlined Management: Efficiently manage devices, and get instant feedback on changes." }</li>
                                    <li>{ "Informed Decision-Making: Access detailed documentation and real-time analysis for better network management decisions." }</li>
                                </ul>
                            }
                        }
                    />
                    <section class="closing" ref={closing_node}>
                        <p class={if closing_visible {"fade-in"} else {"invisible"}}>
                            { "We believe this tool will significantly enhance your ability to manage and visualize network transport structures, making your network operations more efficient and insightful. For any assistance or questions, please refer to our help section or contact our support team. Enjoy exploring and managing your network with ease!" }
                        </p>
                    </section>
                </div>
            </div>
        </div>
    }
}

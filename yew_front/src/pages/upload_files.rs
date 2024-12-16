use yew::prelude::*;
use crate::components::{
    sidebar::SideBar,
    upload_form::UploadForm
};

/// A component for the "Add Devices" page.
///
/// The `AddDevices` component renders the layout for the page where users can
/// add new devices. It includes a sidebar and a form for adding devices.
///
/// # Components
///
/// - **`SideBar`**: A sidebar component that provides navigation options for the application.
/// - **`AddDeviceForm`**: A form component for adding new devices to the system.
///
/// # Layout
///
/// - **`<SideBar />`**: Renders the sidebar on the left side of the page.
/// - **`<div class="main-content">`**: Contains the main content area, which includes the device addition form.
#[function_component(UploadFiles)]
pub fn upload_files() -> Html {
    html! {
        <div class="upload-files-page">
            // Render the sidebar component
            <SideBar />
            
            <div class="main-content">
                // Render the form for adding new devices
                <UploadForm />
            </div>
        </div>
    }
}

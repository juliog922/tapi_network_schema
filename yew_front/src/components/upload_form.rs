use crate::api::connection::upload_connectivity_files;
use crate::components::{alert::Alert, button::Button};
use crate::Route;
use web_sys::{File, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[function_component(UploadForm)]
pub fn upload_form() -> Html {
    let navigator = use_navigator().expect("Navigator not available");

    // State hooks for file details and error messages
    let error_message_handle = use_state(String::default);
    let success_message_handle = use_state(String::default); // New state for success message
    let use_complete_context_handle = use_state(|| true);
    let id_handle = use_state(String::default);
    let complete_context_handle = use_state(Option::<File>::default);
    let topology_handle = use_state(Option::<web_sys::File>::default);
    let connections_handle = use_state(Option::<web_sys::File>::default);
    let connectivity_services_handle = use_state(Option::<web_sys::File>::default);

    // Cloning values for use in async code
    let id = (*id_handle).clone();
    let complete_context = (*complete_context_handle).clone();
    let topology = (*topology_handle).clone();
    let connections = (*connections_handle).clone();
    let connectivity_services = (*connectivity_services_handle).clone();

    let on_change_id = {
        let id_handle = id_handle.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                id_handle.set(input.value());
            }
        })
    };

    let on_change_complete_context = {
        let complete_context_handle = complete_context_handle.clone();

        Callback::from(move |e: Event| {
            // Convierte el `target` a `HtmlInputElement`
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                // Obtiene la lista de archivos
                if let Some(file_list) = input.files() {
                    // Obtén el primer archivo si existe
                    if let Some(file) = file_list.get(0) {
                        complete_context_handle.set(Some(file)); // Actualiza el estado con el archivo seleccionado
                    }
                }
            }
        })
    };

    let on_change_topology = {
        let topology_handle = topology_handle.clone();

        Callback::from(move |e: Event| {
            // Convierte el `target` a `HtmlInputElement`
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                // Obtiene la lista de archivos
                if let Some(file_list) = input.files() {
                    // Obtén el primer archivo si existe
                    if let Some(file) = file_list.get(0) {
                        topology_handle.set(Some(file)); // Actualiza el estado con el archivo seleccionado
                    }
                }
            }
        })
    };

    let on_change_connections = {
        let connections_handle = connections_handle.clone();

        Callback::from(move |e: Event| {
            // Convierte el `target` a `HtmlInputElement`
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                // Obtiene la lista de archivos
                if let Some(file_list) = input.files() {
                    // Obtén el primer archivo si existe
                    if let Some(file) = file_list.get(0) {
                        connections_handle.set(Some(file)); // Actualiza el estado con el archivo seleccionado
                    }
                }
            }
        })
    };

    let on_change_connectivity_services = {
        let connectivity_services_handle = connectivity_services_handle.clone();

        Callback::from(move |e: Event| {
            // Convierte el `target` a `HtmlInputElement`
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                // Obtiene la lista de archivos
                if let Some(file_list) = input.files() {
                    // Obtén el primer archivo si existe
                    if let Some(file) = file_list.get(0) {
                        connectivity_services_handle.set(Some(file)); // Actualiza el estado con el archivo seleccionado
                    }
                }
            }
        })
    };

    let toggle_mode = {
        let use_complete_context_handle = use_complete_context_handle.clone();
        Callback::from(move |_: Event| {
            use_complete_context_handle.set(!*use_complete_context_handle);
        })
    };

    let cloned_id = id.clone();
    let cloned_complete_context = complete_context.clone();
    let cloned_topology = topology.clone();
    let cloned_connections = connections.clone();
    let cloned_connectivity_services = connectivity_services.clone();
    let cloned_success_message_handle = success_message_handle.clone();
    let cloned_error_message_handle = error_message_handle.clone();
    let cloned_use_complete_context_handle = use_complete_context_handle.clone();

    // Handle the click event for the "Upload All Files" button
    let on_submit_upload = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_navigator = navigator.clone();

        let id = cloned_id.clone();
        let complete_context = cloned_complete_context.clone();
        let topology = cloned_topology.clone();
        let connections = cloned_connections.clone();
        let connectivity_services = cloned_connectivity_services.clone();
        let success_message_handle = cloned_success_message_handle.clone();
        let error_message_handle = cloned_error_message_handle.clone();
        let use_complete_context = cloned_use_complete_context_handle.clone();

        if id.is_empty() {
            error_message_handle.set("Please provide an ID.".to_string());
            return;
        }

        if *use_complete_context {
            if let Some(context_file) = complete_context.clone() {
                spawn_local(async move {
                    success_message_handle.set("Processing Response ...".to_string());
                    match upload_connectivity_files(None, None, None, Some(context_file), id).await
                    {
                        Ok(_) => {
                            cloned_navigator.push(&Route::Devices);
                        }
                        Err(e) => {
                            error_message_handle.set(format!("Error during upload: {}", e));
                        }
                    }
                });
            } else {
                error_message_handle.set("Please upload the complete context file.".to_string());
            }
        } else if let (
            Some(topology_file),
            Some(connections_file),
            Some(connectivity_services_file),
        ) = (
            topology.clone(),
            connections.clone(),
            connectivity_services.clone(),
        ) {
            spawn_local(async move {
                match upload_connectivity_files(
                    Some(topology_file),
                    Some(connections_file),
                    Some(connectivity_services_file),
                    None,
                    id,
                )
                .await
                {
                    Ok(_) => {
                        cloned_navigator.push(&Route::Devices);
                    }
                    Err(e) => {
                        error_message_handle.set(format!("Error during upload: {}", e));
                    }
                }
            });
        } else {
            error_message_handle.set("Please upload all three files.".to_string());
        }
    });

    // Render the form
    html! {
        <form class="upload-form-container" onsubmit={on_submit_upload}>
            <div class="form-group">
                <label for="id-input">{"ID: "}</label>
                <input
                    id="id-input"
                    type="text"
                    value={(*id_handle).clone()}
                    oninput={on_change_id}
                    required=true
                />
            </div>

            <div class="form-group toggle-container">
                <label for="mode-toggle" class="toggle-label">
                    {if *use_complete_context_handle { "Switch to Individual Files" } else { "Switch to Complete Context" }}
                </label>
                <input
                    id="mode-toggle"
                    type="checkbox"
                    class="toggle-switch"
                    checked={*use_complete_context_handle}
                    onchange={toggle_mode}
                />
            </div>

            if *use_complete_context_handle {
                <div class="file-upload-container">
                    <label for="complete-context-upload-id">{"tapi-common:context"}</label>
                    <input
                        id="complete-context-upload-id"
                        type="file"
                        accept="application/json"
                        onchange={on_change_complete_context}
                    />
                </div>
                <div class={"fake-file-upload-container"}/>
                <div class={"fake-file-upload-container"}/>
            } else {
                <div class="file-upload-container">
                    <label for="topology-upload-id">{"tapi-topology:topology-context/topology"}</label>
                    <input
                        id="topology-upload-id"
                        type="file"
                        accept="application/json"
                        onchange={on_change_topology}
                    />
                </div>
                <div class="file-upload-container">
                    <label for="connections-upload-id">{"tapi-connectivity:connectivity-context/connection"}</label>
                    <input
                        id="connections-upload-id"
                        type="file"
                        accept="application/json"
                        onchange={on_change_connections}
                    />
                </div>
                <div class="file-upload-container">
                    <label for="connectivity-services-upload-id">{"tapi-connectivity:connectivity-context/connectivity-service"}</label>
                    <input
                        id="connectivity-services-upload-id"
                        type="file"
                        accept="application/json"
                        onchange={on_change_connectivity_services}
                    />
                </div>
            }

            if !error_message_handle.is_empty() {
                <Alert alert_type="danger" message={(*error_message_handle).clone()} />
            }

            if !success_message_handle.is_empty() {
                <Alert alert_type="success" message={(*success_message_handle).clone()} />
            }

            <Button btn_type="submit" class="primary" onclick={None::<Callback<MouseEvent>>} message="Upload All Files"/>
        </form>
    }
}

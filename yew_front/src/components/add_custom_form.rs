use serde_json::Value;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::connection::{add_device, Auth, CustomAuth, Device};
use crate::components::{alert::Alert, button::Button, input::Input};
use crate::Route;

/// A functional Yew component that renders a form to add a new device.
///
/// This component provides a form with input fields for the host, port, user, and password of the device.
/// It handles form submission by making an asynchronous request to add the device to the server.
/// On success, it redirects to the `Devices` route; on failure, it displays an error message.
#[function_component(AddCustomForm)]
pub fn add_device_form() -> Html {
    let navigator = use_navigator().expect("Navigator not available");

    // State hooks to manage form input values and error messages
    let error_message_handle = use_state(String::default);
    let host_handle = use_state(String::default);
    let port_handle: UseStateHandle<Option<i64>> = use_state(|| None);
    let auth_body_handle = use_state(String::default);
    let auth_sufix_handle = use_state(String::default);

    // Cloning state values for use in async tasks and callbacks
    let host = (*host_handle).clone();
    let port = *port_handle;
    let auth_body = (*auth_body_handle).clone();
    let auth_sufix = (*auth_sufix_handle).clone();
    let error_message = (*error_message_handle).clone();

    // Callbacks to handle input change
    let on_change_host = {
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                host_handle.set(input.value());
            }
        })
    };

    let on_change_port = {
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                port_handle.set(Some(input.value().parse::<i64>().unwrap()));
            }
        })
    };

    let on_change_auth_body = {
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                auth_body_handle.set(input.value());
            }
        })
    };

    let on_change_auth_sufix = {
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                auth_sufix_handle.set(input.value());
            }
        })
    };

    // Cloning values for use in async code
    let cloned_host = host.clone();
    let cloned_port = port;
    let cloned_auth_sufix = auth_sufix.clone();
    let cloned_auth_body = auth_body.clone();

    // Callback for form submission
    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default(); // Prevent default form submission behavior

        // Cloning state handles for async block
        let cloned_navigator = navigator.clone();
        let cloned_host = cloned_host.clone();
        let cloned_port = cloned_port;
        let cloned_auth_sufix = cloned_auth_sufix.clone();
        let cloned_auth_body = cloned_auth_body.clone();
        let cloned_error_message_handle = error_message_handle.clone();

        // Asynchronous task to handle form submission
        spawn_local(async move {
            // Call the API to add the device
            let result = add_device(Device {
                ip: cloned_host,
                port: cloned_port,
                auth: Auth::Custom(CustomAuth {
                    auth_body: serde_json::from_str::<Value>(cloned_auth_body.as_str()).unwrap(),
                    auth_sufix: cloned_auth_sufix,
                }),
            })
            .await;

            // Handle the result of the API call
            if result.is_ok() {
                // Redirect on success
                cloned_navigator.push(&Route::Devices);
            } else if let Err(e) = result {
                // Set error message on failure
                cloned_error_message_handle.set(e.to_string());
                // Automatically clear the error message after 2 seconds
                gloo::timers::future::TimeoutFuture::new(2000).await;
                cloned_error_message_handle.set(String::new());
            }
        });
    });

    // Render the form
    html! {
        <form class="form-container" onsubmit={on_submit}>
            if !error_message.is_empty() {
                <Alert alert_type={"danger"} message={error_message}/>
            }
            <div class="input-group">
                <Input
                    input_type="ip"
                    name="ip"
                    label="Ip"
                    placeholder="127.0.0.1"
                    value={host}
                    onchange={on_change_host}
                />
            </div>
            <div class="input-group">
                <Input
                    input_type="port"
                    name="port"
                    label="Port"
                    placeholder="8000"
                    value={port.map(|arg0: i64| i64::to_string(&arg0)).unwrap_or("".to_string())}
                    onchange={on_change_port}
                />
            </div>
            <div class="input-group">
                <Input
                    input_type="auth_body"
                    name="auth_body"
                    label="Auth Body"
                    placeholder="{\"username\":\"my_username\", \"password\":\"my_password\"}"
                    value={auth_body}
                    onchange={on_change_auth_body}
                />
            </div>
            <div class="input-group">
                <Input
                    input_type="auth_sufix"
                    name="auth_sufix"
                    label="Auth Url Sufix"
                    placeholder="/tron/api/v1/tokens"
                    value={auth_sufix}
                    onchange={on_change_auth_sufix}
                />
            </div>
            <Button btn_type="submit" class="primary" onclick={None::<Callback<MouseEvent>>} message="Save"/>
        </form>
    }
}

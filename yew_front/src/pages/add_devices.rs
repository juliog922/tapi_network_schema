use yew::prelude::*;
use crate::components::{
    sidebar::SideBar,
    add_basic_form::AddBasicForm,
    add_oauth_form::AddOauthForm,
    add_custom_form::AddCustomForm,
};

#[function_component(AddDevices)]
pub fn add_devices() -> Html {
    // Estado para saber qué formulario está seleccionado
    let selected_form = use_state(|| "basic".to_string());

    // Función para cambiar el formulario seleccionado
    let on_select_form = {
        let selected_form = selected_form.clone();
        Callback::from(move |form: String| {
            selected_form.set(form);
        })
    };

    // Función para generar las clases de los botones dependiendo del estado seleccionado
    let button_class = |form: &str| {
        if *selected_form == form {
            "button-circle selected"
        } else {
            "button-circle"
        }
    };

    html! {
        <div id="app">
            // Renderizar la barra lateral
            <SideBar />

            <div class="main-device-div">
                // Botones circulares para seleccionar el formulario
                <div class="form-selector">
                    <button
                        class={button_class("basic")}
                        onclick={on_select_form.reform(|_| "basic".to_string())}
                    >
                        { "Basic Auth" }
                    </button>
                    <button
                        class={button_class("oauth2")}
                        onclick={on_select_form.reform(|_| "oauth2".to_string())}
                    >
                        { "OAuth2" }
                    </button>
                    <button
                        class={button_class("custom")}
                        onclick={on_select_form.reform(|_| "custom".to_string())}
                    >
                        { "Custom Auth" }
                    </button>
                </div>

                // Renderizar el formulario correspondiente basado en la selección
                {
                    if *selected_form == "basic" {
                        html! { <AddBasicForm /> }
                    } else if *selected_form == "oauth2" {
                        html! { <AddOauthForm /> }
                    } else {
                        html! { <AddCustomForm /> }
                    }
                }
            </div>
        </div>
    }
}

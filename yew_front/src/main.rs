use yew_router::prelude::*;
use yew::prelude::*;

mod api;
mod pages;
mod components;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/devices")]
    Devices,
    #[at("/devices/add")]
    AddDevices,
    #[at("/files")]
    UploadFiles,
    #[at("/service_schema/:ip")]
    ServiceSchema {ip: String},
    #[at("/node_schema/:ip/:uuid/:name")]
    NodeSchema {ip: String, uuid: String, name: String},
    #[at("/info")]
    Info,
    #[at("/login")]
    Login,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!{ <pages::home::Home/>},
        Route::ServiceSchema {ip} => html!{ <pages::services_schema::ServiceSchema device_ip={ip}/> },
        Route::NodeSchema {ip, uuid, name} => html!{ <pages::nodes_schema::NodeSchema device_ip={ip} service_uuid={uuid} name={name}/> },
        Route::Devices => html!{ <pages::devices::Devices/>},
        Route::AddDevices => html!{ <pages::add_devices::AddDevices/>},
        Route::UploadFiles => html!{<pages::upload_files::UploadFiles/>},
        Route::Info => html!{ <pages::info::Info/>},
        Route::Login => html!{ <pages::login::Login/>},
    }
    
}

#[function_component(App)]
fn app() -> Html {
    html!{
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

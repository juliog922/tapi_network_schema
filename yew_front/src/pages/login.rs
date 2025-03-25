use yew::prelude::*;
use crate::components::sidebar::SideBar;
use web_sys::{console, HtmlInputElement};

#[function_component(Login)]
pub fn login() -> Html {
    let x: String =  String::default();
    let y: Vec<String> = x.chars().into_iter().map(|c| c.to_string()).collect();

    let show_password = use_state(|| false);

    let toggle_password = {
        let show_password = show_password.clone();
        Callback::from(move |_| {
            show_password.set(!*show_password);
        })
    };

    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let on_login_click = Callback::from({
        let username = username.clone();
        let password = password.clone();
        move |event: MouseEvent| {
            event.prevent_default();
            console::log_1(&format!("Username: {}, Password: {}", *username, *password).into());
        }
    });

    

    html! {
        <div class="login-container">
            <SideBar />
            <div class="login-form">
                <h2>{ "Login" }</h2>
                <form>
                    <div class="form-group">
                        <label for="username">{ "Username" }</label>
                        <input
                            id="username"
                            name="username"
                            type="text"
                            placeholder="Enter your username"
                            oninput={Callback::from({
                                let username = username.clone();
                                move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    username.set(input.value());
                                }
                            })}
                        />
                    </div>
                    <div class="form-group password-group">
                        <label for="password">{ "Password" }</label>
                        <div class="password-wrapper">
                            <input
                                id="password"
                                name="password"
                                type={ if *show_password { "text" } else { "password" } }
                                placeholder="Enter your password"
                                oninput={Callback::from({
                                    let password = password.clone();
                                    move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        password.set(input.value());
                                    }
                                })}
                            />
                            <button type="button" class="toggle-password" onclick={toggle_password}>
                                { if *show_password { "Hide" } else { "Show" } }
                            </button>
                        </div>
                    </div>
                    <button type="submit" onclick={on_login_click}>
                        { "Access" }
                    </button>
                </form>
            </div>
        </div>
    }
}

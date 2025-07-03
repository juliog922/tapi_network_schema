use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// A functional component that renders a sidebar navigation menu.
///
/// The sidebar provides links to different routes within the application, including
/// "Home", "Devices", and "Info". It also includes a hamburger menu icon that can
/// be toggled to expand or collapse the menu.
///
/// # Behavior
///
/// - **Active Link Highlighting:** The sidebar highlights the link corresponding to the
///   current route, indicating the active page to the user.
/// - **Hamburger Menu Toggle:** The sidebar includes a hamburger menu icon that, when
///   clicked, toggles the visibility of the navigation links. This is useful for mobile
///   or compact views.
///
/// # Components
///
/// - **Hamburger Menu:** The hamburger menu icon toggles the expansion of the sidebar.
/// - **Navigation Links:** The sidebar contains links to the "Home", "Devices", and "Info" routes.
#[function_component(SideBar)]
pub fn sidebar() -> Html {
    // Get the current route from the router context
    let current_route = use_route::<Route>().expect("No current route defined");

    // Determine the CSS classes for each navigation link based on the current route
    let home_classes = {
        if current_route == Route::Home {
            classes!("nav-link", "active")
        } else {
            classes!("nav-link")
        }
    };

    let devices_classes = {
        if current_route == Route::Devices {
            classes!("nav-link", "active")
        } else {
            classes!("nav-link")
        }
    };

    let info_classes = {
        if current_route == Route::Info {
            classes!("nav-link", "active")
        } else {
            classes!("nav-link")
        }
    };

    // State to manage the expansion of the sidebar menu
    let expanded = use_state(|| false);

    // Callback to toggle the expanded state of the sidebar menu
    let toggle_expanded = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded); // Toggle the state between expanded and collapsed
        })
    };

    // Determine the CSS class for the menu based on the expanded state
    let menu_classes = if *expanded { "menu expanded" } else { "menu" };

    // Determine the CSS class for the hamburger menu based on the expanded state
    let hamburger_classes = if *expanded {
        "hamburger expanded"
    } else {
        "hamburger"
    };

    html! {
        <nav class="navbar">
            // Hamburger menu icon to toggle the sidebar
            <div class={hamburger_classes} onclick={toggle_expanded}>
                <div></div>
                <div></div>
                <div></div>
            </div>
            <ul class={menu_classes}>
                // Navigation links
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={home_classes}>{"Home"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Devices} classes={devices_classes}>{"Devices"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Info} classes={info_classes}>{"Info"}</Link<Route>>
                </li>
            </ul>
        </nav>
    }
}

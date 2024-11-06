use yew::prelude::*;
use yew::html::Html;

#[function_component(FooterLegend)]
pub fn footer_legend() -> Html {
    let is_expanded = use_state(|| false);
    
    let toggle_expanded = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| is_expanded.set(!*is_expanded))
    };

    html! {
        <div class="footer-legend">
            <button class="toggle-button" onclick={toggle_expanded}>
                { if *is_expanded { "Close Leyend" } else { "Open Leyend" } }
            </button>
            if *is_expanded {
                <div class="legend-content">
                    <div class="legend-item selected">{ "Selected Enpoint" }</div>
                    <div class="legend-item highlighted">{ "Link Connection" }</div>
                    <div class="legend-item lower-highlighted">{ "Lower Connection" }</div>
                    <div class="legend-item client-highlighted">{ "Client Endpoint" }</div>
                    <div class="legend-item service-highlighted">{ "Service Interface Point" }</div>
                    <div class="legend-item connection-endpoint-highlighted">{ "Connection End Point" }</div>
                </div>
            }
        </div>
    }
}

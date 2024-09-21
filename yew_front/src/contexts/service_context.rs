use yew::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

/// A context for sharing service responses across components.
#[derive(Clone, PartialEq)]
pub struct ServiceContext(UseStateHandle<HashMap<String, Value>>);

impl ServiceContext {
    pub fn new(state: UseStateHandle<HashMap<String, Value>>) -> Self {
        Self(state)
    }

    pub fn set(&self, ip: String, response: Value) {
        let mut new_map = (*self.0).clone(); // Accede al HashMap
        new_map.insert(ip, response);
        self.0.set(new_map); // Establece el nuevo HashMap
    }

    pub fn get(&self, ip: &str) -> Option<Value> {
        self.0.get(ip).cloned()
    }
}

/// Properties for the `ServiceProvider` component.
#[derive(Properties, PartialEq)]
pub struct ServiceProviderProps {
    #[prop_or_default]
    pub children: Children,
}

/// A component that provides the `ServiceContext` to its children.
#[function_component(ServiceProvider)]
pub fn service_provider(props: &ServiceProviderProps) -> Html {
    let state = use_state(|| HashMap::new());
    let context = ServiceContext::new(state.clone());

    html! {
        <ContextProvider<ServiceContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ServiceContext>>
    }
}

use yew::prelude::*;
use serde_json::Value;

/// A context for sharing `nodes` state across components.
///
/// The `NodesContext` provides a way to manage and access the state of `nodes`
/// within a Yew application. Components that consume this context can read
/// and update the `nodes` state without having to pass it down manually through props.
///
/// # Methods
///
/// - **`new`**: Creates a new `NodesContext` instance with a given `UseStateHandle`.
/// - **`set`**: Sets the `nodes` state to a new value.
/// - **`get`**: Retrieves the current value of the `nodes` state.
#[derive(Clone, PartialEq)]
pub struct NodesContext(UseStateHandle<Option<Value>>);

impl NodesContext {
    /// Creates a new `NodesContext` with a given state handle.
    ///
    /// # Arguments
    ///
    /// * `nodes_state` - A `UseStateHandle` containing the current `nodes` state.
    ///
    /// # Returns
    ///
    /// A new instance of `NodesContext`.
    pub fn new(nodes_state: UseStateHandle<Option<Value>>) -> Self {
        Self(nodes_state)
    }

    /// Sets a new value for the `nodes` state.
    ///
    /// # Arguments
    ///
    /// * `nodes` - An `Option<Value>` containing the new value to set.
    pub fn set(&self, nodes: Option<Value>) {
        self.0.set(nodes);
    }

    /// Retrieves the current value of the `nodes` state.
    ///
    /// # Returns
    ///
    /// An `Option<Value>` containing the current state of `nodes`.
    pub fn get(&self) -> Option<Value> {
        (*self.0).clone()
    }
}

/// Properties for the `NodesProvider` component.
///
/// This component wraps its children in a context provider that makes the `NodesContext`
/// available to descendant components.
///
/// # Properties
///
/// - **`children`**: The child components that will have access to the `NodesContext`.
#[derive(Properties, PartialEq)]
pub struct NodesProviderProps {
    #[prop_or_default]
    pub children: Children,
}

/// A component that provides the `NodesContext` to its children.
///
/// The `NodesProvider` component sets up the context for managing the `nodes` state,
/// allowing any child component to access or modify this state.
///
/// # Behavior
///
/// - **State Management**: The provider uses `UseStateHandle` to manage the `nodes` state,
///   making it accessible to child components through context.
/// - **Context Provision**: The component wraps its children in a `ContextProvider`, passing
///   the `NodesContext` to the children.
#[function_component(NodesProvider)]
pub fn nodes_provider(props: &NodesProviderProps) -> Html {
    // Create a state handle for managing the `nodes` state
    let nodes = use_state(|| None);
    // Create a `NodesContext` instance with the state handle
    let context = NodesContext::new(nodes.clone());

    html! {
        // Provide the `NodesContext` to child components
        <ContextProvider<NodesContext> context={context}>
            { for props.children.iter() } // Render child components
        </ContextProvider<NodesContext>>
    }
}

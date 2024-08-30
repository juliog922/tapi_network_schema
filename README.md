# TAPI Network Schema Web Tool

Welcome to the TAPI Network Schema Web Tool, your comprehensive solution for visualizing and managing network transport structures with ease. This tool is designed to streamline your interaction with the TAPI (Transport API) framework, making it simpler to understand and manipulate network structures.

## Features

- **Device Management**: Easily add, edit, or delete network devices. Specify IP address, port, username, and password to integrate new devices or update existing ones.
- **Interactive Structure Visualization**: View a graphical representation of your network’s TAPI layout. Navigate through nodes, edge points, connectivity services, and connections.
- **Documentation Access**: Read comprehensive documentation about the TAPI structure and its components to help you understand and use the tool effectively.
- **Device Structure Analysis**: Analyze the structure of individual devices, view their detailed configuration, and monitor real-time data to optimize network performance.

## Setup

1. **Install Required Tools**:
   - Ensure you have `rustup` installed. You can install it from [rustup.rs](https://rustup.rs/).
   - Install `cargo`, which is included with `rustup`.
   - Install `trunk`, a tool for building and serving Yew applications. You can install it using:
     ```bash
     cargo install trunk
     ```
   - Add the WebAssembly target for your Rust toolchain:
     ```bash
     rustup target add wasm32-unknown-unknown
     ```

2. **Add Devices**: Use the Device Management section to add your network devices. Enter required details to register each device.
3. **Visualization**: Use the Structure Visualization area to see a graphical representation of your network’s TAPI layout. Explore various elements and their relationships.
4. **Documentation**: Access the Documentation section to read about TAPI structures and best practices. This will help you understand the underlying concepts.
5. **Device Analysis**: Choose a device to analyze its structure and connections. View detailed information and monitor real-time data to manage and optimize your network.

## Running the Project

To run the project locally, follow these instructions:

### Actix API Backend

1. Navigate to the `actix_api` directory:
   ```bash
   cd actix_api
2. Run the backend server using Cargo:
    ```bash
   cargo run

### Yew Frontend

1. Navigate to the `yew_front` directory:
   ```bash
   trunk serve
2. Run the backend server using Cargo:
    ```bash
   cargo run
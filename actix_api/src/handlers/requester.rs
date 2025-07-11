use crate::handlers::http::HttpHandler;
use crate::models::devices::{Auth, Device};
use crate::models::files_model::FilesEnum;
use crate::utils::find_key_recursively;
use crate::AppError;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs::File;
use std::io::BufReader;

/// Enum representing the source of data, either from a device or from a set of files.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataSource {
    Device(Device),
    FilesEnum(FilesEnum),
}

/// Struct representing the service context, including connectivity services, connections, and topology.
pub struct Context {
    pub connectivity_service: Value,
    pub connections: Vec<Value>,
    pub topology: Value,
}

/// Handles operations related to retrieving services and service contexts from various data sources.
pub struct Requester;

impl Requester {
    /// Retrieve a list of services from the specified data source.
    ///
    /// # Arguments
    /// - `data_source`: The data source, which could be a `Device` or a `FilesEnum`.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Value` objects representing the services, or an `Error`.
    pub async fn get_services(data_source: &DataSource) -> Result<Vec<Value>, AppError> {
        match data_source {
            DataSource::Device(device) => DeviceHandler::get_services(device).await,
            DataSource::FilesEnum(file_enum) => FilesHandler::get_services(file_enum).await,
        }
    }

    /// Retrieve the context of a specific service by UUID from the data source.
    ///
    /// # Arguments
    /// - `data_source`: The data source, which could be a `Device` or a `FilesEnum`.
    /// - `service_uuid`: The UUID of the service to retrieve context for.
    ///
    /// # Returns
    /// A `Result` containing a `Context` object or an `Error`.
    pub async fn get_service_context(
        data_source: &DataSource,
        service_uuid: &String,
    ) -> Result<Context, AppError> {
        match data_source {
            DataSource::Device(device) => {
                DeviceHandler::get_service_context(device, service_uuid).await
            }
            DataSource::FilesEnum(file_enum) => {
                FilesHandler::get_service_context(file_enum, service_uuid).await
            }
        }
    }
}

/// Handles operations related to retrieving data from files.
pub struct FilesHandler;

impl FilesHandler {
    /// Retrieve services from files based on the provided `FilesEnum`.
    ///
    /// # Arguments
    /// - `file_enum`: The file representation, either split into parts or complete.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Value` objects representing the services, or an `Error`.
    pub async fn get_services(file_enum: &FilesEnum) -> Result<Vec<Value>, AppError> {
        match file_enum {
            FilesEnum::ByPart(by_part_paths) => {
                let connectivity_services_file =
                    File::open(&by_part_paths.connectivity_services_path)
                        .map_err(|err| AppError::database_error(err.to_string()))?;
                File::open(&by_part_paths.connectivity_services_path)
                    .map_err(|err| AppError::database_error(err.to_string()))?;
                let connectivity_services_reader = BufReader::new(connectivity_services_file);
                let connectivity_services_value: Value =
                    serde_json::from_reader(connectivity_services_reader)
                        .map_err(|err| AppError::validation_error(err.to_string()))?;
                let connectivity_services = connectivity_services_value
                    .as_array()
                    .cloned()
                    .unwrap_or_default();

                Ok(connectivity_services)
            }
            FilesEnum::Complete(complete_path) => {
                let file = File::open(&complete_path.complete_context_path)
                    .map_err(|err| AppError::database_error(err.to_string()))?;
                let reader = BufReader::new(file);
                let json_value: Value = serde_json::from_reader(reader)
                    .map_err(|err| AppError::validation_error(err.to_string()))?;
                let connectivity_services = json_value
                                                            .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")
                                                            .and_then(Value::as_array)
                                                            .ok_or_else(|| AppError::database_error("Cannot find connectivity-context"))?
                                                            .clone();
                Ok(connectivity_services)
            }
        }
    }

    /// Retrieve the context of a specific service from files.
    ///
    /// # Arguments
    /// - `file_enum`: The file representation, either split into parts or complete.
    /// - `service_uuid`: The UUID of the service to retrieve context for.
    ///
    /// # Returns
    /// A `Result` containing a `Context` object or an `Error`.
    pub async fn get_service_context(
        file_enum: &FilesEnum,
        service_uuid: &str,
    ) -> Result<Context, AppError> {
        match file_enum {
            FilesEnum::ByPart(by_part_paths) => {
                let topology_file = File::open(&by_part_paths.topology_path)
                    .map_err(|err| AppError::database_error(err.to_string()))?;
                let topology_reader = BufReader::new(topology_file);
                let topology: Value = serde_json::from_reader(topology_reader)
                    .map_err(|err| AppError::validation_error(err.to_string()))?;

                let connections_file = File::open(&by_part_paths.connections_path)
                    .map_err(|err| AppError::database_error(err.to_string()))?;
                let connections_reader = BufReader::new(connections_file);
                let connections_value: Value = serde_json::from_reader(connections_reader)
                    .map_err(|err| AppError::validation_error(err.to_string()))?;
                let connections = connections_value.as_array().cloned().unwrap_or_default();

                let connectivity_services_file =
                    File::open(&by_part_paths.connectivity_services_path)
                        .map_err(|err| AppError::database_error(err.to_string()))?;
                File::open(&by_part_paths.connectivity_services_path)
                    .map_err(|err| AppError::database_error(err.to_string()))?;
                let connectivity_services_reader = BufReader::new(connectivity_services_file);
                let connectivity_services_value: Value =
                    serde_json::from_reader(connectivity_services_reader)
                        .map_err(|err| AppError::validation_error(err.to_string()))?;
                let connectivity_services = connectivity_services_value
                    .as_array()
                    .cloned()
                    .unwrap_or_default();

                let connectivity_service = connectivity_services
                    .iter()
                    .find(|service| {
                        service
                            .get("uuid")
                            .and_then(|uuid| uuid.as_str())
                            .map(|uuid_str| uuid_str == service_uuid)
                            .unwrap_or(false)
                    })
                    .ok_or_else(|| {
                        AppError::validation_error("There is not any Service with that id")
                    })?
                    .clone();

                Ok(Context {
                    connectivity_service,
                    connections,
                    topology,
                })
            }
            FilesEnum::Complete(complete_path) => {
                let file = File::open(&complete_path.complete_context_path)
                    .map_err(|err| AppError::database_error(err.to_string()))?;
                let reader = BufReader::new(file);
                let json_value: Value = serde_json::from_reader(reader)
                    .map_err(|err| AppError::validation_error(err.to_string()))?;

                context_by_context_json(json_value, service_uuid)
            }
        }
    }
}

/// Handles operations related to retrieving data from devices.
pub struct DeviceHandler;

impl DeviceHandler {
    /// Retrieve services from a device via API calls.
    ///
    /// # Arguments
    /// - `device`: A reference to the `Device` object containing connection details.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Value` objects representing the services, or an `Error`.
    pub async fn get_services(device: &Device) -> Result<Vec<Value>, AppError> {
        let mut token: String = String::default();

        if let Auth::Token(token_auth) = &device.auth {
            token = HttpHandler::get_token(&device.get_full_auth_url(), &token_auth.auth_body)
                .await?;
        }

        let well_known_url = format!(
            "https://{}{}/.well-known/host-meta/",
            &device.ip,
            &device.port.map(|s| format!(":{}", s)).unwrap_or_default()
        );

        let well_known_json = match &device.auth {
            Auth::Basic(basic_auth) => {
                HttpHandler::basic_request(
                    &well_known_url,
                    basic_auth.username.clone(),
                    Some(basic_auth.password.clone()),
                )
                .await?
            }
            Auth::Token(_) => HttpHandler::token_request(&well_known_url, token.as_str()).await?,
        };

        let rel_default_value = Value::String("restconf".to_string());
        let rel_value = find_key_recursively("rel", &well_known_json)
            .unwrap_or_else(|| rel_default_value.clone());
        let rel = rel_value
            .as_str()
            .ok_or(AppError::validation_error("Invalid Rel Value"))?;

        let base_url = format!(
            "https://{}{}/{}",
            &device.ip,
            &device.port.map(|s| format!(":{}", s)).unwrap_or_default(),
            rel
        );

        let services_url = format!(
            "{}/data/tapi-common:context/tapi-connectivity:connectivity-context?fields=connectivity-service(uuid)",
            &base_url
        );

        let context_url = format!("{}/data/tapi-common:context", &base_url);

        if let Ok(services_uuid_json) = match &device.auth {
            Auth::Basic(basic_auth) => {
                HttpHandler::basic_request(
                    &services_url,
                    basic_auth.username.clone(),
                    Some(basic_auth.password.clone()),
                )
                .await
            }
            Auth::Token(_) => HttpHandler::token_request(&services_url, token.as_str()).await,
        } {
            let connectivity_services = services_uuid_json
                .pointer("/tapi-connectivity:connectivity-context/connectivity-service")
                .ok_or(AppError::validation_error("Connectivity Context Not Found"))?
                .as_array()
                .ok_or(AppError::validation_error("Invalid Connectivity Service"))?
                .clone();

            let mut services_vector: Vec<Value> = vec![];
            for service in connectivity_services {
                let service_uuid = service
                    .get("uuid")
                    .ok_or(AppError::validation_error("UUID Not Found"))?
                    .as_str()
                    .ok_or(AppError::validation_error("Invalid UUID"))?;

                let service_url = format!(
                    "{}/data/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service={}",
                    &base_url, service_uuid
                );
                let service_json = match &device.auth {
                    Auth::Basic(basic_auth) => {
                        HttpHandler::basic_request(
                            &service_url,
                            basic_auth.username.clone(),
                            Some(basic_auth.password.clone()),
                        )
                        .await?
                    }
                    Auth::Token(_) => {
                        HttpHandler::token_request(&service_url, token.as_str()).await?
                    }
                };
                let service_data = &service_json
                    .get("tapi-connectivity:connectivity-service")
                    .ok_or(AppError::validation_error("Service Data Not Found"))?
                    .as_array()
                    .unwrap()[0];
                services_vector.push(service_data.clone()); // Add the service data to the vector.
            }
            Ok(services_vector)
        } else {
            let json = match &device.auth {
                Auth::Basic(basic_auth) => {
                    HttpHandler::basic_request(
                        &context_url,
                        basic_auth.username.clone(),
                        Some(basic_auth.password.clone()),
                    )
                    .await?
                }
                Auth::Token(_) => HttpHandler::token_request(&context_url, token.as_str()).await?,
            };
            let connectivity_services = json
                .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")
                .and_then(Value::as_array)
                .ok_or_else(|| AppError::validation_error("Cannot find connectivity-context"))?
                .clone();

            Ok(connectivity_services)
        }
    }

    /// Retrieve the context of a specific service from a device.
    ///
    /// # Arguments
    /// - `device`: A reference to the `Device` object containing connection details.
    /// - `service_uuid`: The UUID of the service to retrieve context for.
    ///
    /// # Returns
    /// A `Result` containing a `Context` object or an `Error`.
    pub async fn get_service_context(
        device: &Device,
        service_uuid: &String,
    ) -> Result<Context, AppError> {
        let mut token: String = String::default();

        if let Auth::Token(token_auth) = &device.auth {
            token = HttpHandler::get_token(&device.get_full_auth_url(), &token_auth.auth_body)
                .await?;
        }

        let well_known_url = format!(
            "https://{}{}/.well-known/host-meta",
            &device.ip,
            &device.port.map(|s| format!(":{}", s)).unwrap_or_default()
        );

        let well_known_json = match &device.auth {
            Auth::Basic(basic_auth) => {
                HttpHandler::basic_request(
                    &well_known_url,
                    basic_auth.username.clone(),
                    Some(basic_auth.password.clone()),
                )
                .await?
            }
            Auth::Token(_) => HttpHandler::token_request(&well_known_url, token.as_str()).await?,
        };

        let rel_default_value = Value::String("restconf".to_string());
        let rel_value = find_key_recursively("rel", &well_known_json)
            .unwrap_or_else(|| rel_default_value.clone());
        let rel = rel_value
            .as_str()
            .ok_or(AppError::validation_error("Invalid Rel Value"))?;

        let base_url = format!(
            "https://{}{}/{}",
            &device.ip,
            &device.port.map(|s| format!(":{}", s)).unwrap_or_default(),
            rel
        );

        let topology_by_uuid_url = format!(
            "{}/data/tapi-common:context/tapi-topology:topology-context?fields=topology(uuid)",
            &base_url
        );

        let connections_url = format!(
            "{}/data/tapi-common:context/tapi-connectivity:connectivity-context",
            base_url
        );

        let context_url = format!("{}/data/tapi-common:context", base_url);

        match async {
            // Obtain the JSON of topologies based on the authentication method.
            let topology_uuids_json = match &device.auth {
                Auth::Basic(basic_auth) => HttpHandler::basic_request(&topology_by_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Token(_) => HttpHandler::token_request(&topology_by_uuid_url, token.as_str()).await?,
            };

            // Parse the context and the topology UUID.
            let topologies = topology_uuids_json
                .pointer("/tapi-topology:topology-context/topology")
                .ok_or(AppError::validation_error("Topology Context Not Found"))?
                .as_array()
                .ok_or(AppError::validation_error("Invalid Topology"))?;

            // Verify that there is exactly one topology UUID.
            if topologies.len() != 1 {
                return Err(AppError::validation_error("There is more or less than one topology uuid"));
            }

            let topology_uuid = topologies[0]
                        .get("uuid")
                        .ok_or(AppError::validation_error("Topology UUID Not Found"))?
                        .as_str()
                        .ok_or(AppError::validation_error("Invalid str UUID"))?;

            let topology_fields_url = format!(
                "{}/data/tapi-common:context/tapi-topology:topology-context/topology={topology_uuid}?fields=uuid;name;layer-protocol-name",
                &base_url
            );

            let topology_fields_json = match &device.auth {
                Auth::Basic(basic_auth) => HttpHandler::basic_request(&topology_fields_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Token(_) => HttpHandler::token_request(&topology_fields_url, token.as_str()).await?,
            };

            let topology_fields_array = topology_fields_json
                .pointer("/tapi-topology:topology")
                .ok_or(AppError::validation_error("Topology Context Not Found"))?
                .as_array()
                .ok_or(AppError::validation_error("Invalid Topology"))?;

            // Verify that there is exactly one topology UUID.
            if topology_fields_array.len() != 1 {
                return Err(AppError::validation_error("There is more or less than one topology uuid"));
            }

            let topology_fields = topology_fields_array[0]
                .as_object()
                .ok_or(AppError::validation_error("Invalid Map Topology"))?;

            let nodes_uuid_url = format!(
                "{}/data/tapi-common:context/tapi-topology:topology-context/topology={topology_uuid}?fields=node(uuid)",
                &base_url
            );
            // Obtain the JSON of nodes uuids based on the authentication method.
            let node_uuids_json = match &device.auth {
                Auth::Basic(basic_auth) => HttpHandler::basic_request(&nodes_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Token(_) => HttpHandler::token_request(&nodes_uuid_url, token.as_str()).await?,
            };
            // Parse the context and the nodes UUIDs.
            let node_uuids = {
                let node_array = node_uuids_json
                    .pointer("/tapi-topology:topology")
                    .ok_or(AppError::validation_error("Node Context Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Node Context Not Found"))?;

                if node_array.len() > 1 {
                    log::warn!("More than one topology found, using the first one.");
                }

                node_array[0]
                    .pointer("/node")
                    .ok_or(AppError::validation_error("Node Context Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Invalid Node"))?
            };

            let mut nodes: Vec<Value> = vec![];
            //Iterate over each node_uuid
            for node_uuid_value in node_uuids {
                let node_uuid = node_uuid_value
                    .get("uuid")
                    .ok_or(AppError::validation_error("Node UUID Not Found"))?
                    .as_str()
                    .ok_or(AppError::validation_error("Invalid str UUID"))?;

                let node_by_uuid_url = format!(
                    "{}/data/tapi-common:context/tapi-topology:topology-context/topology={topology_uuid}/node={node_uuid}",
                    &base_url
                );

                let node_value = match &device.auth {
                    Auth::Basic(basic_auth) => HttpHandler::basic_request(&node_by_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                    Auth::Token(_) => HttpHandler::token_request(&node_by_uuid_url, token.as_str()).await?,
                };

                let node_array = node_value
                    .get("tapi-topology:node")
                    .ok_or(AppError::validation_error("Node Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Invalid Node"))?;

                for node in node_array {
                    nodes.push(node.clone());
                }
            }

            let links_uuid_url = format!(
                "{}/data/tapi-common:context/tapi-topology:topology-context/topology={topology_uuid}?fields=link(uuid)",
                &base_url
            );
            // Obtain the JSON of links uuids based on the authentication method.
            let link_uuids_json = match &device.auth {
                Auth::Basic(basic_auth) => HttpHandler::basic_request(&links_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Token(_) => HttpHandler::token_request(&links_uuid_url, token.as_str()).await?,
            };
            // Parse the context and the links UUIDs.
            let link_uuids = {
                let link_array = link_uuids_json
                    .pointer("/tapi-topology:topology")
                    .ok_or(AppError::validation_error("Link Context Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Link Context Not Found"))?;

                if link_array.len() > 1 {
                    log::warn!("More than one topology found, using the first one.");
                }

                link_array[0]
                    .pointer("/link")
                    .ok_or(AppError::validation_error("Link Context Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Invalid Link"))?
            };

            let mut links: Vec<Value> = vec![];

            //Iterate over each link_uuid
            for link_uuid_value in link_uuids {
                let link_uuid = link_uuid_value
                    .get("uuid")
                    .ok_or(AppError::validation_error("Link UUID Not Found"))?
                    .as_str()
                    .ok_or(AppError::validation_error("Invalid str UUID"))?;

                let link_by_uuid_url = format!(
                    "{}/data/tapi-common:context/tapi-topology:topology-context/topology={topology_uuid}/link={link_uuid}",
                    &base_url
                );

                let link_value = match &device.auth {
                    Auth::Basic(basic_auth) => HttpHandler::basic_request(&link_by_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                    Auth::Token(_) => HttpHandler::token_request(&link_by_uuid_url, token.as_str()).await?,
                };

                let link_array = link_value
                    .get("tapi-topology:link")
                    .ok_or(AppError::validation_error("Link Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Invalid Link"))?;

                for link in link_array {
                    links.push(link.clone());
                }

            }

            let mut connectivity_services: Vec<Value> = vec![];
            let mut connections: Vec<Value> = vec![];

            let connectivity_service_by_uuid_url =format!(
                "{}/data/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service={service_uuid}",
                &base_url
            );

            let connectivity_service_value = match &device.auth {
                Auth::Basic(basic_auth) => HttpHandler::basic_request(&connectivity_service_by_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Token(_) => HttpHandler::token_request(&connectivity_service_by_uuid_url, token.as_str()).await?,
            };

            let connectivity_service_array = connectivity_service_value
                .get("tapi-connectivity:connectivity-service")
                .ok_or(AppError::validation_error("Connectivity Service Not Found"))?
                .as_array()
                .ok_or(AppError::validation_error("Invalid Connectivity Service"))?;

            for connectivity_service in connectivity_service_array {
                connectivity_services.push(connectivity_service.clone());
            }

            let connections_uuids_url =format!(
                "{}/data/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service={service_uuid}?fields=connection(connection-uuid)",
                &base_url
            );

            // Obtain the JSON of connections uuids based on the authentication method.
            let connections_uuids_json = match &device.auth {
                Auth::Basic(basic_auth) => HttpHandler::basic_request(&connections_uuids_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Token(_) => HttpHandler::token_request(&connections_uuids_url, token.as_str()).await?,
            };

            // Parse the context and the connectivity_services UUID.
            let connections_uuids = {
                let connections_array = connections_uuids_json
                    .pointer("/tapi-connectivity:connectivity-service")
                    .ok_or(AppError::validation_error("Connections Context Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Connections Context Not Found"))?;

                if connections_array.len() > 1 {
                    log::warn!("More than one Connections found, using the first one.");
                }

                connections_array[0]
                    .pointer("/connection")
                    .ok_or(AppError::validation_error("Connections Context Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Invalid Connections"))?
            };

            for connection_uuid_value in connections_uuids {
                let connection_uuid = connection_uuid_value
                    .get("connection-uuid")
                    .ok_or(AppError::validation_error("Connection UUID Not Found"))?
                    .as_str()
                    .ok_or(AppError::validation_error("Invalid str UUID"))?;

                let connection_by_uuid_url =format!(
                    "{}/data/tapi-common:context/tapi-connectivity:connectivity-context/connection={connection_uuid}",
                    &base_url
                );

                let connections_value = match &device.auth {
                    Auth::Basic(basic_auth) => HttpHandler::basic_request(&connection_by_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                    Auth::Token(_) => HttpHandler::token_request(&connection_by_uuid_url, token.as_str()).await?,
                };
                let connections_array = connections_value
                    .get("tapi-connectivity:connection")
                    .ok_or(AppError::validation_error("Connection Not Found"))?
                    .as_array()
                    .ok_or(AppError::validation_error("Invalid Connection UUID"))?;
                for connection in connections_array {
                    connections.push(connection.clone());
                }

            }

            // Construct the topology hashmap.
            let mut topology_hashmap: Map<String, Value> = Map::new();
            topology_hashmap.insert(
                "link".to_string(),
                Value::Array(links),
            );
            topology_hashmap.insert(
                "node".to_string(),
                Value::Array(nodes),
            );

            topology_hashmap.extend(topology_fields.clone());

            let topology_object: Value = Value::Object(topology_hashmap);
            let topology: Value = Value::Array(vec![topology_object]);

            Ok(Context {
                connectivity_service: Value::Array(connectivity_services),
                connections,
                topology,
            })

        } .await {
            Ok(context) => Ok(context),
            Err(_) => {
                match async {
                    // Obtain the JSON of topologies based on the authentication method.
                    let topology_uuids_json = match &device.auth {
                        Auth::Basic(basic_auth) => HttpHandler::basic_request(&topology_by_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                        Auth::Token(_) => HttpHandler::token_request(&topology_by_uuid_url, token.as_str()).await?,
                    };
                    // Parse the context and the topology UUID.
                    let topologies = topology_uuids_json
                        .pointer("/tapi-topology:topology-context/topology")
                        .ok_or(AppError::validation_error("Topology Context Not Found"))?
                        .as_array()
                        .ok_or(AppError::validation_error("Invalid Topology"))?;
                    // Verify that there is exactly one topology UUID.
                    if topologies.len() != 1 {
                        return Err(AppError::validation_error("There is more or less than one topology uuid"));
                    }
                    let topology_uuid = topologies[0]
                        .get("uuid")
                        .ok_or(AppError::validation_error("Topology UUID Not Found"))?
                        .as_str()
                        .ok_or(AppError::validation_error("Invalid str UUID"))?;
                    // Construct the URLs for links and nodes.
                    let link_url = format!(
                        "https://{}{}/tapi/data/tapi-common:context/tapi-topology:topology-context/topology={}/link",
                        &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), topology_uuid
                    );
                    let nodes_url = format!(
                        "https://{}{}/tapi/data/tapi-common:context/tapi-topology:topology-context/topology={}/node",
                        &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), topology_uuid
                    );

                    // Retrieve the data for links and nodes.
                    let link_json = match &device.auth {
                        Auth::Basic(basic_auth) => HttpHandler::basic_request(&link_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                        Auth::Token(_) => HttpHandler::token_request(&link_url, token.as_str()).await?,
                    };

                    let node_json = match &device.auth {
                        Auth::Basic(basic_auth) => HttpHandler::basic_request(&nodes_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                        Auth::Token(_) => HttpHandler::token_request(&nodes_url, token.as_str()).await?,
                    };

                    // Construct the topology hashmap.
                    let mut topology_hashmap: Map<String, Value> = Map::new();
                    topology_hashmap.insert(
                        "link".to_string(),
                        link_json.get("tapi-topology:link").ok_or(AppError::validation_error("tapi-topology:link Not Found"))?.clone(),
                    );
                    topology_hashmap.insert(
                        "node".to_string(),
                        node_json.get("tapi-topology:node").ok_or(AppError::validation_error("tapi-topology:node Not Found"))?.clone(),
                    );

                    let topology_object: Value = Value::Object(topology_hashmap);
                    let topology: Value = Value::Array(vec![topology_object]);

                    // Retrieve the connections.
                    let connections = match &device.auth {
                        Auth::Basic(basic_auth) => HttpHandler::basic_request(&connections_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                        Auth::Token(_) => HttpHandler::token_request(&connections_url, token.as_str()).await?,
                    }
                    .as_array()
                    .ok_or(AppError::validation_error("Connections cannot convert into array"))?
                    .clone();

                    // Construct the JSON for services.
                    let service_url = format!(
                        "{}/data/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service={}",
                        &base_url, service_uuid
                    );

                    let service_json = match &device.auth {
                        Auth::Basic(basic_auth) => HttpHandler::basic_request(&service_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                        Auth::Token(_) => HttpHandler::token_request(&service_url, token.as_str()).await?,
                    };

                    let connectivity_service = service_json
                        .get("tapi-connectivity:connectivity-service")
                        .ok_or(AppError::validation_error("Service Data Not Found"))?;


                    // Return the constructed context.
                    Ok(Context {
                        connectivity_service: connectivity_service.clone(),
                        connections,
                        topology,
                    })
                }
                .await
                {
                    // If the `async` block executes successfully, continue normally.
                    Ok(context) => Ok(context),
                    Err(_) => {
                        // If an error occurs, execute the `else` block.
                        let json = match &device.auth {
                            Auth::Basic(basic_auth) => HttpHandler::basic_request(&context_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                            Auth::Token(_) => HttpHandler::token_request(&context_url, token.as_str()).await?,
                        };

                        context_by_context_json(json, service_uuid)
                    }
                }
            }
        }
    }
}

/// Helper function to construct a `Context` from a JSON structure.
///
/// # Arguments
/// - `json`: The JSON `Value` containing service, connections, and topology data.
/// - `service_uuid`: The UUID of the service to construct the context for.
///
/// # Returns
/// A `Result` containing a `Context` object or an `Error`.
fn context_by_context_json(json: Value, service_uuid: &str) -> Result<Context, AppError> {
    let connectivity_services = json
        .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")
        .and_then(Value::as_array)
        .ok_or_else(|| AppError::validation_error("Cannot find connectivity-context"))?
        .clone();

    let connectivity_service = connectivity_services
        .iter()
        .find(|service| {
            service
                .get("uuid")
                .and_then(|uuid| uuid.as_str())
                .map(|uuid_str| uuid_str == service_uuid)
                .unwrap_or(false)
        })
        .ok_or_else(|| AppError::validation_error("There is not any Service with that id"))?
        .clone();

    let connections = json
        .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connection")
        .and_then(Value::as_array)
        .ok_or_else(|| AppError::validation_error("Cannot find connections-context"))?
        .clone();
    let topology = json
        .pointer("/tapi-common:context/tapi-topology:topology-context/topology")
        .ok_or_else(|| AppError::validation_error("Cannot find connections-context"))?
        .clone();

    Ok(Context {
        connectivity_service,
        connections,
        topology,
    })
}

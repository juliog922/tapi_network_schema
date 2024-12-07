use super::files_model::FilesEnum;
use super::devices::{Device, Auth};
use serde_json::{Value, Map};
use reqwest::{Client, RequestBuilder};
use super::error::Error;
use std::io::BufReader;
use std::fs::File;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataSource {
    Device(Device),
    FilesEnum(FilesEnum),
}

pub struct Context {
    pub connectivity_service: Value,
    pub connections: Vec<Value>,
    pub topology: Value,
}

pub struct Requester;

impl Requester {
    pub async fn get_services(data_source: &DataSource) -> Result<Vec<Value>, Error> {
        match data_source {
            DataSource::Device(device) => DeviceHandler::get_services(device).await,
            DataSource::FilesEnum(file_enum) => FilesHandler::get_services(file_enum),
        }
    }

    pub async fn get_service_context(data_source: &DataSource, service_uuid: &String) -> Result<Context, Error> {
        match data_source {
            DataSource::Device(device) => DeviceHandler::get_service_context(device, service_uuid).await,
            DataSource::FilesEnum(file_enum) => FilesHandler::get_service_context(file_enum, service_uuid),
        }
    }
}

pub struct FilesHandler;

impl FilesHandler {
    pub fn get_services(file_enum: &FilesEnum) -> Result<Vec<Value>, Error> {
        match file_enum {
            FilesEnum::ByPart(by_part_paths) => {
                let connectivity_services_file = File::open(format!("{}", &by_part_paths.connectivity_services_path))
                                                            .map_err(|err| Error::from(format!("File cannot be opened: {}", err).as_str()))?;
                let connectivity_services_reader = BufReader::new(connectivity_services_file);
                let connectivity_services_value: Value = serde_json::from_reader(connectivity_services_reader)
                                                            .map_err(|err| Error::from(format!("File cannot be readed: {}", err).as_str()))?;
                let connectivity_services = connectivity_services_value.as_array().cloned().unwrap_or_default();

                return Ok(connectivity_services);
            },
            FilesEnum::Complete(complete_path) => {
                let file = File::open(format!("{}", &complete_path.complete_context_path))
                                    .map_err(|err| Error::from(format!("File cannot be opened: {}", err).as_str()))?;
                let reader = BufReader::new(file);
                let json_value: Value = serde_json::from_reader(reader)
                                            .map_err(|err| Error::from(format!("File cannot be readed: {}", err).as_str()))?;
                let connectivity_services = json_value
                                                            .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")
                                                            .and_then(&Value::as_array)
                                                            .ok_or_else(|| Error::from("Cannot find connectivity-context"))?
                                                            .clone();
                return Ok(connectivity_services);
            },
        }
    }

    pub fn get_service_context(
        file_enum: &FilesEnum,
        service_uuid: &String
    ) -> Result<Context, Error> {
        match file_enum {
            FilesEnum::ByPart(by_part_paths) => {
                let topology_file = File::open(format!("{}", &by_part_paths.topology_path))
                                            .map_err(|err| Error::from(format!("File cannot be opened: {}", err).as_str()))?;
                let topology_reader = BufReader::new(topology_file);
                let topology: Value = serde_json::from_reader(topology_reader)
                                    .map_err(|err| Error::from(format!("File cannot be readed: {}", err).as_str()))?;

                let connections_file = File::open(format!("{}", &by_part_paths.connections_path))
                                                .map_err(|err| Error::from(format!("File cannot be opened: {}", err).as_str()))?;
                let connections_reader = BufReader::new(connections_file);
                let connections_value: Value = serde_json::from_reader(connections_reader)
                                                    .map_err(|err| Error::from(format!("File cannot be readed: {}", err).as_str()))?;
                let connections = connections_value.as_array().cloned().unwrap_or_default();

                let connectivity_services_file = File::open(format!("{}", &by_part_paths.connectivity_services_path))
                                                            .map_err(|err| Error::from(format!("File cannot be opened: {}", err).as_str()))?;
                let connectivity_services_reader = BufReader::new(connectivity_services_file);
                let connectivity_services_value: Value = serde_json::from_reader(connectivity_services_reader)
                                                            .map_err(|err| Error::from(format!("File cannot be readed: {}", err).as_str()))?;
                let connectivity_services = connectivity_services_value.as_array().cloned().unwrap_or_default();

                let connectivity_service = connectivity_services.iter().find(|service| {
                        service.get("uuid").and_then(|uuid| uuid.as_str()).map(|uuid_str| uuid_str == service_uuid.as_str()).unwrap_or(false)
                    }).ok_or_else(|| Error::from("There is not any Service with that id"))?.clone();

                return Ok(Context {
                    connectivity_service: connectivity_service,
                    connections: connections,
                    topology: topology,
                });
            },
            FilesEnum::Complete(complete_path) => {
                let file = File::open(format!("{}", &complete_path.complete_context_path))
                                    .map_err(|err| Error::from(format!("File cannot be opened: {}", err).as_str()))?;
                let reader = BufReader::new(file);
                let json_value: Value = serde_json::from_reader(reader)
                                            .map_err(|err| Error::from(format!("File cannot be readed: {}", err).as_str()))?;

                return context_by_context_json(json_value, service_uuid);

            },
        }
    }
}

pub struct DeviceHandler;

impl DeviceHandler {

    pub async fn get_services(device: &Device) -> Result<Vec<Value>, Error> {
        
        let services_url = format!(
            "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context?fields=connectivity-service(uuid)",
            &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default()
        );

        let context_url = format!(
            "https://{}{}/restconf/data/tapi-common:context",
             &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default()
        );

        let mut token_oauth: String = String::default();
        let mut token_custom: String = String::default();

        match &device.auth {
            Auth::Oauth2(oauth) => {
                let url = format!("https://{}{}{}", &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), &oauth.auth_sufix);
                let mut json = HashMap::new();
                json.insert("username", oauth.username.clone());
                json.insert("password", oauth.password.clone());
                json.insert("grant_type", oauth.grant_type.clone());

                let response = Self::custom_post_request(&url, &json).await?;
                token_oauth = String::from(response.get("token").ok_or_else(|| Error::from("Cannot find Token in oauth2 response"))?.as_str().unwrap());
            },
            Auth::Custom(custom_auth) => {
                let url = format!("https://{}{}{}", &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), &custom_auth.auth_sufix);

                let auth_body = custom_auth.auth_body.as_object().ok_or_else(|| Error::from("Auth body its not a Hashmap"))?;
                let mut json = HashMap::new();
                for (key, value) in auth_body {
                    json.insert(key.as_str(), String::from(value.as_str().unwrap()));
                }

                let response = Self::custom_post_request(&url, &json).await?;
                token_custom = String::from(response.get("token").ok_or_else(|| Error::from("Cannot find Token in oauth2 response"))?.as_str().unwrap());
            }
            _ => {}
        }

        if let Ok(services_uuid_json) = match &device.auth {
            Auth::BasicAuth(basic_auth) => Self::basic_request(&services_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await,
            Auth::Oauth2(_) => {

                Self::token_request(&services_url, token_oauth.as_str()).await
            },
            Auth::Custom(_) => {

                Self::token_request(&services_url, token_custom.as_str()).await
            }
        } {
            let connectivity_services = services_uuid_json
                        .pointer("/tapi-connectivity:connectivity-context/connectivity-service")
                        .ok_or(Error::from("Connectivity Context Not Found"))?
                        .as_array()
                        .ok_or(Error::from("Invalid Connectivity Service"))?
                        .clone();

            let mut services_vector: Vec<Value> = vec![];
            for service in connectivity_services {
                let service_uuid = service
                    .get("uuid")
                    .ok_or(Error::from("UUID Not Found"))?
                    .as_str()
                    .ok_or(Error::from("Invalid UUID"))?;
        
                let service_url = format!(
                    "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service={}",
                    &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), service_uuid
                );
                let service_json = match &device.auth {
                    Auth::BasicAuth(basic_auth) => Self::basic_request(&service_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                    Auth::Oauth2(_) => Self::token_request(&service_url, token_oauth.as_str()).await?,
                    Auth::Custom(_) => Self::token_request(&service_url, token_custom.as_str()).await?,
                };
                let service_data = &service_json
                            .get("tapi-connectivity:connectivity-service")
                            .ok_or(Error::from("Service Data Not Found"))?
                            .as_array().unwrap()[0];
                services_vector.push(service_data.clone()); // Add the service data to the vector.
            }
            //println!("{:?}", services_vector);
            return Ok(services_vector);
        } else {
            let json = match &device.auth {
                Auth::BasicAuth(basic_auth) => Self::basic_request(&context_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Oauth2(_) => Self::token_request(&context_url, token_oauth.as_str()).await?,
                Auth::Custom(_) => Self::token_request(&context_url, token_custom.as_str()).await?,
            };
            //println!("{}", json);
            let connectivity_services = json
                                                        .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")
                                                        .and_then(&Value::as_array)
                                                        .ok_or_else(|| Error::from("Cannot find connectivity-context"))?
                                                        .clone();

            return Ok(connectivity_services);
        }     
    }

    fn client_get_builder(url: &String) -> RequestBuilder {
        Client::builder()
            .danger_accept_invalid_certs(true) // Accept invalid certificates.
            .gzip(true) // Enable gzip encoding.
            .brotli(true) // Enable brotli encoding.
            .deflate(true) // Enable deflate encoding.
            .build()
            .unwrap() // Handle client build error.
            .get(url) // Set up the GET request.
            .header("Accept", "application/yang-data+json")
            .header("Accept-Encoding", "gzip, deflate, br")
    }

    async fn custom_post_request(url: &String, json: &HashMap<&str, String>) -> Result<Value, Error> {
        Client::builder()
            .danger_accept_invalid_certs(true) // Accept invalid certificates.
            .build()
            .unwrap() // Handle client build error.
            .post(url) // Set up the GET request.
            .json(&json)
            .send()
            .await
            .map_err(|_| Error::from("Request Error"))?
            .json()
            .await
            .map_err(|_| Error::from("Json Error"))?
    }

    async fn basic_request(url: &String, username: String, password: Option<String>) -> Result<Value, Error> {
        Self::client_get_builder(url).basic_auth(username, password).send().await
            .map_err(|err| Error::from(format!("Error on request {}", err).as_str()))?
            .json::<Value>().await
            .map_err(|err| Error::from(format!("Error transforming request in Json {}", err).as_str()))
            
    }

    async fn token_request(url: &String, token: &str) -> Result<Value, Error> {
        Self::client_get_builder(url).bearer_auth(token).send().await
            .map_err(|err| Error::from(format!("Error on request {}", err).as_str()))?
            .json::<Value>().await.map_err(|err| Error::from(format!("Error transforming request in Json {}", err).as_str()))
    }

    pub async fn get_service_context(device: &Device, service_uuid: &String) -> Result<Context, Error> {
        let topology_by_uuid_url =format!(
            "https://{}{}/restconf/data/tapi-common:context/tapi-topology:topology-context?fields=topology(uuid)",
            &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default()
        );

        let connections_url = format!(
            "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context",
            &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default()
        );

        //let services_url = format!(
        //    "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context?fields=connectivity-service(uuid)",
        //    &device.ip, &device.port.unwrap_or_default()
        //);

        let context_url = format!(
            "https://{}{}/restconf/data/tapi-common:context",
             &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default()
        );

        let mut token_oauth: String = String::default();
        let mut token_custom: String = String::default();

        match &device.auth {
            Auth::Oauth2(oauth) => {
                let url = format!("https://{}{}{}", &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), &oauth.auth_sufix);
                let mut json = HashMap::new();
                json.insert("username", oauth.username.clone());
                json.insert("password", oauth.password.clone());
                json.insert("grant_type", oauth.grant_type.clone());

                let response = Self::custom_post_request(&url, &json).await?;
                token_oauth = String::from(response.get("token").ok_or_else(|| Error::from("Cannot find Token in oauth2 response"))?.as_str().unwrap());
            },
            Auth::Custom(custom_auth) => {
                let url = format!("https://{}{}{}", &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), &custom_auth.auth_sufix);

                let auth_body = custom_auth.auth_body.as_object().ok_or_else(|| Error::from("Auth body its not a Hashmap"))?;
                let mut json = HashMap::new();
                for (key, value) in auth_body {
                    json.insert(key.as_str(), String::from(value.as_str().unwrap()));
                }

                let response = Self::custom_post_request(&url, &json).await?;
                token_custom = String::from(response.get("token").ok_or_else(|| Error::from("Cannot find Token in oauth2 response"))?.as_str().unwrap());
            }
            _ => {}
        }

        match async {
            // Obtener el JSON de topologías según el método de autenticación.
            let topology_uuids_json = match &device.auth {
                Auth::BasicAuth(basic_auth) => Self::basic_request(&topology_by_uuid_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Custom(_) => Self::token_request(&topology_by_uuid_url, token_custom.as_str()).await?,
                Auth::Oauth2(_) => Self::token_request(&topology_by_uuid_url, token_oauth.as_str()).await?,
            };
        
            // Parsear el contexto y el UUID de topología.
            let topologies = topology_uuids_json
                .pointer("/tapi-topology:topology-context/topology")
                .ok_or(Error::from("Topology Context Not Found"))?
                .as_array()
                .ok_or(Error::from("Invalid Topology"))?;
        
            // Verificar que haya exactamente una topología UUID.
            if topologies.len() != 1 {
                return Err(Error::from("There is more or less than one topology uuid"));
            }
        
            let topology_uuid = topologies[0]
                .get("uuid")
                .ok_or(Error::from("Topology UUID Not Found"))?
                .as_str()
                .ok_or(Error::from("Invalid str UUID"))?;
        
            // Construir las URLs de enlace y nodos.
            let link_url = format!(
                "https://{}{}/tapi/data/tapi-common:context/tapi-topology:topology-context/topology={}/link",
                &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), topology_uuid
            );
            let nodes_url = format!(
                "https://{}{}/tapi/data/tapi-common:context/tapi-topology:topology-context/topology={}/node",
                &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), topology_uuid
            );
        
            // Obtener los datos de enlace y nodos.
            let link_json = match &device.auth {
                Auth::BasicAuth(basic_auth) => Self::basic_request(&link_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Custom(_) => Self::token_request(&link_url, token_custom.as_str()).await?,
                Auth::Oauth2(_) => Self::token_request(&link_url, token_oauth.as_str()).await?,
            };
        
            let node_json = match &device.auth {
                Auth::BasicAuth(basic_auth) => Self::basic_request(&nodes_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Custom(_) => Self::token_request(&nodes_url, token_custom.as_str()).await?,
                Auth::Oauth2(_) => Self::token_request(&nodes_url, token_oauth.as_str()).await?,
            };
        
            // Construir el hashmap de topología.
            let mut topology_hashmap: Map<String, Value> = Map::new();
            topology_hashmap.insert(
                "link".to_string(),
                link_json.get("tapi-topology:link").ok_or(Error::from("tapi-topology:link Not Found"))?.clone(),
            );
            topology_hashmap.insert(
                "node".to_string(),
                node_json.get("tapi-topology:node").ok_or(Error::from("tapi-topology:node Not Found"))?.clone(),
            );
        
            let topology_object: Value = Value::Object(topology_hashmap);
            let topology: Value = Value::Array(vec![topology_object]);
        
            // Obtener las conexiones.
            let connections = match &device.auth {
                Auth::BasicAuth(basic_auth) => Self::basic_request(&connections_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Custom(_) => Self::token_request(&connections_url, token_custom.as_str()).await?,
                Auth::Oauth2(_) => Self::token_request(&connections_url, token_oauth.as_str()).await?,
            }
            .as_array()
            .ok_or(Error::from("Connections cannot convert into array"))?
            .clone();
        
            // Construir el JSON de servicios.
            let service_url = format!(
                "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service={}",
                &device.ip, &device.port.map(|s| format!(":{}", s)).unwrap_or_default(), service_uuid
            );
        
            let service_json = match &device.auth {
                Auth::BasicAuth(basic_auth) => Self::basic_request(&service_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                Auth::Custom(_) => Self::token_request(&service_url, token_custom.as_str()).await?,
                Auth::Oauth2(_) => Self::token_request(&service_url, token_oauth.as_str()).await?,
            };
        
            let connectivity_service = &service_json
                .get("tapi-connectivity:connectivity-service")
                .ok_or(Error::from("Service Data Not Found"))?
                .as_array()
                .unwrap()[0];

        
            // Devolver el contexto construido.
            Ok(Context {
                connections,
                connectivity_service: connectivity_service.clone(),
                topology,
            })
        }
        .await
        {
            // Si el bloque `async` se ejecuta sin errores, continuamos normalmente.
            Ok(context) => return Ok(context),
            Err(err) => {
                println!("{:?}", err);
                // Si ocurre un error, ejecutamos el bloque `else`.
                let json = match &device.auth {
                    Auth::BasicAuth(basic_auth) => Self::basic_request(&context_url, basic_auth.username.clone(), Some(basic_auth.password.clone())).await?,
                    Auth::Custom(_) => Self::token_request(&context_url, token_custom.as_str()).await?,
                    Auth::Oauth2(_) => Self::token_request(&context_url, token_oauth.as_str()).await?,
                };
        
                return context_by_context_json(json, service_uuid);
            }
        }
        
    }
}

fn context_by_context_json(json: Value, service_uuid: &String) -> Result<Context, Error> {
    let connectivity_services = json
                                                .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")
                                                .and_then(&Value::as_array)
                                                .ok_or_else(|| Error::from("Cannot find connectivity-context"))?
                                                .clone();
    let connectivity_service = connectivity_services.iter().find(|service| {
            service.get("uuid").and_then(|uuid| uuid.as_str()).map(|uuid_str| uuid_str == service_uuid.as_str()).unwrap_or(false)
    }).ok_or_else(|| Error::from("There is not any Service with that id"))?.clone();

    let connections = json
                                    .pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connection")
                                    .and_then(&Value::as_array)
                                    .ok_or_else(|| Error::from("Cannot find connections-context"))?
                                    .clone();
    let topology = json
                            .pointer("/tapi-common:context/tapi-topology:topology-context/topology")
                            .ok_or_else(|| Error::from("Cannot find connections-context"))?
                            .clone();

    return  Ok(Context{
        connectivity_service: connectivity_service, 
        connections: connections,
        topology: topology,
    });
}


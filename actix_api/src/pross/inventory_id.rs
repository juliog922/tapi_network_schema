use actix_web::{Error, error};
use serde_json::{Value, json, Map};

pub fn inventory_creation(schema: &mut Value) -> Result<Value, Error> {

    let mut new_services_array:  Vec<Value> = vec![];
    if let Some(connectivity_services) = schema.as_object_mut().unwrap().get_mut(&"connectivity_services".to_string()) {

        // Retrieve the array of services.
        let services_array: &mut Vec<Value> = connectivity_services.as_array_mut().unwrap();

        // Iterate over each service in the array.
        for service in services_array {

            let mut new_service_object = Map::new();
            new_service_object.insert(
                "uuid".to_string(), 
                service.get("uuid").unwrap().clone()
            );
            new_service_object.insert(
                "value_name".to_string(), 
                service.get("value_name").unwrap().clone()
            );

            // Check if the service contains the "nodes" key.
            if let Some(nodes) = service.as_object_mut().unwrap().get_mut(&"nodes".to_string()) {

                // Retrieve the array of nodes.
                let nodes_array: &mut Vec<Value> = nodes.as_array_mut().unwrap();

                let mut new_nodes_array:  Vec<Value> = vec![];

                // Iterate over each node.
                for node in nodes_array {

                    let mut new_node_object = Map::new();

                    new_node_object.insert(
                        "node_uuid".to_string(), 
                        node.get("node_uuid").unwrap().clone()
                    );

                    new_node_object.insert(
                        "node_id".to_string(), 
                        node.get("node_id").unwrap().clone()
                    );

                    // Check if the node contains the "end_points" key.
                    if let Some(endpoints) = node.as_object_mut().unwrap().get_mut(&"end_points".to_string()) {

                        // Retrieve the array of endpoints.
                        let endpoints_array: &mut Vec<Value> = endpoints.as_array_mut().unwrap();

                        let mut new_inventories_array: Vec<Value> = vec![];
                        
                        for endpoint in endpoints_array {
                            // Obtener el inventory_id del endpoint actual
                            let inventory_id = endpoint["inventory_id"].clone();
                            
                            // Verificar si ya existe un objeto con este inventory_id en new_inventories_array
                            let mut inventory_exists = false;
                            
                            for inventory_obj in &mut new_inventories_array {
                                // Si encontramos un objeto con el mismo inventory_id
                                if inventory_obj["inventory_id"] == inventory_id {
                                    // Agregar este endpoint a la lista de endpoints
                                    inventory_obj["endpoints"].as_array_mut().unwrap().push(endpoint.clone());
                                    inventory_exists = true;
                                    break;
                                }
                            }
                        
                            // Si no existe un objeto con este inventory_id, creamos uno nuevo
                            if !inventory_exists {
                                let new_inventory_obj = json!({
                                    "inventory_id": inventory_id,
                                    "endpoints": vec![endpoint.clone()] // Crear una nueva lista con este endpoint
                                });
                                
                                // Añadir el nuevo objeto a la lista de new_inventories_array
                                new_inventories_array.push(new_inventory_obj);
                            }
                        }

                        new_node_object.insert(
                            "inventories".to_string(), 
                            Value::Array(new_inventories_array)
                        );

                    } else {
                        return Err(error::ErrorNotFound("Cannot find end_points on nodes"));
                    }

                    new_nodes_array.push(Value::Object(new_node_object));
                }

                new_service_object.insert(
                    "nodes".to_string(), 
                    Value::Array(new_nodes_array)
                );

            } else {
                return Err(error::ErrorNotFound("Cannot find nodes on connectivity_services"));
            }

            new_services_array.push(Value::Object(new_service_object));
        }

    } else {
        return Err(error::ErrorNotFound("Cannot find connectivity_services on schema"));
    }

    Ok(Value::Array(new_services_array))
}
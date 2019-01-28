use serde_json;
use inventory_api::RESTApi;
use minv_config;

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct System {
    id: u32,
    pub hostname: String,
    serial: String,
    #[serde(default)]
    server_model_id: String,
    #[serde(default)]
    server_model_name: String,
    #[serde(default)]
    asset_tag: String,
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id={} hostname={}", self.id, self.hostname)
    }
}

impl Default for System {
    fn default() -> System {
        System { 
            id: 0,
            hostname: String::new(),
            serial: String::new(),
            asset_tag: String::new(),
            server_model_id: String::new(),
            server_model_name: String::new(),
        }
    }
}

const ENDPOINT: &'static str = "systems";
pub fn execute(host_matches: &clap::ArgMatches, config: minv_config::Config){
    if let Some(_get_matches) = host_matches.subcommand_matches("get") {
        match _get_matches.value_of("hostname"){
            Some(value) => { 
                let hostname_search = &format!("{}/{}", ENDPOINT, value);
                get_system(&hostname_search, false, config.clone());
            },
            None => println!("Hostname Required")
        }
    }
    if let Some(_get_matches) = host_matches.subcommand_matches("delete") {
        let mut s = System{ ..Default::default() };
        match _get_matches.value_of("hostname"){
            Some(_value) => { 
                s.hostname = _value.to_string();
                delete_system(s, config.clone());
            },
            None => println!("Hostname Required")
        }
    }
    if let Some(_get_matches) = host_matches.subcommand_matches("update") {
        match _get_matches.value_of("hostname"){
            Some(_value) => { 
                let hostname_search = &format!("{}/{}", ENDPOINT, _value);
                match get_system(&hostname_search, true, config.clone()) {
                    Some(mut _s) => { 
                        match _get_matches.value_of("serial"){
                            Some(_value) => { 
                                _s.serial = _value.to_string();
                            },
                            None => { 
                                // Possibly check here for error?
                            }
                        }
                        update_system(_s, config.clone());
                    },
                    None => { println!("No System Found")}
                }
                },
            None => println!("Hostname Required")
        }

    }
    if let Some(_get_matches) = host_matches.subcommand_matches("create") {
        let mut s = System{ ..Default::default() };
        match _get_matches.value_of("hostname"){
            Some(_value) => { 
                s.hostname = _value.to_string();
            },
            None => println!("Hostname Required")
        }
        match _get_matches.value_of("serial"){
            Some(_value) => { 
                s.serial = _value.to_string();
            },
            None => { 
                // Possibly check here for error?
            }
        }
        match _get_matches.value_of("asset-tag"){
            Some(_value) => {
                s.asset_tag = _value.to_string();
            },
            None => {
                // Possibly check here for error?
            }
        }
        match _get_matches.value_of("server-model-id"){
            Some(_value) => { 
                s.server_model_id = _value.to_string();
            },
            None => { 
                // Possibly check here for error?
            }
        }
        match _get_matches.value_of("server-model-name"){
            Some(_value) => { 
                s.server_model_name = _value.to_string();
            },
            None => { 
                // Possibly check here for error?
            }
        }
        create_system(s, config.clone());
    }
}

fn get_system(search: &str, should_return: bool, config: minv_config::Config) -> Option<System> {
    let token = config.clone().token;
    let r = RESTApi {
        config: config
    };
    match r.get(search.to_string(), token) {
        Some(value) => {
            match serde_json::from_value(value) {
                Ok(_value) => {
                    let s: System = _value;
                    if should_return {
                        return Some(s);
                    } else {
                        println!("{}", s)
                    }
                },
                Err(_e) => { println!("No Results")}
            }
        },
        None => { println!("No Results") }
    }
    None
}
fn update_system(system: System, config: minv_config::Config) {
    let r = RESTApi {
        config: config
    };
    let json_data = serde_json::to_value(&system).unwrap();
    let url = format!("{}/{}", ENDPOINT, system.hostname);
    match r.update(url, json_data, &r.config.token) {
        Some(mut _value) => {
            let s: System = _value.json().unwrap();
            println!("{}", s);
        },
        None => {}
    }
}
fn create_system(system: System, config: minv_config::Config) {
    let r = RESTApi {
        config: config
    };
    let json_data = serde_json::to_value(&system).unwrap();
    match r.create(ENDPOINT.to_string(), json_data, &r.config.token) {
        Some(mut _value) => {
            let s: System = _value.json().unwrap();
            println!("{}", s);
        },
        None => {}
    }
}
fn delete_system(system: System, config: minv_config::Config) {
    let r = RESTApi {
        config: config
    };
    let url = format!("{}/{}", ENDPOINT, system.hostname);
    match r.delete(url, &system, &r.config.token) {
        Some(mut _value) => {
            println!("{} deleted", &system.hostname);
        },
        None => {}
    }
    
    /*

    if api_out.count == 0 {
        println!("Error: {} not found.", search);
    } else {
        let entries = serialize_entries(api_out.response);
        for entry in entries {
            println!("{}", entry);
        }
    }
    */
}
/*
will need this for search implementation down the road
fn serialize_entries(entries: Vec<Value>) -> Vec<System> {
    let entries: Vec<Value> = entries;
    let mut return_systems = vec![];
    for entry in entries {
        let system: System = serde_json::from_value(entry).unwrap();
        return_systems.push(system);
    }
    return_systems

}
*/
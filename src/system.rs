use serde_json;
use serde_json::Value;
use inventory_api::InventoryResponse;
use inventory_api::RESTApi;
use minv_config;
#[derive(Deserialize,Serialize)]
#[allow(dead_code)]
#[derive(Clone)]
struct System {
    id: u32,
    hostname: String,
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
        }
    }
}

const ENDPOINT: &'static str = "systems";
pub fn execute(host_matches: &clap::ArgMatches, config: minv_config::Config){
    if let Some(_get_matches) = host_matches.subcommand_matches("get") {
        match _get_matches.value_of("hostname"){
            Some(value) => { 
                let hostname_search = &format!("{}/{}", ENDPOINT, value);
                get_system(&hostname_search, config.clone());
            },
            None => println!("Hostname Required")
        }
    }
    if let Some(_get_matches) = host_matches.subcommand_matches("create") {
        let hostname="";
        let mut s = System{ ..Default::default() };
        match _get_matches.value_of("hostname"){
            Some(_value) => { 
                s.hostname = _value.to_string();
            },
            None => println!("Hostname Required")
        }
        create_system(s, config.clone());
    }
}

fn get_system(search: &str, config: minv_config::Config) {
    let token = config.clone().token;
    let r = RESTApi {
        config: config
    };
    match r.get(search.to_string(), token) {
        Some(value) => {
            match serde_json::from_value(value) {
                Ok(_value) => {
                    let s: System = _value;
                    println!("{}", s)

                },
                Err(_e) => { println!("No Results")}
            }
        },
        None => { println!("No Results") }
    }
}
fn create_system(system: System, config: minv_config::Config) {
    let r = RESTApi {
        config: config
    };
    let json_data = serde_json::to_value(&system).unwrap();
    match r.create(ENDPOINT.to_string(), json_data, &r.config.token) {
        Some(value) => {
            println!("{:?} YAY", value);
            //let s: System = serde_json::from_value(value).unwrap();
            //println!("{}", s)
        },
        None => { println!("No Results") }
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
fn serialize_entries(entries: Vec<Value>) -> Vec<System> {
    let entries: Vec<Value> = entries;
    let mut return_systems = vec![];
    for entry in entries {
        let system: System = serde_json::from_value(entry).unwrap();
        return_systems.push(system);
    }
    return_systems

}
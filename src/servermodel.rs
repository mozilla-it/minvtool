use serde_json;
use inventory_api::RESTApi;
use minv_config;

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct ServerModel {
    id: u32,
    #[serde(default)]
    vendor: String,
    #[serde(default)]
    model: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    part_number: Option<String>,
}

impl std::fmt::Display for ServerModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id={} vendor={} model={}", self.id, self.vendor, self.model)
    }
}

impl Default for ServerModel {
    fn default() -> ServerModel {
        ServerModel { 
            id: 0,
            vendor: String::new(),
            model: String::new(),
            part_number: Some(String::new()),
            description: Some(String::new()),
        }
    }
}

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct ServerModelSearchResponse {
    pub count: usize,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub results: Vec<ServerModel>
}

const ENDPOINT: &'static str = "servermodel";
pub fn execute(host_matches: &clap::ArgMatches, config: minv_config::Config){
    if let Some(_get_matches) = host_matches.subcommand_matches("search") {
        match _get_matches.value_of("search"){
            Some(value) => { 
                let search_string = &format!("{}/?search={}", ENDPOINT, value);
                search(&search_string, config.clone());
            },
            None => println!("Search Required")
        }
    }
}

fn search(search: &str, config: minv_config::Config) -> Option<ServerModel> {
    let token = config.clone().token;
    let r = RESTApi {
        config: config
    };
    match r.get(search.to_string(), token) {
        Some(value) => {
            match serde_json::from_value(value) {
                Ok(_value) => {
                    let s: ServerModelSearchResponse = _value;
                    if s.results.len() > 0 {
                        for sm in s.results {
                            println!("{}", sm);
                        }
                    } else {
                        println!("No Results");
                    }
                },
                Err(_e) => { println!("No Results")}
            }
        },
        None => { println!("No Results") }
    }
    None
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
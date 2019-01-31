use serde_json;
use inventory_api::RESTApi;
use minv_config;

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct SystemType {
    id: u32,
    #[serde(default)]
    type_name: String,
}

impl std::fmt::Display for SystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id={} name={}", self.id, self.type_name)
    }
}

impl Default for SystemType {
    fn default() -> SystemType {
        SystemType { 
            id: 0,
            type_name: String::new(),
        }
    }
}

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct SystemTypeSearchResponse {
    pub count: usize,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub results: Vec<SystemType>
}

const ENDPOINT: &'static str = "systemtype";
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
    if let Some(_get_matches) = host_matches.subcommand_matches("list") {
        let search_string = format!("{}/", ENDPOINT);
        search(&search_string, config.clone());
    }
}

fn search(search: &str, config: minv_config::Config) -> Option<SystemType> {
    let token = config.clone().token;
    let r = RESTApi {
        config: config
    };
    match r.get(search.to_string(), token) {
        Some(value) => {
            match serde_json::from_value(value) {
                Ok(_value) => {
                    let s: SystemTypeSearchResponse = _value;
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
        None => { println!("No Resuls") }
    }
    None
}
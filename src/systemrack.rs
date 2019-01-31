use serde_json;
use inventory_api::RESTApi;
use minv_config;

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct SystemRack {
    id: u32,
    #[serde(default)]
    name: String,
    #[serde(default)]
    location: String,
    #[serde(default)]
    site: String,
}

impl std::fmt::Display for SystemRack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id={} name={} location={} site={}", self.id, self.name, self.location, self.site)
    }
}

impl Default for SystemRack {
    fn default() -> SystemRack {
        SystemRack { 
            id: 0,
            name: String::new(),
            location: String::new(),
            site: String::new(),
        }
    }
}

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct SystemRackSearchResponse {
    pub count: usize,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub results: Vec<SystemRack>
}

const ENDPOINT: &'static str = "systemrack";
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

fn search(search: &str, config: minv_config::Config) -> Option<SystemRack> {
    let token = config.clone().token;
    let r = RESTApi {
        config: config
    };
    match r.get(search.to_string(), token) {
        Some(value) => {
            match serde_json::from_value(value) {
                Ok(_value) => {
                    let s: SystemRackSearchResponse = _value;
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
use serde_json;
use inventory_api::RESTApi;
use minv_config;
use system::System;
use prettytable::{Table};

#[derive(Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct SystemRack {
    id: u32,
    #[serde(default)]
    name: String,
    #[serde(default)]
    site: String,
    #[serde(default)]
    systems: Vec<System>,

}

impl std::fmt::Display for SystemRack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id={} name={} site={}", self.id, self.name, self.site)
    }
}

impl Default for SystemRack {
    fn default() -> SystemRack {
        SystemRack { 
            id: 0,
            name: String::new(),
            site: String::new(),
            systems: vec![]
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
    if let Some(_get_matches) = host_matches.subcommand_matches("get") {
        match _get_matches.value_of("rack_id"){
            Some(value) => { 
                let search_string = &format!("{}/{}", ENDPOINT, value);
                get(&search_string, false, _get_matches.is_present("tabular"), config.clone());
            },
            None => println!("Search Required")
        }
    }
    if let Some(_get_matches) = host_matches.subcommand_matches("create") {
        let mut name = "";
        let mut site_name = "";
        match _get_matches.value_of("name"){
            Some(value) => { 
                name = value;
            },
            None => println!("name Required")
        }
        match _get_matches.value_of("site"){
            Some(value) => { 
                site_name = value;
            },
            None => println!("site Required")
        }
        let sr = SystemRack {
            id: 0,
            name: name.to_string(),
            site: site_name.to_string(),
            systems: vec![]
        };
        create(sr, config.clone());
    }
}


fn create(sr: SystemRack, config: minv_config::Config) {
    let r = RESTApi {
        config: config
    };
    let json_data = serde_json::to_value(&sr).unwrap();
    match r.create(ENDPOINT.to_string(), json_data, &r.config.token) {
        Some(mut _value) => {
            let s: SystemRack = _value.json().unwrap();
            println!("{}", s);
        },
        None => {}
    }
}
fn get(search: &str, should_return: bool, tabular: bool, config: minv_config::Config) -> Option<SystemRack> {
    let token = config.clone().token;
    let r = RESTApi {
        config: config
    };
    let final_search = format!("{}/", search);
    match r.get(final_search.to_string(), token) {
        Some(value) => {
            match serde_json::from_value(value) {
                Ok(_value) => {
                    let s: SystemRack = _value;
                    if should_return {
                        return Some(s);
                    } else {
                        if tabular {
                            let mut headertable = Table::new();
                            headertable.add_row(row![Fg => "ID", "Name ", "Site"]);
                            headertable.add_row(row![s.id.to_string(), s.name, s.site]);
                            headertable.printstd();
                        } else {
                            println!("id={} name={} site={}", s.id, s.name, s.site);
                        }
                        if s.systems.len() > 0 {
                            if tabular {
                                let mut table = Table::new();
                                table.add_row(row![Fg => "Rack Order", "Hostname ", "Asset"]);
                                for system in s.systems {
                                    let mut asset_tag = String::new();
                                    let mut rack_order = "0.0".to_string();
                                    match system.asset_tag {
                                        Some(_s) => { asset_tag = _s.to_string() },
                                        None => { }
                                    }
                                    match system.rack_order {
                                        Some(_s) => { rack_order = _s.to_string() },
                                        None => { }
                                    }
                                    table.add_row(row![rack_order, system.hostname, asset_tag]);
                                }
                                table.printstd();
                            } else {
                                for system in s.systems {
                                    let mut asset_tag = String::new();
                                    let mut rack_order = "0.0".to_string();
                                    match system.asset_tag {
                                        Some(_s) => { asset_tag = _s.to_string() },
                                        None => { }
                                    }
                                    match system.rack_order {
                                        Some(_s) => { rack_order = _s.to_string() },
                                        None => { }
                                    }
                                    println!("\track_order={} hostname={} asset_tag={}", rack_order, system.hostname, asset_tag);
                                }
                                
                            }

                        }
                    }
                },
                Err(_e) => { println!("No Results.{}", _e)}
            }
        },
        None => { println!("No Results") }
    }
    None
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
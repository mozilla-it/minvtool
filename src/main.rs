extern crate dirs;
extern crate ini;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod system;
mod servermodel;
mod operatingsystem;
mod systemtype;
mod systemrack;
mod minv_config;
mod inventory_api;
mod return_matches;


fn main() {
    let home_path = dirs::home_dir().unwrap();
    let config = minv_config::get_config(home_path);
    let matches = return_matches::return_matches();
    match matches.subcommand_matches("system") {
        Some(value) => { system::execute(value, config.clone()) },
        None => {}
    }
    match matches.subcommand_matches("servermodel") {
        Some(value) => { servermodel::execute(value, config.clone()) },
        None => {}
    }
    match matches.subcommand_matches("operatingsystem") {
        Some(value) => { operatingsystem::execute(value, config.clone()) },
        None => {}
    }
    match matches.subcommand_matches("systemtype") {
        Some(value) => { systemtype::execute(value, config.clone()) },
        None => {}
    }
    match matches.subcommand_matches("systemrack") {
        Some(value) => { systemrack::execute(value, config.clone()) },
        None => {}
    }
}
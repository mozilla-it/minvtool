extern crate dirs;
extern crate ini;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod system;
mod minv_config;
mod inventory_api;
use clap::{App, Arg, SubCommand, AppSettings};


fn main() {
    let home_path = dirs::home_dir().unwrap();
    let config = minv_config::get_config(home_path);
    // let _yaml = load_yaml!("cli.yml");
    // let matches = App::from_yaml(_yaml).get_matches();
        let matches = App::new("minvtool")
        .about("CLI interface to Mozilla Inventory")
        .version("0.0.1")
        .author("Rob Tucker <rtucker@mozilla.com>")
        .subcommand(SubCommand::with_name("system")
            .about("Interaces with System Objects")
            .subcommand(SubCommand::with_name("get")
                .about("Get System")
                .arg(Arg::with_name("hostname")
                    .required(true)
                    .help("Hostname of System to be retrieved")
                )
            )
            .subcommand(SubCommand::with_name("delete")
                .about("Delete System Objects")
                .arg(Arg::with_name("hostname")
                    .required(true)
                    .help("Hostname of System to be created")
                )
            )
            .subcommand(SubCommand::with_name("create")
                .about("Creates System Objects")
                .arg(Arg::with_name("hostname")
                    .required(true)
                    .help("Hostname of System to be created")
                )
                .arg(Arg::with_name("server-model-id")
                    .required(false)
                    .long("server-model-id")
                    .help("ID of server-model")
                    .takes_value(true)
                )
                .arg(Arg::with_name("server-model-name")
                    .required(false)
                    .long("server-model-name")
                    .help("Name of server-model")
                    .takes_value(true)
                )
                .arg(Arg::with_name("serial")
                    .required(false)
                    .long("serial")
                    .help("System serial number")
                    .takes_value(true)
                )
                .arg(Arg::with_name("asset-tag")
                    .required(false)
                    .long("asset-tag")
                    .help("System Asset Tag")
                    .takes_value(true)
                )
            )
            
        )
        .get_matches();

    match matches.subcommand_matches("system") {
        Some(value) => { system::execute(value, config.clone()) },
        None => {}
    }
}
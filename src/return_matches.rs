use clap::{App, Arg, SubCommand, ArgMatches};
pub fn return_matches<'a>() -> ArgMatches<'static> {
    let shared_args = vec!(
        Arg::with_name("serial")
            .required(false)
            .long("serial")
            .help("System serial number")
            .takes_value(true),
        Arg::with_name("asset-tag")
            .required(false)
            .long("asset-tag")
            .help("System asset-tag")
            .takes_value(true),
        Arg::with_name("switch-ports")
            .required(false)
            .long("switch-ports")
            .help("System switch-ports")
            .takes_value(true),
        Arg::with_name("oob-ip")
            .required(false)
            .long("oob-ip")
            .help("System oob-ip")
            .takes_value(true),
        Arg::with_name("server-model-id")
            .required(false)
            .long("server-model-id")
            .help("ID of server-model")
            .takes_value(true),
        Arg::with_name("server-model-name")
            .required(false)
            .long("server-model-name")
            .help("Name of server-model")
            .takes_value(true),
    );
        let matches = App::new("minvtool")
        .about("CLI interface to Mozilla Inventory")
        .version("0.0.1")
        .author("Rob Tucker <rtucker@mozilla.com>")
        .subcommand(SubCommand::with_name("system")
            .about("Interfaces with System Objects")
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
            .subcommand(SubCommand::with_name("update")
                .about("Updates System Objects")
                .arg(Arg::with_name("hostname")
                    .required(true)
                    .help("Hostname of System to be updated")
                )
                .args(&shared_args)
            )
            .subcommand(SubCommand::with_name("create")
                .about("Creates System Objects")
                .arg(Arg::with_name("hostname")
                    .required(true)
                    .help("Hostname of System to be created")
                )
                .args(&shared_args)
            )
            
        )
        .get_matches();
        return matches;

}
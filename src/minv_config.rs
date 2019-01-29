use ini::Ini;
use std::path::PathBuf;
use std::process;

#[derive(Clone)]
pub struct Config {
    pub token: String,
    pub hostname: String,
}

impl Config {
    pub fn full_path(& self) -> String{
        return self.hostname.clone();
    }
}

pub fn get_config(path: PathBuf) -> Config {
    let path_string = path.as_path().to_str().unwrap();
    let _file_name = ".minvtool.cfg";
    let full_file_path = format!("{}/{}",path_string.to_string(), _file_name);
    let conf = match Ini::load_from_file(full_file_path) {
        Ok(value) => value,
        Err(_error) => {
            println!("Error: unable to read config file.");
            process::exit(2);
        }
    };
    let main_config = match conf.section(Some("minvtool")) {
        Some(value) => {value},
        None => { 
            println!("Error: unable to read config section.");
            process::exit(2);
        }
    };

    let l_token = match main_config.get("token") {
        Some(value) => { value.to_string() },
        None => { 
            println!("Error: token required in .minvtool.cfg");
            process::exit(2);
        }
    };

    let l_hostname = match main_config.get("hostname") {
        Some(value) => { value.to_string() },
        None => { 
            println!("Error: hostname required in .minvtool.cfg");
            process::exit(2);
        },
    };

    let config = Config{
        token: l_token,
        hostname: l_hostname
    };
    return config;
}
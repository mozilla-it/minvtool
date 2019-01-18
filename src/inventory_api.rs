use reqwest;
use minv_config;
use serde_json::Value;
use serde_json::to_string;
use serde_json::from_str;
use reqwest::StatusCode;

#[allow(dead_code)]
pub struct InventoryResponse {
    pub count: usize,
    pub is_error: bool,
    pub is_empty: bool,
    pub text: String,
    pub response: Vec<Value>
}

impl Default for InventoryResponse {
    fn default() -> InventoryResponse {
        InventoryResponse { 
            count: 0,
            is_error: false,
            is_empty: true,
            text: String::new(),
            response: Vec::new()
        }
    }
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct RESTResponse {
    pub error: String,
    pub code: String,
    pub text: String,
}

fn resp_text_to_vec(mut i_resp: reqwest::Response) -> Result<Vec<Value>, reqwest::Error> {
    match i_resp.text() {
        Ok(_str) => { 
            let retval = from_str(&_str).unwrap();
            Ok(vec![retval])
        },
        Err(_err) => { Err(_err) }
    }
}

#[allow(dead_code)]
fn format_error(mut _resp: reqwest::Response) -> String {
    let error_text = _resp.text().unwrap();
    let error_response: Value = from_str(&error_text).unwrap();
    let error_response_text = to_string(&error_response["text"]).unwrap();
    let error_response_text_trimmed = error_response_text.trim_matches('"').to_string();
    return format!("{}", error_response_text_trimmed);

}

pub struct RESTApi {
    pub config: minv_config::Config
}

pub struct InventoryResult {
    pub count: String,
    pub next: String,
    pub previous: String,
    pub results: Vec<String>,
}

impl RESTApi {

    pub fn get_client(&self) -> reqwest::Client {
        return reqwest::Client::new();
    }

    fn get_url(&self, hostpath: &String, ipath: &String) -> String {

        return format!("{}/{}",hostpath, ipath);

    }

    pub fn get(&self, iref: String, token: String) -> Option<Value> {
        let client = self.get_client();
        let config = self.config.clone();
        let host_path = config.full_path();
        let full_path = format!("{}/", self.get_url(&host_path, &iref));
        let resp = client.get(full_path.as_str()).header("Authorization", format!("Token {}", token)).send();
        match resp {
            Ok(mut _resp) => {
                match _resp.status(){
                    StatusCode::OK => { 
                        let v: Value = serde_json::from_str(&_resp.text().unwrap()).unwrap();
                        return Some(v)
                    },
                    StatusCode::UNAUTHORIZED => {
                        println!("Error: Invalid Authentication")
                    },
                    StatusCode::NOT_IMPLEMENTED => {
                        println!("Error: Unimplemented")
                    },
                    s => { println!("Error: Unknown: {}", s) }
                }
            },
            Err(_err) => { 
                println!("Error: {}", _err);
            }
        }
        None
    }

    pub fn delete(&self, iref: String, token: String) -> Result<Value, String> {

        let client = self.get_client();
        let config = self.config.clone();
        let host_path = config.full_path();
        let full_path = self.get_url(&host_path, &iref);
        let resp = client.delete(full_path.as_str()).send();
        let resp = client.delete(full_path.as_str()).header("Authorization", format!("Token {}", token)).send();
        match resp {
            Ok(mut _resp) => {
                Ok(_resp.json().unwrap())
            }
            Err(_error) => {
                return Result::Err("Uknown Error".to_string());
            }
        }
    }




    pub fn create(&self, iref: String, post_data: Value, token: &String) -> Option<Vec<Value>> {
        let client = self.get_client();
        let config = self.config.clone();
        let host_path = config.full_path();
        let full_path = format!("{}/", self.get_url(&host_path, &iref));
        let resp = client.post(full_path.as_str()).header("Content-Type", "application/json").header("Authorization", format!("Token {}", token)).json(&post_data).send();
        match resp {
            Ok(mut _resp) => {
                match _resp.status(){
                    StatusCode::CREATED => { 
                        match resp_text_to_vec(_resp) {
                           Ok(_val) => { return Some(_val) } ,
                           Err(_) => { () }
                        }
                    },
                    StatusCode::OK => { 
                        match resp_text_to_vec(_resp) {
                           Ok(_val) => { return Some(_val) } ,
                           Err(_) => { () }
                        }
                    },
                    StatusCode::UNAUTHORIZED => {
                        println!("Invalid Authentication.");
                    },
                    StatusCode::BAD_REQUEST => { 
                        println!("{:?}", &_resp.status());
                        let foo = &_resp.text().unwrap();

                        println!("{:?}", &foo);
                    },
                    StatusCode::INTERNAL_SERVER_ERROR => { 
                        println!("{:?}", &_resp.text().unwrap());
                        println!("Internal Server Error");
                    },
                    s => { println!("Unknown Response: {}", s)}
                }
            },
            Err(_err) => { 
                println!("{}", _err);
            }
        }
        None
    }
}
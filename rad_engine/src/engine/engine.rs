extern crate serde_json;
use serde_json::{Value, Error};

pub fn rad(extended_rad: String) -> Result<String, String> {
    let json_rad_request: Value = serde_json::from_str(&extended_rad).unwrap();

    // Retrieve
    if json_rad_request["retrieve"]["type"] == "http-get" {

        Ok("Hola".to_string())
    } else {
        Err("Not allowed retrieve type".to_string())
    }


}

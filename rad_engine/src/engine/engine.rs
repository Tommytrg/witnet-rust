extern crate reqwest;
extern crate serde_json;

use serde_json::{Error, Value};

use std::io::{stdout, Write};

// struct RadRequest {
//     notBefore: u64,
//     retrieve: Vec<RadRetrieve>,
//     aggregate: Vec<RadAggregate>,
//     deliver: Vec<RadDeliver>
// }

// struct RadRetrieve {
//     kind: String,
//     url: String,
//     script: Vec<Function>,
// }

// struct RadAggregate {
//     script: Vec<Function>,
// }

// struct RadDeliver {
//     kind: String,
//     url: String,
// }

// struct Function {
//     f: String,
//     params: Params,
// }

// struct Params {
//     key: String,
//     amount: Option<String>,
//     kind: Option<String>,
//     f: Option<String>,
// }

pub fn rad(extended_rad: String) -> Result<u32, String> {
    let json_rad_request: Value = serde_json::from_str(&extended_rad).unwrap();

    // Retrieve
    if json_rad_request["retrieve"][0]["type"].to_string() == "\"http-get\"".to_string() {
        let request_url: String = str::replace(
            &json_rad_request["retrieve"][0]["url"].to_string(),
            "\"",
            "",
        );
        let request_script = &json_rad_request["retrieve"][0]["script"]
            .as_array()
            .unwrap();
        println!("{}", request_url);
        let mut response = reqwest::get(&request_url).unwrap();
        let response_headers = &response;
        let response_content = response.text();

        println!("****{:?}*****", response_content);

        // TODO: transform into fold
        let mut cost = 0;
        // let mut functions: Vec<?>;

        for x in request_script.iter() {}

        Ok(3)
    } else {
        Err("Not allowed retrieve type: ".to_string()
            + &json_rad_request["retrieve"][0]["type"].to_string())
    }
}

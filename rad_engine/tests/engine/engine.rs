use witnet_rad_engine::engine::engine::rad;

const RAD_REQUEST_SUCCESS: &str = r#"{
    "notBefore": 1577836800,
    "retrieve": [{
        "type": "http-get",
        "url": "https://openweathermap.org/data/2.5/weather?id=2950159&appid=b6907d289e10d714a6e88b30761fae22",
        "script": [
            { "f": "parseJSON" },
            { "f": "get", "params": { "key": "weather" } },
            { "f": "get", "params": { "key": "temp" } },
            { "f": "asFloat" }
        ]
    }, {
        "type": "http-get",
        "url": "https://api.apixu.com/v1/current.json?key=297bc8f9aa7841d7a0e95208180310&q=Berlin,DE",
        "script": [
            { "f": "parseJSON" },
            { "f": "get", "params": { "key": "current" } },
            { "f": "get", "params": { "key": "temp_c" } },
            { "f": "asFloat" }
        ]
    }],
    "aggregate": {
        "script": [
            { "f": "filter", "params": { "f": "gt", "value": -30  } },
            { "f": "filter", "params": { "f": "lt", "value": 50  } },
            { "f": "filter", "params": { "f": "deviates",  "type": "abs", "amount": 1.5 } },
            { "f": "reduce", "params": { "f": "avg", "type": "arithmetic" } }
        ]
    },
    "deliver": [{
        "type": "http-post",
        "url": "https://hooks.zapier.com/hooks/catch/3860543/l1awcw/"
    }]
}"#;

const RAD_REQUEST_INVALID_RETRIEVE_TYPE: &str = r#"{
    "notBefore": 1577836800,
    "retrieve": [{
        "type": "http-ge",
        "url": "https://openweathermap.org/data/2.5/weather?id=2950159&appid=b6907d289e10d714a6e88b30761fae22",
        "script": [
            { "f": "parseJSON" },
            { "f": "get", "params": { "key": "weather" } },
            { "f": "get", "params": { "key": "temp" } },
            { "f": "asFloat" }
        ]
    }, {
        "type": "http-get",
        "url": "https://api.apixu.com/v1/current.json?key=297bc8f9aa7841d7a0e95208180310&q=Berlin,DE",
        "script": [
            { "f": "parseJSON" },
            { "f": "get", "params": { "key": "current" } },
            { "f": "get", "params": { "key": "temp_c" } },
            { "f": "asFloat" }
        ]
    }],
    "aggregate": {
    "script": [
        { "f": "filter", "params": { "f": "gt", "value": -30  } },
        { "f": "filter", "params": { "f": "lt", "value": 50  } },
        { "f": "filter", "params": { "f": "deviates",  "type": "abs", "amount": 1.5 } },
        { "f": "reduce", "params": { "f": "avg", "type": "arithmetic" } }
    ]
    },
    "deliver": [{
        "type": "http-post",
        "url": "https://hooks.zapier.com/hooks/catch/3860543/l1awcw/"
    }]
}"#;

#[test]
fn invalid_retrieve_type() {
    // TODO: scape characters usin an enum
    assert_eq!(rad(RAD_REQUEST_INVALID_RETRIEVE_TYPE.to_string()), Err("Not allowed retrieve type: \"http-ge\"".to_string()));
}

#[test]
fn retrieve_content() {
    assert_eq!(rad(RAD_REQUEST_SUCCESS.to_string()), Ok(21));
}
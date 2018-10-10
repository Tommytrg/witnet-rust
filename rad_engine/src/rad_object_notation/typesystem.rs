use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::slice::Iter;

extern crate serde_json;

use serde_json::{Value, Error};

pub enum RadOperations {
    ParseJson,
}

pub struct ParseJson {}

impl ParseJson {
    pub fn operation(&self, string: &str) -> Result<Value, &str> {
        match serde_json::from_str(string) {
            Ok(json) => Ok(json),
            Err(e) => Err("Error parsing json")
        }
    }

    pub fn cost(&self) -> u64 {
        3
    }
}

pub struct GetFromJson {
    pub key: String // Should be String or Int64 but Value type only have Strings
}

impl GetFromJson {

    // pub fn operation(&self, json: Value) -> Option<Value> {
    //     Some(json[&self.key])
    // }

    pub fn cost(&self) -> u64 {
        3
    }
}

pub struct Filter {
    f: FilterType,
    kind: Option<String>,
    value: Option<f64>,
    amount: Option<f64>,
}

enum FilterType {
    gt,
    lt,
    loet,
    goet,
    equal,
    deviates
}

#[derive(PartialEq, Eq)]
enum NumOrStr {
    int64,
    Sring
}

impl Filter {
    pub fn operation<NumOrStr>(&self, array: Iter<NumOrStr>) -> Iter<NumOrStr> {
        match self.f {
            FilterType::gt => array.filter(|item| *item > self.value),
            // "lt" => array.filter(|item| item < self.value),
            // "loet" => array.filter(|item| item < self.value),
            // "goet" => array.filter(|item| item < self.value),
            // "equal" => array.filter(|item| item < self.value),
            // "deviates" => array.filter(|item| item < self.value),
        };

        array
    }
}

// Object
// toJSON() -> String
// TODO
// toXML() -> String
// TODO
// get(key: String) -> Object | Array | String | Int | Float
//TODO

// Array
// toJSON() -> String
// TODO

// get(index: Int) -> Object | Array | String | Int | Float
// TODO

// String
// categorize<T>(categories: HashMap<String, T>) -> T
// TODO

// hash(function: String) -> String
pub fn string_hash(string: String) -> String {
    let mut s = DefaultHasher::new();
    string.hash(&mut s);
    s.finish().to_string()
}

// length() -> Int
pub fn string_length(string: String) -> usize {
    string.len()
}

// parseJSON() -> Object | Array
// TODO
// parseXML() -> Object
// TODO

// toFloat() -> Float
pub fn string_to_float(string: String) -> Result<f64, ParseFloatError> {
    string.parse::<f64>()
}

// toInt() -> Int
pub fn string_to_int(string: String) -> Result<i64, ParseIntError> {
    string.parse::<i64>()
}

// toLowerCase() -> String
pub fn string_to_lowercase(string: String) -> String {
    string.to_ascii_lowercase()
}

// toUpperCase() -> String
pub fn string_to_uppercase(string: String) -> String {
    string.to_ascii_uppercase()
}

// Int
// abs() -> Int
pub fn int_abs(num: i64) -> i64 {
    num.abs()
}

// categorize<T>(categories: HashMap<Int, T>) -> T
// TODO

// modulo -> Int
// TODO

// mult() -> Int (division is mult(recip(x)))
pub fn int_mult (num_1: i64, num_2: i64) -> Result<i64, String> {
    match num_1.checked_mul(num_2) {
        Some(x) => Ok(x),
        None => Err("multiplication overflow".to_string())
    }
}

// neg() -> Int
pub fn int_neg (num: i64) -> Result<i64, String> {
    match num.checked_neg() {
        Some(x) => Ok(x),
        None => Err("negative overflow".to_string())
    }
}

// pow() -> Int (root is pow(recip(x)))
pub fn int_pow(num: i64, pow: u32) -> Result<i64, String> {
    match num.checked_pow(pow) {
        Some(x) => Ok(x),
        None => Err("pow overflow".to_string())
    }
}

// recip() -> Int
// TODO

// round() -> Int
// TODO

// sum() -> Int (difference is sum(neg(x)))
pub fn int_sum(num_1: i64, num_2: i64) -> Result<i64, String> {
    match num_1.checked_add(num_2) {
        Some(x) => Ok(x),
        None => Err("sum overflow".to_string())
    }
}

// toFloat() -> Float
pub fn int_to_float(num: i64) -> f64 {
    num as f64
}

// toString() -> String
pub fn int_to_string(num: i64) -> String {
    num.to_string()
}

// Float
// abs() -> Float
pub fn float_abs(num: f64) -> f64 {
    num.abs()
}

// ceil() -> Int
pub fn float_ceil(num: f64) -> i64 {
    num.ceil() as i64
}

// floor() -> Int
pub fn float_floor(num: f64) -> i64 {
    num.floor() as i64
}

// modulo() -> Float
pub fn float_modulo(num: f64, rhs: f64) -> f64 {
    num.mod_euc(rhs)
}

// mult() -> Float (division is mult(recip(x)))
// TODO

// neg() -> Float
// TODO

// pow() -> Float (root is pow(recip(x)))
// TODO

// recip() -> Float
// TODO

// round -> Int
pub fn float_round(num: f64) -> i64 {
    num.round() as i64
}

// sum() -> Float (difference is sum(neg(x)))
// TODO

// toString() -> String
pub fn float_to_string(num: f64) -> String {
    num.to_string()
}

// Boolean
// categorize<T>(categories: HashMap<Boolean, T>) -> T
// TODO

// neg() -> Boolean
pub fn bool_neg(value: bool) -> bool {
    !value
}

// and() -> Boolean
pub fn bool_and(value_1: bool, value_2: bool) -> bool {
    value_1 & value_2
}

//or() -> Boolean
pub fn bool_or(value_1: bool, value_2: bool) -> bool {
    value_1 | value_2
}

// toString() -> String
pub fn bool_to_string(value: bool) -> String {
    value.to_string()
}
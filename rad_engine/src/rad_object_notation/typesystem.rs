use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
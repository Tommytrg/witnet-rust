use witnet_rad_engine::rad_object_notation::typesystem;

#[test]
fn filter_gt () {
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 0, "gt", 0), [1, 2, 3].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 2, "gt", 0), [3].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 3, "gt", 0), Vec::<i64>::new());
}

#[test]
fn filter_lt () {
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 1, "lt", 0), Vec::<i64>::new());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 2, "lt", 0), [1].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 4, "lt", 0), [1, 2, 3].to_vec());
}

#[test]
fn filter_goet () {
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 1, "goet", 0), [1, 2, 3].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 2, "goet", 0), [2, 3].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 3, "goet", 0), [3].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 4, "goet", 0), Vec::<i64>::new());
}

#[test]
fn filter_loet () {
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 0, "loet", 0), Vec::<i64>::new());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 1, "loet", 0), [1].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 2, "loet", 0), [1, 2].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 4, "loet", 0), [1, 2, 3].to_vec());
}

#[test]
fn filter_equal () {
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 1, "equal", 0), [1].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 2, "equal", 0), [2].to_vec());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), 4, "equal", 0), Vec::<i64>::new());
    assert_eq!(typesystem::filter([1, 2, 3].to_vec(), -1, "equal", 0), Vec::<i64>::new());

}

#[test]
fn get_from_json() {
    let object = json!({
        "A": "first",
        "B": 1,
        "C": {
            "D": "last"
        },
    });
    assert_eq!(typesystem::get_from_json(object.clone(), "A"), Some(json!("first")));
    assert_eq!(typesystem::get_from_json(object.clone(), "B"), Some(json!(1)));
    assert_eq!(typesystem::get_from_json(object.clone(), "C"), Some(json!({"D": "last"})));
}

// Float methods
#[test]
fn float_abs() {
    assert_eq!(typesystem::float_abs(5.0), 5.0);
    assert_eq!(typesystem::float_abs(-5.0), 5.0);
    assert_eq!(typesystem::float_abs(0.0), 0.0);
}

#[test]
fn float_ceil() {
    assert_eq!(typesystem::float_ceil(4.4), 5);
    assert_eq!(typesystem::float_ceil(4.9), 5);
    assert_eq!(typesystem::float_ceil(4.0), 4);
}

#[test]
fn float_floor() {
    assert_eq!(typesystem::float_floor(4.4), 4);
    assert_eq!(typesystem::float_floor(4.9), 4);
    assert_eq!(typesystem::float_floor(4.0), 4);
}

#[test]
fn float_modulo() {
    assert_eq!(typesystem::float_modulo(7.0, 4.0), 3.0);
    assert_eq!(typesystem::float_modulo(-7.0, 4.0), 1.0);
    assert_eq!(typesystem::float_modulo(7.0, -4.0), 3.0);
    assert_eq!(typesystem::float_modulo(-7.0, -4.0), 1.0);
    // limitation due to round-off error
    assert_eq!(typesystem::float_modulo(-std::f64::EPSILON, -3.0), 3.0);
}

#[test]
fn float_round() {
    assert_eq!(typesystem::float_round(7.8), 8);
    assert_eq!(typesystem::float_round(6.5), 7);
    assert_eq!(typesystem::float_round(6.1), 6);
    assert_eq!(typesystem::float_round(-6.1), -6);
    assert_eq!(typesystem::float_round(0.0), 0);
}

#[test]
fn float_to_string() {
    assert_eq!(typesystem::float_to_string(7.8), "7.8".to_string());
    assert_eq!(typesystem::float_to_string(6.1), "6.1".to_string());
    assert_eq!(typesystem::float_to_string(6.0), "6".to_string());
    assert_eq!(typesystem::float_to_string(-11.0), "-11".to_string());
}

// Int methods
#[test]
fn int_abs() {
    assert_eq!(typesystem::int_abs(5), 5);
    assert_eq!(typesystem::int_abs(-5), 5);
    assert_eq!(typesystem::int_abs(0), 0);
}

#[test]
fn int_mult() {
    assert_eq!(typesystem::int_mult(2, 2).is_ok(), true);
    assert_eq!(typesystem::int_mult(2, 2).unwrap(), 4);
    assert_eq!(typesystem::int_mult(i64::max_value(), 2).is_err(), true);
}

#[test]
fn int_sum() {
    assert_eq!(typesystem::int_sum(2, 2).is_ok(), true);
    assert_eq!(typesystem::int_sum(2, 2).unwrap(), 4);
    assert_eq!(typesystem::int_sum(i64::max_value(), 2).is_err(), true);
}

#[test]
fn int_neg() {
    assert_eq!(typesystem::int_neg(1).is_ok(), true);
    assert_eq!(typesystem::int_neg(1).unwrap(), -1);
    assert_eq!(typesystem::int_neg(-1).is_ok(), true);
    assert_eq!(typesystem::int_neg(-1).unwrap(), 1);
    assert_eq!(typesystem::int_neg(0).is_ok(), true);
    assert_eq!(typesystem::int_neg(0).unwrap(), 0);
    assert_eq!(typesystem::int_neg(i64::min_value()).is_err(), true);
}

#[test]
fn int_pow() {
    assert_eq!(typesystem::int_pow(2, 3).is_ok(), true);
    assert_eq!(typesystem::int_pow(2, 3).unwrap(), 8);
    assert_eq!(typesystem::int_pow(2, 0).is_ok(), true);
    assert_eq!(typesystem::int_pow(2, 0).unwrap(), 1);
    assert_eq!(typesystem::int_pow(i64::max_value(), 2).is_err(), true);
}

// String methods
#[test]
fn string_hash() {
    let string1 = "abcd";
    let string2 = "dcba";
    assert_eq!(
        typesystem::string_hash(string1.to_string()),
        "13543138095457285553"
    );
    assert_eq!(
        typesystem::string_hash(string2.to_string()),
        "4094066109971077621"
    );
    assert_ne!(string1, string2);
    assert_ne!(
        typesystem::string_hash(string1.to_string()),
        typesystem::string_hash(string2.to_string())
    );
}

#[test]
fn int_to_float() {
    assert_eq!(typesystem::int_to_float(3), 3.0);
    assert_eq!(typesystem::int_to_float(-3), -3.0);
    assert_eq!(
        typesystem::int_to_float(i64::max_value()),
        9223372036854776000.0
    );
    assert_eq!(
        typesystem::int_to_float(i64::min_value()),
        -9223372036854776000.0
    );
}

#[test]
fn int_to_string() {
    assert_eq!(typesystem::int_to_string(3), "3".to_string());
    assert_eq!(typesystem::int_to_string(-1), "-1".to_string());
}

#[test]
fn string_to_float() {
    assert_eq!(typesystem::string_to_float("1.2".to_string()).is_ok(), true);
    assert_eq!(typesystem::string_to_float("1.2".to_string()).unwrap(), 1.2);
    assert_eq!(typesystem::string_to_float("1.0".to_string()).unwrap(), 1.0);
    assert_eq!(
        typesystem::string_to_float("1,2".to_string()).is_err(),
        true
    );
    assert_eq!(
        typesystem::string_to_float("abcd".to_string()).is_err(),
        true
    );
    assert_eq!(
        typesystem::string_to_float("1.E".to_string()).is_err(),
        true
    );
}

#[test]
fn string_to_int() {
    assert_eq!(typesystem::string_to_int("8".to_string()).is_ok(), true);
    assert_eq!(typesystem::string_to_int("8".to_string()).unwrap(), 8);
    assert_eq!(typesystem::string_to_int("1.2".to_string()).is_err(), true);
    assert_eq!(typesystem::string_to_int("1,0".to_string()).is_err(), true);
    assert_eq!(typesystem::string_to_int("abcd".to_string()).is_err(), true);
}

#[test]
fn string_length() {
    let string = "abcd";
    assert_eq!(typesystem::string_length(string.to_string()), 4);
}

#[test]
fn string_to_lowercase() {
    let lowercase_string = "abcd";
    let uppercase_string = "ABCD";
    let string = "AbCd 5";
    assert_eq!(
        typesystem::string_to_lowercase(uppercase_string.to_string()),
        "abcd"
    );
    assert_eq!(
        typesystem::string_to_lowercase(lowercase_string.to_string()),
        "abcd"
    );
    assert_eq!(
        typesystem::string_to_lowercase(string.to_string()),
        "abcd 5"
    );
}

#[test]
fn string_to_uppercase() {
    let lowercase_string = "abcd";
    let uppercase_string = "ABCD";
    let string = "AbCd 5";
    assert_eq!(
        typesystem::string_to_uppercase(uppercase_string.to_string()),
        "ABCD"
    );
    assert_eq!(
        typesystem::string_to_uppercase(lowercase_string.to_string()),
        "ABCD"
    );
    assert_eq!(
        typesystem::string_to_uppercase(string.to_string()),
        "ABCD 5"
    );
}

#[test]
fn bool_to_string() {
    assert_eq!(typesystem::bool_to_string(true), "true".to_string());
    assert_eq!(typesystem::bool_to_string(false), "false".to_string());
}

#[test]
fn bool_neg() {
    assert_eq!(typesystem::bool_neg(true), false);
    assert_eq!(typesystem::bool_neg(false), true);
}

#[test]
fn bool_and() {
    assert_eq!(typesystem::bool_and(false, false), false);
    assert_eq!(typesystem::bool_and(false, true), false);
    assert_eq!(typesystem::bool_and(true, false), false);
    assert_eq!(typesystem::bool_and(true, true), true);
}

#[test]
fn bool_or() {
    assert_eq!(typesystem::bool_or(false, false), false);
    assert_eq!(typesystem::bool_or(false, true), true);
    assert_eq!(typesystem::bool_or(true, false), true);
    assert_eq!(typesystem::bool_or(true, true), true);
}

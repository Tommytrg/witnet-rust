extern crate witnet_rad_engine as rad_engine;

use rad_engine::greetings;

#[test]
fn data_structures_greeeting() {
    assert_eq!(greetings(), String::from("Hello from rad_engine!"));
}

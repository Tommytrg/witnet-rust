#[derive(Debug)]
struct Object <T>{
    value: T,
}

impl<T> Object<T> {
    fn to_json(&self) {}
    fn to_xml(&self) {}
    fn get(&self, key: &str) {}
}
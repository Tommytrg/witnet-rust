#[derive(Debug)]
struct Option <T>{
    value: T,
}

impl<T> Option<T> {
    fn is_some(&self) {}
    fn is_none(&self) {}
    fn get(&self) {}
    fn get_or(&self) {}
    fn ok_or(&self) {}
}

#[derive(Debug)]
struct Int <T>{
    value: T,
}

impl<T> Int<T> {
    fn to_string(&self) {}
    fn to_float(&self) {}
    fn neg(&self) {}
    fn abs(&self) {}
    fn recip(&self) {}
    fn round(&self, num: u64) {}
    fn sum(&self, num: u64) {}
    fn mult(&self, num: u64) {}
    fn pow(&self, num: u64) {}
    fn modulo(&self, num: u64) {}
    fn categorize(&self) {}
}

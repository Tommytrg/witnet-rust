#[derive(Debug)]
struct Float <T>{
    value: T,
}

impl<T> Float<T> {
    fn to_string(&self) {}
    fn floor(&self) {}
    fn ceil(&self) {}
    fn round(&self, num: Option<u64>) {}
    fn neg(&self) {}
    fn abs(&self) {}
    fn recip(&self) {}
    fn sum(&self, num: u64) {}
    fn mult(&self, num: u64) {}
    fn pow(&self, num: u64) {}
    fn modulo(&self, num: u64) {}
    fn categorize(&self) {}
}
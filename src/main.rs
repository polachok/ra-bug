struct Hello(inner::Hello);

impl Hello {
    fn new() -> Self {
        Hello(inner::Hello::new())
    }
}

mod inner {
    use super::*;

    pub struct Hello {
        field: u32,
    }

    impl Hello {
        pub fn new() -> Self {
            Hello { field: 0 }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

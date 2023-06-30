pub mod hello_world {
    pub fn get_hello_world() -> &'static str {
        "Hello, World!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world::get_hello_world(), "Hello, World!");
    }
}
//rustc --test helloworld.rs -o helloworld_test && ./helloworld_test

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
        let result = hello_world::get_hello_world();
        assert_eq!("Hello, World!", result);
    }
}
//rustc --test helloworld.rs -o helloworld_test && ./helloworld_test

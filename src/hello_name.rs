pub fn hello(name: &str) -> String {
    println!("Hello {name}.");
    format!("Hello {}", name)    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello("Eduardo"), "Hello Eduardo")
    }
}
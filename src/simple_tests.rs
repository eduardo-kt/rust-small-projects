pub fn simple_sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sum_2_2() {
        assert_eq!(simple_sum(2,2), 4)
    }

    #[test]
    fn simple_sum_fails() {
        assert_eq!(simple_sum(3, 5),9)
    }

    #[test]
    #[ignore = "waiting revision"] // rodar com: cargo test --lib -- --ignored
    // para rodar todos use: cargo test --lib -- --include-ignored
    fn simple_sum_waits() {
        assert_eq!(simple_sum(-1, -1),-2)
    }
}
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
}
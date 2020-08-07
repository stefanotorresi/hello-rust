pub fn fibonacci(n: u128) -> u128 {
    if n < 2 {
        return 1
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(1, fibonacci(0));
        assert_eq!(1, fibonacci(1));
        assert_eq!(2, fibonacci(2));
        assert_eq!(3, fibonacci(3));
        assert_eq!(5, fibonacci(4));
        assert_eq!(8, fibonacci(5));
        assert_eq!(13, fibonacci(6));
    }
}
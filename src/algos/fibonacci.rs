pub fn fibonacci(n: u128) -> u128 {
    if n <= 1 {
        return 0
    }
    if n == 2 {
        return 1
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

static mut FIBONACCI_NUMBERS: Vec<u128> = Vec::new();

pub unsafe fn fibonacci_iterative(n: usize) -> u128 {
    let result = FIBONACCI_NUMBERS.get(n);
    if result.is_some() {
        return *result.unwrap()
    }

    let last = FIBONACCI_NUMBERS.len();
    for i in last..=n {
        if i <= 1 {
            FIBONACCI_NUMBERS.push(0);
            continue
        }

        if i == 2 {
            FIBONACCI_NUMBERS.push(1);
            continue
        }

        FIBONACCI_NUMBERS.push(FIBONACCI_NUMBERS[i - 1] + FIBONACCI_NUMBERS[i - 2]);
    }

    return FIBONACCI_NUMBERS[n];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(0, fibonacci(0));
        assert_eq!(0, fibonacci(1));
        assert_eq!(1, fibonacci(2));
        assert_eq!(1, fibonacci(3));
        assert_eq!(2, fibonacci(4));
        assert_eq!(3, fibonacci(5));
        assert_eq!(5, fibonacci(6));
        assert_eq!(8, fibonacci(7));
        assert_eq!(13, fibonacci(8));
    }

    #[test]
    fn test_fibonacci_memoized() {
        unsafe {
            assert_eq!(63245986, fibonacci_memoized(40));
            assert_eq!(0, fibonacci_memoized(0));
            assert_eq!(0, fibonacci_memoized(1));
            assert_eq!(1, fibonacci_memoized(2));
            assert_eq!(1, fibonacci_memoized(3));
            assert_eq!(2, fibonacci_memoized(4));
            assert_eq!(3, fibonacci_memoized(5));
            assert_eq!(5, fibonacci_memoized(6));
            assert_eq!(8, fibonacci_memoized(7));
            assert_eq!(13, fibonacci_memoized(8));
        }
    }
}
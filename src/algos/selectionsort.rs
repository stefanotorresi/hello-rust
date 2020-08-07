pub fn sort<T: Ord>(mut elements: Vec<T>) -> Vec<T> {
    let mut i = 0;
    let length = elements.len();
    while i < length-1 {
        let mut j = i + 1;
        let mut min_index = i as usize;
        while j < length {
            if elements[j] < elements[min_index] {
                min_index = j;
            }
            j += 1;
        }
        if min_index != i {
            elements.swap(i, min_index);
        }
        i += 1;
    }

    return elements;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(vec![1, 2, 3, 4, 5, 7, 8, 11], sort(vec![11, 1, 4, 3, 2, 5, 8, 7]));
        assert_eq!(vec!["a", "b", "c", "d"], sort(vec!["c", "b", "d", "a"]));
    }
}
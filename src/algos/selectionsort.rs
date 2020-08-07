pub fn selectionsort<T: Ord>(mut elements: Vec<T>) -> Vec<T> {
    let mut i = 0;
    while i < elements.len()-1 {
        let min_index = min_index(&mut elements, i);
        if min_index != i {
            elements.swap(i, min_index);
        }
        i += 1;
    }

    return elements;
}

fn min_index<T: Ord>(elements: &mut Vec<T>, i: usize) -> usize {
    let mut j = i + 1;
    let mut min_index = i;
    while j < elements.len() {
        if elements[j] < elements[min_index] {
            min_index = j;
        }
        j += 1;
    }
    min_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(vec![1, 2, 3, 4, 5, 7, 8, 11], selectionsort(vec![11, 1, 4, 3, 2, 5, 8, 7]));
        assert_eq!(vec!["a", "b", "c", "d"], selectionsort(vec!["c", "b", "d", "a"]));
    }
}
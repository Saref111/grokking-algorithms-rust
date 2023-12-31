pub fn search_binary<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        let guess = &arr[mid];

        if guess == target {
            return Some(mid)
        }

        if guess > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_binary() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8];
        let target = 5;
        let result = search_binary(&arr, &target);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_search_binary_not_found() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8];
        let target = 9;
        let result = search_binary(&arr, &target);
        assert_eq!(result, None);
    }
}

pub fn selection_sort_1<T: Ord>(arr: &mut [T]) -> &[T] {
    for i in 0..arr.len() {
        let mut small = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[i] {
                small = j;
            }
        }
        arr.swap(small, i);
    }

    arr
} 

pub fn selection_sort_2<T: Ord>(arr: &mut [T]) -> &[T] {
    for i in 0..arr.len() {
        if let Some((small, _)) = arr.iter().enumerate().skip(i).min_by_key(|&(_, x)| x) {
            arr.swap(small, i);
        }
    }

    arr
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_selection_sort_1() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        let result = selection_sort_1(&mut arr);
        assert_eq!(result, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_selection_sort_2() {
        let mut arr = [8, 7, 6, 5, 4, 3, 2, 1];
        let result = selection_sort_1(&mut arr);
        assert_eq!(result, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_selection_sort_3() {
        let mut arr = [8, 7, 6, 5, 4, 3, 2, 1];
        let result = selection_sort_1(&mut arr);
        assert_eq!(result, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

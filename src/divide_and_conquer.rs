#[allow(dead_code)]
pub fn sum(arr: &[i32]) -> i32 {
    match arr.len() {
        0 => 0,
        1 => arr[0],
        _ => arr[0] + sum(&arr[1..])
    }
}

#[allow(dead_code)]
pub fn count(list: &[i32]) -> i32 {
    match list {
        [] => 0,
        _ => 1 + count(&list[1..])
    }
}

#[allow(dead_code)]
pub fn find_max(arr: &[i32]) -> Option<i32> {
    match arr {
        [] => None,
        [a] => Some(*a),
        [a, ..] => {
            let rest_max = find_max(&arr[1..]);
            match rest_max {
                None => Some(*a),
                Some(num) => Some(*a.max(&num)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty_array() {
        let arr = [];
        let sum_result = sum(&arr);

        assert_eq!(sum_result, 0);
    }

    #[test]
    fn test_sum_with_one_elem() {
        let arr = [345];
        let sum_result = sum(&arr);

        assert_eq!(sum_result, 345);
    }

    #[test]
    fn test_sum() {
        let arr = [1,2,3,4,5,6,7,8,9];
        let sum_result = sum(&arr);

        assert_eq!(sum_result, 45);
    }

    #[test]
    fn test_count_with_empty_array() {
        let list = [];
        let len = count(&list);

        assert_eq!(len, list.len() as i32);
    }
    #[test]
    fn test_count_with_one_elem() {
        let list = [1];
        let len = count(&list);

        assert_eq!(len, list.len() as i32);
    }
    #[test]
    fn test_count() {
        let list = [1,2,3,4,5,6,7,8,9];
        let len = count(&list);

        assert_eq!(len, list.len() as i32);
    }

    #[test]
    fn test_find_max_empty_array() {
        let arr = [];
        let max_result = find_max(&arr);

        assert_eq!(max_result, None);
    }

    #[test]
    fn test_find_max_with_one_elem() {
        let arr = [345];
        let max_result = find_max(&arr);

        assert_eq!(max_result, Some(345));
    }

    #[test]
    fn test_find_max() {
        let arr = [1,2,3,4,5,6,7,8,9];
        let max_result = find_max(&arr);

        assert_eq!(max_result, Some(9));
    }

    #[test]
    fn test_find_max_with_negative_numbers() {
        let arr = [-1,-2,-3,-4,-5,-6,-7,-8,-9];
        let max_result = find_max(&arr);

        assert_eq!(max_result, Some(-1));
    }
}

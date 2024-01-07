pub fn sum(arr: &[i32]) -> i32 {
    match arr.len() {
        0 => 0,
        1 => arr[0],
        _ => arr[0] + sum(&arr[1..])
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
}

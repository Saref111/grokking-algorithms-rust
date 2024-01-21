pub fn quick_sort(arr: &[i32]) -> Vec<i32> {
    match arr.len() {
        0 => arr.into(),
        1 => arr.into(),
        2 => if arr[0] > arr[1]  {
            vec![arr[1], arr[0]]            
        } else {arr.into()},
        _ => {
            let pivot = arr[0];
            let (less, more): (Vec<i32>, Vec<i32>) = arr.iter()
                                                            .skip(1)
                                                            .fold((vec![],vec![]), |(mut less, mut more), i| {
                                                                if i > &pivot {
                                                                    more.push(*i);
                                                                } else {
                                                                    less.push(*i);
                                                                }

                                                                (less, more)
                                                            }); 

            let sorted_less = quick_sort(&less);
            let sorted_more = quick_sort(&more);


            [sorted_less, vec![pivot], sorted_more].concat()
        } 
    }
}

pub fn quick_sort_part(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..len]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(quick_sort(&arr), vec![]);
    }

    #[test]
    fn test_quick_sort_single() {
        let arr = vec![5];
        assert_eq!(quick_sort(&arr), vec![5]);
    }

    #[test]
    fn test_quick_sort_two_elements() {
        let arr = vec![5, 3];
        assert_eq!(quick_sort(&arr), vec![3, 5]);
    }

    #[test]
    fn test_quick_sort_multiple_elements() {
        let arr = vec![5, 3, 8, 4, 6];
        assert_eq!(quick_sort(&arr), vec![3, 4, 5, 6, 8]);
    }

    #[test]
    fn test_quick_sort_already_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(quick_sort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse_sorted() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(quick_sort(&arr), vec![1, 2, 3, 4, 5]);
    }
}

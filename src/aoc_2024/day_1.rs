// this is my quick and dirty solution to day 1 from 2024 advent of code
fn day_one(l1: &mut [i32], l2: &mut [i32]) -> i32 {
    l1.sort();
    l2.sort();
    let mut total_distance = 0;
    for (n1, n2) in l1.iter().zip(l2.iter()) {
        let distance = n1 - n2;
        total_distance += distance.abs();
    }

    total_distance
}

// cleaner solution
fn day_one_no_sort(l1: &mut [i32], l2: &mut [i32]) -> i32 {
    let l1_sum: i32 = l1.iter().sum();
    let l2_sum: i32 = l2.iter().sum();

    (l1_sum - l2_sum).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one() {
        let mut l1 = vec![3, 4, 2, 1, 3, 3];
        let mut l2 = vec![4, 3, 5, 3, 9, 3];
        let expected_result = 11;

        let result = day_one(&mut l1, &mut l2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_day_one_no_sort() {
        let mut l1 = vec![3, 4, 2, 1, 3, 3];
        let mut l2 = vec![4, 3, 5, 3, 9, 3];
        let expected_result = 11;

        let result = day_one_no_sort(&mut l1, &mut l2);

        assert_eq!(result, expected_result);
    }
}

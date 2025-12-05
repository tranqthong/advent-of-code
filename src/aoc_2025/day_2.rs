use std::fs;

pub fn gift_shop(input_filepath: &str) -> (u64, u64) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let ids = input_content.split(",");
    let id_collection: Vec<&str> = ids.collect();

    sum_invalid_ids(&id_collection)
}

fn sum_invalid_ids(ids: &Vec<&str>) -> (u64, u64) {
    let mut invalid_ids_p1: Vec<u64> = vec![];
    let mut invalid_ids_p2: Vec<u64> = vec![];

    for id_range in ids {
        check_id_range(id_range, &mut invalid_ids_p1, &mut invalid_ids_p2);
    }

    (invalid_ids_p1.iter().sum(), invalid_ids_p2.iter().sum())
}

fn check_id_range(id_range: &str, invalid_ids_p1: &mut Vec<u64>, invalid_ids_p2: &mut Vec<u64>) {
    let num_range: Vec<&str> = id_range.split("-").collect();
    let x = num_range[0]
        .parse::<u64>()
        .expect("Unable to convert str to int");
    let y = num_range[1]
        .parse::<u64>()
        .expect("Unable to convert str to int");
    for i in x..y + 1 {
        if is_invalid_p1(i) {
            invalid_ids_p1.push(i);
        }

        if is_invalid_p2(i) {
            invalid_ids_p2.push(i);
        }
    }
}

fn is_invalid_p1(num: u64) -> bool {
    let num_str = num.to_string();
    if num_str.len().is_multiple_of(2) {
        let half_point = num_str.len() / 2;
        let first_half = &num_str[0..half_point];
        let second_half = &num_str[half_point..num_str.len()];

        return first_half == second_half;
    }
    false
}

fn is_invalid_p2(num: u64) -> bool {
    let num_str: String = num.to_string();
    let str_len = num_str.len();
    for i in 1..str_len {
        if num_str.len().is_multiple_of(i) {
            let num_substrs = split_num_substr(&num_str, i);
            if is_all_elements_same(num_substrs) {
                return true;
            }
        }
    }
    false
}

fn is_all_elements_same(num_substrs: Vec<&str>) -> bool {
    let sub_str = num_substrs[0];
    num_substrs.iter().all(|&sub| sub == sub_str)
}

fn split_num_substr(num_str: &str, substr_len: usize) -> Vec<&str> {
    let mut sub_strs = Vec::with_capacity(num_str.len() / substr_len);
    let mut iter = num_str.chars();
    let mut idx = 0;
    while idx < num_str.len() {
        let mut length = 0;
        for c in iter.by_ref().take(substr_len) {
            length += c.len_utf8();
        }
        sub_strs.push(&num_str[idx..idx + length]);
        idx += length;
    }
    sub_strs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_input() {
        let ex_input = "src/aoc_2025/inputs/day_2_ex.txt";
        let result = gift_shop(ex_input);
        let expected_p1 = 1227775554;
        let expected_p2 = 4174379265;

        assert_eq!(result.0, expected_p1);
        assert_eq!(result.1, expected_p2);
    }

    #[test]
    fn test_split_num_substr() {
        let num_str = "123";
        let mut result = split_num_substr(num_str, 1);

        assert_eq!(result.len(), 3);
        assert_eq!(result.pop().unwrap(), "3");
        assert_eq!(result.pop().unwrap(), "2");
        assert_eq!(result.pop().unwrap(), "1");
    }

    #[test]
    fn test_split_num_substr_length_5() {
        let num_str = "1188511885";
        let mut result = split_num_substr(num_str, 5);

        assert_eq!(result.len(), 2);
        assert_eq!(result.pop().unwrap(), "11885");
        assert_eq!(result.pop().unwrap(), "11885");
    }

    #[test]
    fn test_is_all_elements_same() {
        let test_example = split_num_substr("111", 1);
        let result = is_all_elements_same(test_example);

        assert!(result);
    }

    #[test]
    fn test_id_range() {
        let mut invalid_ids_p1: Vec<u64> = vec![];
        let mut invalid_ids_p2: Vec<u64> = vec![];
        let id_range = "998-1012";
        check_id_range(id_range, &mut invalid_ids_p1, &mut invalid_ids_p2);

        let result = invalid_ids_p1.pop().unwrap();

        assert_eq!(result, 1010);
    }

    #[test]
    fn test_id_range_95_115() {
        let mut invalid_ids_p1: Vec<u64> = vec![];
        let mut invalid_ids_p2: Vec<u64> = vec![];
        let id_range = "95-115";

        check_id_range(id_range, &mut invalid_ids_p1, &mut invalid_ids_p2);
        let result = invalid_ids_p1.pop().unwrap();
        assert_eq!(result, 99);

        let result_1 = invalid_ids_p2.pop().unwrap();
        assert_eq!(result_1, 111);

        let result_2 = invalid_ids_p2.pop().unwrap();
        assert_eq!(result_2, 99);
    }

    #[test]
    fn test_is_invalid_11() {
        let result = is_invalid_p1(11);
        assert!(result);
    }

    #[test]
    fn test_is_invalid_10() {
        let result = is_invalid_p1(10);
        assert!(!result);
    }
}

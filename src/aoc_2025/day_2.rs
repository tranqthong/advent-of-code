use std::fs;

pub fn gift_shop(input_filepath: &str) -> u64 {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let ids = input_content.split(",");
    let id_collection: Vec<&str> = ids.collect();

    sum_invalid_ids(&id_collection)
}

fn sum_invalid_ids(ids: &Vec<&str>) -> u64 {
    let mut invalid_ids: Vec<u64> = vec![];

    for id_range in ids {
        check_id_range(id_range, &mut invalid_ids);
    }

    invalid_ids.iter().sum()
}

fn check_id_range(id_range: &str, invalid_ids: &mut Vec<u64>) {
    let num_range: Vec<&str> = id_range.split("-").collect();
    let x = num_range[0].parse::<u64>().expect("Unable to convert str to int");
    let y = num_range[1].parse::<u64>().expect("Unable to convert str to int");
    for i in x..y+1 {
        if is_invalid(i) {
            invalid_ids.push(i);
        }

    }
}

fn is_invalid(num: u64) -> bool {
    let num_str = num.to_string();
    if num_str.len().is_multiple_of(2) {
        let half_point = num_str.len() / 2;
        let first_half = &num_str[0..half_point];
        let second_half = &num_str[half_point..num_str.len()];

        return first_half == second_half;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_input() {
        let ex_input = "src/aoc_2025/inputs/day_2_ex.txt";
        let result = gift_shop(ex_input);
        let expected = 1227775554;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_id_range() {
        let mut invalid_ids: Vec<u64> = vec![];
        let id_range = "998-1012";
        check_id_range(id_range, &mut invalid_ids);

        let result = invalid_ids.pop().unwrap();

        assert_eq!(result, 1010);
    }

    #[test]
    fn test_is_invalid_11() {
        let result = is_invalid(11);
        assert!(result);
    }

    #[test]
    fn test_is_invalid_10() {
        let result = is_invalid(10);
        assert!(!result);
    }
}
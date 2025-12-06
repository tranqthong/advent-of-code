use std::fs;

pub fn lobby(input_filepath: &str) -> (u32, u64) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_iter = input_content.lines();
    let battery_banks: Vec<&str> = input_iter.collect();

    get_total_output_joltage(battery_banks)
}

fn get_total_output_joltage(battery_banks: Vec<&str>) -> (u32, u64) {
    let mut total_joltage_p1 = 0;
    let mut total_joltage_p2 = 0;
    for bank in battery_banks {
        total_joltage_p1 += largest_joltage_p1(bank);
        total_joltage_p2 += largest_joltage_p2(bank);
    }

    (total_joltage_p1, total_joltage_p2)
}

fn largest_joltage_p1(battery_bank: &str) -> u32 {
    let mut largest_digit = 0;
    let mut largest_digit_pos = 0;

    let bank_values: Vec<u32> = battery_bank.chars().flat_map(|c| c.to_digit(10)).collect();
    for idx in 0..(bank_values.len() - 1) {
        let jolt_value = bank_values[idx];
        if jolt_value > largest_digit {
            largest_digit = jolt_value;
            largest_digit_pos = idx;
        }
    }

    let mut second_largest_digit = 0;
    for idx in (largest_digit_pos + 1)..bank_values.len() {
        if bank_values[idx] > second_largest_digit {
            second_largest_digit = bank_values[idx];
        }
    }

    largest_digit * 10 + second_largest_digit
}

fn largest_joltage_p2(battery_bank: &str) -> u64 {
    let mut joltage_p2: Vec<u32> = Vec::with_capacity(12);
    let bank_values: Vec<u32> = battery_bank.chars().flat_map(|c| c.to_digit(10)).collect();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_joltage() {
        let ex_input = "src/aoc_2025/inputs/day_3_ex.txt";
        let p1_result = 357;
        let p2_result: u64 = 3121910778619;

        let result = lobby(ex_input);
        assert_eq!(result.0, p1_result);
        assert_eq!(result.1, p2_result);
    }

    #[test]
    fn test_largest_joltage_98() {
        let test_input = "987654321111111";

        let result = largest_joltage_p1(test_input);

        assert_eq!(result, 98);
    }

    #[test]
    fn test_largest_joltage_89() {
        let test_input = "811111111111119";

        let result = largest_joltage_p1(test_input);

        assert_eq!(result, 89);
    }
}

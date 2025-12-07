use std::fs;

pub fn lobby(input_filepath: &str) -> (u64, u64) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_iter = input_content.lines();
    let battery_banks: Vec<&str> = input_iter.collect();

    get_total_output_joltage(&battery_banks)
}

fn get_total_output_joltage(battery_banks: &Vec<&str>) -> (u64, u64) {
    let mut total_joltage_p1 = 0;
    let mut total_joltage_p2 = 0;
    for bank in battery_banks {
        total_joltage_p1 += largest_joltage(bank.as_bytes(), 2);
        total_joltage_p2 += largest_joltage(bank.as_bytes(), 12);
    }

    (total_joltage_p1, total_joltage_p2)
}

fn largest_joltage(mut battery_bank: &[u8], num_digits: usize) -> u64 {
    let mut joltage = 0;

    for i in (0..num_digits).rev() {
        let (largest_idx, volts) = battery_bank
            .iter()
            .enumerate()
            .rev()
            .skip(i)
            .max_by_key(|x| x.1)
            .unwrap();
        battery_bank = &battery_bank[largest_idx + 1..];
        joltage = joltage * 10 + (volts - b'0') as u64;
    }

    joltage
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
    fn test_largest_joltage_p1_98() {
        let test_input = "987654321111111";

        let result = largest_joltage(test_input.as_bytes(), 2);

        assert_eq!(result, 98);
    }

    #[test]
    fn test_largest_joltage_p2_98() {
        let test_input = "987654321111111";

        let result = largest_joltage(test_input.as_bytes(), 12);

        assert_eq!(result, 987654321111);
    }

    #[test]
    fn test_largest_joltage_p1_89() {
        let test_input = "811111111111119";

        let result = largest_joltage(test_input.as_bytes(), 2);

        assert_eq!(result, 89);
    }

    #[test]
    fn test_largest_joltage_p2_89() {
        let test_input = "811111111111119";

        let result = largest_joltage(test_input.as_bytes(), 12);

        assert_eq!(result, 811111111119);
    }

    #[test]
    fn test_largest_voltage_p1_78() {
        let test_input = "234234234234278";

        let result = largest_joltage(test_input.as_bytes(), 2);

        assert_eq!(result, 78);
    }

    #[test]
    fn test_largest_voltage_p2_78() {
        let test_input = "234234234234278";

        let result = largest_joltage(test_input.as_bytes(), 12);

        assert_eq!(result, 434234234278);
    }
}

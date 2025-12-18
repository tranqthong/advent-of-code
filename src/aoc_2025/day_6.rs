use std::fs;

pub fn trash_compactor(input_filepath: &str) -> (usize, usize) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_str_lists: Vec<&str> = input_content.lines().collect();

    let mut numbers_list: Vec<Vec<usize>> = vec![];
    let mut operations: Vec<&str> = vec![];

    // parsing input for part 1
    for (line_idx, line) in input_str_lists.iter().enumerate() {
        if line_idx != input_str_lists.len() - 1 {
            let num_str: Vec<&str> = line.split_whitespace().collect();
            let mut num: Vec<usize> = vec![];
            for number in num_str.iter() {
                num.push(number.parse::<usize>().unwrap());
            }

            numbers_list.push(num);
        }
        operations = line.split_whitespace().collect::<Vec<&str>>();
    }

    // parsing input for part 2
    let mut numbers_list_p2: Vec<Vec<usize>> = Vec::with_capacity(operations.len());
    let operation_line = input_str_lists[input_str_lists.len() - 1];
    let number_lines = &input_str_lists[0..input_str_lists.len() - 1];
    let number_line_str_len = operation_line.len();
    let mut number_set_list: Vec<usize> = Vec::with_capacity(number_lines.len());

    for idx in 0..number_line_str_len {
        let current_digits = get_digits(number_lines, idx);
        if current_digits.is_empty() {
            numbers_list_p2.push(number_set_list.clone());
            number_set_list.clear();
        } else {
            let combined_number = current_digits.iter().fold(0, |sig_fig, d| sig_fig * 10 + d);
            number_set_list.push(combined_number);
        }
    }

    if !number_set_list.is_empty() && numbers_list_p2.len() != operations.len() {
        numbers_list_p2.push(number_set_list.clone());
        number_set_list.clear();
    }

    let p1 = calculate_math_p1(&numbers_list, &operations);
    let p2 = calculate_math_p2(&numbers_list_p2, &operations);
    (p1, p2)
}

fn get_digits(number_lines: &[&str], idx: usize) -> Vec<usize> {
    let n_length = number_lines.len();
    let mut digits = Vec::with_capacity(n_length);
    for i in 0..n_length {
        let line = number_lines.get(i).expect("Unable to get line");
        let digit = line.chars().nth(idx);
        match digit {
            Some(d) => {
                if d != ' ' {
                    digits.push(d.to_digit(10).expect("Unable to parse str as a int") as usize)
                }
            }
            None => continue,
        }
    }

    digits
}

fn calculate_math_p2(number_lists: &[Vec<usize>], operations: &Vec<&str>) -> usize {
    let mut sums_list: Vec<usize> = vec![];

    for (idx, operand) in operations.iter().enumerate() {
        if *operand == "+" {
            let mut sum = 0;
            for number in number_lists[idx].iter() {
                sum += number;
            }
            sums_list.push(sum);
        } else if *operand == "*" {
            let mut prod = 1;
            for number in number_lists[idx].iter() {
                prod *= number;
            }
            sums_list.push(prod);
        }
    }

    sums_list.iter().sum()
}

fn calculate_math_p1(number_lists: &[Vec<usize>], operations: &Vec<&str>) -> usize {
    let mut sums_list: Vec<usize> = vec![];

    for (idx, operand) in operations.iter().enumerate() {
        if *operand == "+" {
            let mut sum = 0;
            for num in number_lists.iter() {
                sum += num[idx];
            }
            sums_list.push(sum);
        } else if *operand == "*" {
            let mut prod = 1;
            for num in number_lists.iter() {
                prod *= num[idx];
            }
            sums_list.push(prod);
        }
    }
    sums_list.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_input() {
        let result = trash_compactor("src/aoc_2025/inputs/day_6_ex.txt");

        assert_eq!(result.0, 4277556);
        assert_eq!(result.1, 3263827);
    }
}

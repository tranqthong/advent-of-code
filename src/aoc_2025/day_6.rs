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
    for (ops_idx, operand) in input_str_lists.iter().enumerate().rev() {
        if *operand == "+" || *operand == "*" {
            get_digits(&input_str_lists, &mut numbers_list_p2, ops_idx);
        }

    }

    for (line_idx, line) in input_str_lists.iter().enumerate().rev() {
        println!("{}", line);
        for (char_idx, c) in line.char_indices() {
            let mut next_set = false;
            if c == '+' || c == '*' {
                println!("{}", c);
                next_set = true;
            }
        }
    }

    let p1 = calculate_math_p1(&numbers_list, &operations);
    let p2 = calculate_math_p2(&numbers_list, &operations);
    (p1, p2)
}

fn get_digits(input_str_lists: &[&str], number_lists: &mut [Vec<usize>], current_idx: usize) {
    for 
}

fn seperate_digits(mut num: usize) -> Vec<usize> {
    let mut num_digits: Vec<usize> = vec![];
    if num == 0 {
        num_digits.push(0);
    }

    while num > 0 {
        num_digits.push(num % 10);
        num /= 10;
    }

    num_digits.reverse();
    num_digits
}

fn calculate_math_p2(number_lists: &[Vec<usize>], operations: &Vec<&str>) -> usize {
    let mut sums_list: Vec<usize> = vec![];

    for (idx, operand) in operations.iter().enumerate() {
        let mut sum = 0;
        let mut number_set: Vec<Vec<usize>> = vec![];
        for num_list in number_lists.iter() {
            number_set.push(seperate_digits(num_list[idx]));
        }

        if *operand == "+" {
        } else if *operand == "*" {
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

    #[test]
    fn test_num_digits() {
        let num = 12345 as usize;
        let expected = vec![1, 2, 3, 4, 5];

        let result = seperate_digits(num);

        assert_eq!(result, expected);
    }
}

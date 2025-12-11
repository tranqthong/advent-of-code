use std::fs;

pub fn trash_compactor(input_filepath: &str) -> (usize, usize) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_iter: Vec<&str> = input_content.lines().collect();

    let mut numbers_list: Vec<Vec<usize>> = vec![];
    let mut operations: Vec<&str> = vec![];

    // parsing input for part 1
    for (line_idx, line) in input_iter.iter().enumerate() {
        if line_idx != input_iter.len() - 1 {
            let num_str: Vec<&str> = line.split_whitespace().collect();
            let mut num: Vec<usize> = vec![];
            for number in num_str.iter() {
                num.push(number.parse::<usize>().unwrap());
            }

            numbers_list.push(num);
        }
        operations = line.split_whitespace().collect::<Vec<&str>>();
    }

    // TODO parsing input for part 2

    let p1 = calculate_math_p1(&numbers_list, &operations);
    let p2 = calculate_mate_p2();
    (p1, p2)
}

fn calculate_mate_p2() -> usize {
    0
}

fn calculate_math_p1(number_lists: &Vec<Vec<usize>>, operations: &Vec<&str>) -> usize {
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

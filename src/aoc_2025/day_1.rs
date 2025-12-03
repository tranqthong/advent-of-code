use std::fs;

pub fn secret_password(input_filepath: &str) -> (i32, i32) {
    let file_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let dial_instructions = file_content.lines();

    let dial_instruct = dial_instructions.collect::<Vec<&str>>();

    parse_instructions(&dial_instruct)
}

fn parse_instructions(instructions: &Vec<&str>) -> (i32, i32) {
    let mut dial_pointer = 50;
    let mut end_at_zero_counter = 0;
    let mut zero_click_counter = 0;

    for instruct in instructions {
        let parse_result = parse_instruct(instruct);
        if let Some((spin_direction, spin_count)) = parse_result {
            if spin_direction == 'L' {
                spin_left(&mut dial_pointer, spin_count, &mut zero_click_counter);
            } else if spin_direction == 'R' {
                spin_right(&mut dial_pointer, spin_count, &mut zero_click_counter);
            } else {
                println!("Error in parsing instructions");
            }

            if dial_pointer == 0 {
                end_at_zero_counter += 1;
            }
        }
    }
    (end_at_zero_counter, zero_click_counter)
}

fn parse_instruct(instruct: &str) -> Option<(char, i32)> {
    let mut inter = instruct.chars();
    inter.next().and_then(|c| {
        if c.is_alphabetic() {
            inter.as_str().parse().ok().map(|i| (c, i))
        } else {
            None
        }
    })
}

fn spin_left(dial_pointer: &mut i32, spin_count: i32, zero_counter: &mut i32) {
    // yea... I'm just going to brute force for part 2, haha
    for _i in 0..spin_count {
        *dial_pointer -= 1;
        if *dial_pointer == -1 {
            *dial_pointer = 99;
        }
        if *dial_pointer == 0 {
            *zero_counter += 1;
        }


    }
}

fn spin_right(dial_pointer: &mut i32, spin_count: i32, zero_counter: &mut i32) {
    // yea... I'm just going to brute force for part 2, haha
    for _i in 0..spin_count {
        *dial_pointer += 1;
        if *dial_pointer == 100 {
            *dial_pointer = 0;
        }

        if *dial_pointer == 0 {
            *zero_counter += 1;
        }


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_example_input() {
        let example_input = "src/aoc_2025/inputs/day_1_ex_input.txt";
        let (result, zero_click) = secret_password(&example_input);
        assert_eq!(result, 3);
        assert_eq!(zero_click, 6);
    }

    #[test]
    fn test_spin_left() {
        let mut z_counter = 0;
        let mut dial_ptr = 50;
        spin_left(&mut dial_ptr, 50, &mut z_counter);
        assert_eq!(dial_ptr, 0);
    }

    #[test]
    fn test_sping_left_rollover() {
        let mut z_counter = 0;
        let mut dial_ptr = 50;
        spin_left(&mut dial_ptr, 60, &mut z_counter);
        assert_eq!(dial_ptr, 90);
    }

    #[test]
    fn test_sping_left_rollover_zero_clicks() {
        let mut z_counter = 0;
        let mut dial_ptr = 50;
        spin_left(&mut dial_ptr, 250, &mut z_counter);
        assert_eq!(dial_ptr, 0);
        assert_eq!(z_counter, 3);
    }

    #[test]
    fn test_sping_right() {
        let mut z_counter = 0;
        let mut dial_ptr = 50;
        spin_right(&mut dial_ptr, 50, &mut z_counter);
        assert_eq!(dial_ptr, 0);
    }

    #[test]
    fn test_spring_right_rollover() {
        let mut z_counter = 0;
        let mut dial_ptr = 99;
        spin_right(&mut dial_ptr, 2, &mut z_counter);
        assert_eq!(dial_ptr, 1);
    }

    #[test]
    fn test_spring_right_rollover_zero_clicks() {
        let mut z_counter = 0;
        let mut dial_ptr = 99;
        spin_right(&mut dial_ptr, 2, &mut z_counter);
        assert_eq!(z_counter, 1);
    }
}

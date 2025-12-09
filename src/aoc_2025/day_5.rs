use std::fs;

struct IngredientIdRange {
    start: u64,
    end: u64,
}

pub fn cafeteria(input_filepath: &str) -> u64 {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let (input_ingred_range, input_ingred_ids) = input_content
        .split_once("\n\n")
        .expect("Unable to split on empty line");

    let ingredient_id_range = input_ingred_range
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once("-")
                .expect("Unable to split Ingredient ID range");
            let start = start.parse().expect("Unable to parse start range");
            let end = end.parse().expect("Unable to parse end range");
            IngredientIdRange { start, end }
        })
        .collect::<Vec<IngredientIdRange>>();

    let ingredient_ids = input_ingred_ids
        .lines()
        .map(|line| line.parse().expect("Unable to parse IDs"))
        .collect::<Vec<u64>>();

    is_ingredient_fresh(&ingredient_id_range, &ingredient_ids)
}

fn is_ingredient_fresh(id_range: &Vec<IngredientIdRange>, ids: &Vec<u64>) -> u64 {
    let mut fresh_counter = 0;
    for id in ids {
        for range in id_range {
            if id >= &range.start && id <= &range.end {
                fresh_counter += 1;
                break;
            }
        }
    }

    fresh_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_input() {
        let result = cafeteria("src/aoc_2025/inputs/day_5_ex.txt");

        assert_eq!(result, 3);
    }
}

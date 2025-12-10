use std::{fs, process::id};

struct IngredientIdRange {
    start: u64,
    end: u64,
}

pub fn cafeteria(input_filepath: &str) -> (u64, u64) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let (input_ingred_range, input_ingred_ids) = input_content
        .split_once("\n\n")
        .expect("Unable to split on empty line");

    let mut ingredient_id_range = input_ingred_range
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

    let p1_counter = get_fresh_ingredient_count(&ingredient_id_range, &ingredient_ids);
    let p2_counter = get_all_fresh_id_count(&mut ingredient_id_range);

    (p1_counter, p2_counter)
}

fn get_fresh_ingredient_count(id_range: &Vec<IngredientIdRange>, ids: &Vec<u64>) -> u64 {
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

fn get_all_fresh_id_count(id_ranges: &mut Vec<IngredientIdRange>) -> u64 {
    let mut fresh_counter = 0;

    while let Some(overlapped_range) = find_overlaps(id_ranges) {
        let overlapped_1 = id_ranges.remove(overlapped_range.1);
        let overlapped_2 = id_ranges.remove(overlapped_range.0);

        let new_range_id_start = overlapped_1.start.min(overlapped_2.start);
        let new_range_id_end = overlapped_1.end.max(overlapped_2.end);
        let new_id_range = IngredientIdRange {
            start: new_range_id_start,
            end: new_range_id_end,
        };

        id_ranges.push(new_id_range);
    }

    for range in id_ranges.iter() {
        fresh_counter += range.end - range.start + 1;
    }

    fresh_counter
}

fn find_overlaps(id_range: &[IngredientIdRange]) -> Option<(usize, usize)> {
    for (r1_idx, r1) in id_range.iter().enumerate() {
        for (r2_idx, r2) in id_range.iter().enumerate().skip(r1_idx + 1) {
            if overlaps(r1, r2) {
                return Some((r1_idx, r2_idx));
            }
        }
    }
    None
}

fn overlaps(r1: &IngredientIdRange, r2: &IngredientIdRange) -> bool {
    !(r1.start > r2.end || r1.end < r2.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_input() {
        let result = cafeteria("src/aoc_2025/inputs/day_5_ex.txt");

        assert_eq!(result.0, 3);
        assert_eq!(result.1, 14);
    }

    #[test]
    fn test_overlaps() {
        let r1 = IngredientIdRange { start: 1, end: 10 };
        let r2 = IngredientIdRange { start: 10, end: 14 };

        assert!(overlaps(&r1, &r2));
    }

    #[test]
    fn test_overlaps_2() {
        let r1 = IngredientIdRange { start: 5, end: 15 };
        let r2 = IngredientIdRange { start: 1, end: 6 };

        assert!(overlaps(&r1, &r2));
    }
}

use std::fs;

use itertools::Itertools;

#[derive(Debug)]
struct RedTilePos {
    x: usize,
    y: usize,
}

fn calc_area(a: &RedTilePos, b: &RedTilePos) -> u64 {
    ((a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)) as u64
}

pub fn movie_theater(input_filepath: &str) -> (u64, u64) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_lines: Vec<&str> = input_content.lines().collect();
    let rect_vec: Vec<RedTilePos> = input_lines
        .iter()
        .map(|line| {
            let mut line = line.split(",");
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();
            RedTilePos { x, y }
        })
        .collect();

    let p1_result = part1(&rect_vec);

    (p1_result, 0)
}

fn part1(input: &[RedTilePos]) -> u64 {
    input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| calc_area(a, b))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let result = movie_theater("src/aoc_2025/inputs/day_9_ex.txt");
        assert_eq!(result.0, 50);
    }
}

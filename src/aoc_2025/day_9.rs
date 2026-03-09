use std::fs;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
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
    let p2_result = part2(&rect_vec);

    (p1_result, p2_result)
}

fn part1(input: &[RedTilePos]) -> u64 {
    input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| calc_area(a, b))
        .max()
        .unwrap()
}

fn part2(input: &[RedTilePos]) -> u64 {
    let green_tile_edges: Vec<(RedTilePos, RedTilePos)> =
        input.windows(2).map(|win| (win[0], win[1])).collect();

    let (tile_pos1, tile_pos2) = input
        .iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| calc_area(a, b))
        .rev()
        .find(|(tile_1, tile_2)| {
            let min_x = tile_1.x.min(tile_2.x);
            let min_y = tile_1.y.min(tile_2.y);

            let max_x = tile_1.x.max(tile_2.x);
            let max_y = tile_1.y.max(tile_2.y);

            green_tile_edges.iter().all(|(edge_1, edge_2)| {
                min_x >= edge_1.x.max(edge_2.x)
                    || max_x <= edge_1.x.min(edge_2.x)
                    || min_y >= edge_1.y.max(edge_2.y)
                    || max_y <= edge_1.y.min(edge_2.y)
            })
        })
        .unwrap();

    calc_area(tile_pos1, tile_pos2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let result = movie_theater("src/aoc_2025/inputs/day_9_ex.txt");
        assert_eq!(result.0, 50);
    }

    #[test]
    fn test_part2() {
        let result = movie_theater("src/aoc_2025/inputs/day_9_ex.txt");
        assert_eq!(result.1, 24);
    }
}

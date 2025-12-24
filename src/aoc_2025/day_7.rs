use std::fs;

pub fn laboratories(input_filepath: &str) -> (usize, usize) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let mut input_grid: Vec<Vec<u8>> = input_content
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let split_count = map_beams(&mut input_grid);
    print_grid(&input_grid);
    (split_count, 0)
}

fn print_grid(input_grid: &[Vec<u8>]) {
    for row in input_grid {
        let row_str = std::str::from_utf8(row).unwrap();
        println!("{}", row_str);
    }
}

fn map_beams(grid: &mut [Vec<u8>]) -> usize {
    let mut split_counter = 0;
    let row_length = grid.len();
    for line_idx in 0..row_length {
        let col_length = grid[line_idx].len();
        for c_idx in 0..col_length {
            // check for existing beam if any
            if line_idx != 0 && grid[line_idx - 1][c_idx] == b'|' && grid[line_idx][c_idx] == b'.' {
                grid[line_idx][c_idx] = b'|';
            }
            // initial beam
            if grid[line_idx][c_idx] == b'S' {
                if grid[line_idx + 1][c_idx] == b'.' {
                    grid[line_idx + 1][c_idx] = b'|';
                }
            } else if grid[line_idx][c_idx] == b'^' && grid[line_idx - 1][c_idx] == b'|' {
                if grid[line_idx][c_idx - 1] == b'.' {
                    grid[line_idx][c_idx - 1] = b'|';
                }
                if grid[line_idx][c_idx + 1] == b'.' {
                    grid[line_idx][c_idx + 1] = b'|';
                }
                split_counter += 1;
            }
        }
    }
    split_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_input() {
        let result = laboratories("src/aoc_2025/inputs/day_7_ex.txt");

        assert_eq!(result.0, 21);
    }
}

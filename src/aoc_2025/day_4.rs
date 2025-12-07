use std::fs;

pub fn printing_department(input_filepath: &str) -> (u64, u64) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_iter = input_content.lines();

    let mut paper_grid: Vec<Vec<u8>> = input_iter.map(|line| line.bytes().collect()).collect();

    let part1 = get_total_paper_rolls(&paper_grid);
    let part2 = get_removable_paper_rolls(&mut paper_grid);

    (part1, part2)
}

fn get_total_paper_rolls(paper_grid: &Vec<Vec<u8>>) -> u64 {
    let mut rollable_paper_count = 0;

    for (r_idx, row) in paper_grid.iter().enumerate() {
        for (c_idx, grid_element) in row.iter().enumerate() {
            if grid_element == &b'@' {
                let mut neighbor_count = 0;
                for (row, col) in neighbor_iterator(r_idx, c_idx) {
                    if let Some(row) = paper_grid.get(row)
                        && let Some(grid_element) = row.get(col)
                        && grid_element == &b'@'
                    {
                        neighbor_count += 1;
                    }
                }
                if neighbor_count < 5 {
                    rollable_paper_count += 1;
                }
            }
        }
    }
    rollable_paper_count
}

fn get_removable_paper_rolls(paper_grid: &mut Vec<Vec<u8>>) -> u64 {
    let mut removable_paper_count = 0;

    let mut papers_rolls_removal_list: Vec<(usize, usize)> = Vec::new();
    loop {
        if papers_rolls_removal_list.is_empty() {
            break;
        }
    }
    for (r_idx, row) in paper_grid.iter().enumerate() {
        for (c_idx, grid_element) in row.iter().enumerate() {
            if grid_element == &b'@' {
                let mut neighbor_count = 0;
                for (row, col) in neighbor_iterator(r_idx, c_idx) {
                    if let Some(row) = paper_grid.get(row)
                        && let Some(grid_element) = row.get(col)
                        && grid_element == &b'@'
                    {
                        neighbor_count += 1;
                    }
                    if neighbor_count < 5 {
                        papers_rolls_removal_list.push((r_idx, c_idx));
                    }
                }
            }
        }
    }

    while let Some((row, col)) = papers_rolls_removal_list.pop() {
        if paper_grid[row][col] == b'@' {
            paper_grid[row][col] = b'.';
            removable_paper_count += 1;
        }
    }

    removable_paper_count
}

fn neighbor_iterator(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    let neighbors = [
        (row.wrapping_sub(1), col.wrapping_sub(1)),
        (row.wrapping_sub(1), col),
        (row.wrapping_sub(1), col.wrapping_add(1)),
        (row, col.wrapping_sub(1)),
        (row, col),
        (row, col.wrapping_add(1)),
        (row.wrapping_add(1), col.wrapping_sub(1)),
        (row.wrapping_add(1), col),
        (row.wrapping_add(1), col.wrapping_add(1)),
    ];
    neighbors.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_input() {
        let result = printing_department("src/aoc_2025/inputs/day_4_ex.txt");

        assert_eq!(result.0, 13);
        assert_eq!(result.1, 43);
    }
}

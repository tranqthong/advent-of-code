use std::fs;

struct BoxCoordinates {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for BoxCoordinates {
    fn from(xyz: &str) -> Self {
        let mut xyz_coordinates = xyz
            .split(',')
            .map(|coordinates| coordinates.parse().unwrap());

        BoxCoordinates {
            x: xyz_coordinates.next().unwrap(),
            y: xyz_coordinates.next().unwrap(),
            z: xyz_coordinates.next().unwrap(),
        }
    }
}

impl BoxCoordinates {
    fn euclidean_distance(&self, another: &Self) -> usize {
        ((another.x - self.x).pow(2) + (another.y - self.y).pow(2) + (another.z - self.z).pow(2))
            .isqrt()
    }
}

pub fn playground(input_filepath: &str) -> (usize, usize) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let junction_boxes: Vec<BoxCoordinates> = input_content.lines().map(BoxCoordinates::from).collect();

    let part1_result = multiply_three_largest_circuits(&junction_boxes);

    (part1_result, 0)
}

fn multiply_three_largest_circuits(junction_boxes: &[BoxCoordinates]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex_input() {
        let result = playground("src/aoc_2025/inputs/day_8_ex.txt");

        assert_eq!(result.0, 40);
    }
}
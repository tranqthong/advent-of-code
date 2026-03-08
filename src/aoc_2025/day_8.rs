use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use disjoint::DisjointSetVec;
use itertools::Itertools;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionCoordinates {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionCoordinates {
    fn not_euclidean_distance(&self, other: &JunctionCoordinates) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

pub fn playground(input_filepath: &str, num_pairs: usize) -> (usize, u64) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_lines: Vec<&str> = input_content.lines().collect();
    let junction_boxes: Vec<JunctionCoordinates> = build_junction_boxes(&input_lines);

    let p1_result = solve_p1(&junction_boxes, num_pairs);
    let p2_result = solve_p2(&junction_boxes);
    (p1_result, p2_result)
}

fn build_junction_boxes(lines: &[impl AsRef<str>]) -> Vec<JunctionCoordinates> {
    lines
        .iter()
        .map(|line| {
            let mut line = line.as_ref().split(",");
            let x = line.next().unwrap().parse::<u64>().unwrap();
            let y = line.next().unwrap().parse::<u64>().unwrap();
            let z = line.next().unwrap().parse::<u64>().unwrap();
            JunctionCoordinates { x, y, z }
        })
        .collect()
}

fn solve_p1(junction_boxes: &[JunctionCoordinates], num_pairs: usize) -> usize {
    let mut dsu_jbox: DisjointSetVec<JunctionCoordinates> = DisjointSetVec::from(junction_boxes);

    let mut distance_btree: BTreeMap<u64, (usize, usize)> = BTreeMap::new();

    for (idx, box_i) in junction_boxes.iter().enumerate() {
        for (jdx, box_j) in junction_boxes.iter().enumerate() {
            if idx != jdx {
                let jbox_distance = box_i.not_euclidean_distance(box_j);
                distance_btree.entry(jbox_distance).or_insert((idx, jdx));
            }
        }
    }

    let mut count = 0;
    for (_tree_idx, (idx, jdx)) in distance_btree.iter() {
        dsu_jbox.join(*idx, *jdx);
        count += 1;

        if count == num_pairs {
            break;
        }
    }

    let mut circuit_size: HashMap<usize, usize> = HashMap::new();

    for idx in 0..dsu_jbox.len() {
        let root = dsu_jbox.root_of(idx);
        *circuit_size.entry(root).or_insert(0) += 1;
    }

    circuit_size
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .take(3)
        .map(|(_, count)| count)
        .product()
}

fn solve_p2(junction_boxes: &[JunctionCoordinates]) -> u64 {
    let mut dsu_jbox: DisjointSetVec<JunctionCoordinates> = DisjointSetVec::from(junction_boxes);

    let mut distance_btree: BTreeMap<u64, (usize, usize)> = BTreeMap::new();

    for (idx, box_i) in junction_boxes.iter().enumerate() {
        for (jdx, box_j) in junction_boxes.iter().enumerate() {
            if idx != jdx {
                let jbox_distance = box_i.not_euclidean_distance(box_j);
                distance_btree.entry(jbox_distance).or_insert((idx, jdx));
            }
        }
    }

    let mut joined_set_unions: [(usize, usize); 1] = [(0, 1)];

    for (_, (idx, jdx)) in distance_btree.iter() {
        if dsu_jbox.join(*idx, *jdx) {
            joined_set_unions[0] = (*idx, *jdx);
        }
    }

    junction_boxes[joined_set_unions[0].0].x * junction_boxes[joined_set_unions[0].1].x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let num_pairs = 1000;
        let result = playground("src/aoc_2025/inputs/day_8_ex.txt", num_pairs);

        assert_eq!(result.0, 40);
    }

    #[test]
    fn test_part2() {
        let num_pairs = 1000;
        let result = playground("src/aoc_2025/inputs/day_8_ex.txt", num_pairs);

        assert_eq!(result.1, 25272);
    }
}

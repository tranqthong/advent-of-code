use std::{fs, mem::swap};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionCoordinates {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionCoordinates {
    fn not_euclidean_distance(&self, other: &JunctionCoordinates) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct Node {
    parent: usize,
    size: usize,
}

struct DisjointSetUnion {
    nodes: Vec<Node>,
}

impl DisjointSetUnion {
    fn new(length: usize) -> Self {
        Self {
            nodes: (0..length).map(|parent| Node { parent, size: 1 }).collect(),
        }
    }

    fn parent(&mut self, mut idx: usize) -> usize {
        let mut parent = self.nodes[idx].parent;
        while parent != idx {
            self.nodes[idx].parent = self.nodes[parent].parent;
            idx = parent;
            parent = self.nodes[idx].parent;
        }
        parent
    }

    fn add_circuit_pair(&mut self, a: usize, b: usize) {
        let mut parent_a = self.parent(a);
        let mut parent_b = self.parent(b);

        if parent_a == parent_b {
            return;
        } else if self.nodes[parent_a].size < self.nodes[parent_b].size {
            swap(&mut parent_a, &mut parent_b);
        }
        self.nodes[parent_b].parent = parent_a;
        self.nodes[parent_a].size += self.nodes[parent_b].size;
    }
}

pub fn playground(input_filepath: &str, num_pairs: usize) -> (usize, usize) {
    let input_content = fs::read_to_string(input_filepath).expect("Unable to read file");
    let input_lines: Vec<&str> = input_content.lines().collect();
    let junction_boxes: Vec<JunctionCoordinates> = build_junction_boxes(&input_lines);

    let circuit_pairs = build_closest_circuit_pairs(&junction_boxes, num_pairs);
    let p2_circuit_pairs = build_closest_circuit_pairs(&junction_boxes, 5000);

    let p1_result = solve_p1(&circuit_pairs, num_pairs);
    let p2_result = solve_p2(&junction_boxes, &p2_circuit_pairs, num_pairs);
    (p1_result, p2_result)
}

fn build_junction_boxes(lines: &[impl AsRef<str>]) -> Vec<JunctionCoordinates> {
    lines
        .iter()
        .map(|line| {
            let mut line = line.as_ref().split(",");
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();
            let z = line.next().unwrap().parse::<usize>().unwrap();
            JunctionCoordinates { x, y, z }
        })
        .collect()
}

fn build_closest_circuit_pairs(
    junction_boxes: &[JunctionCoordinates],
    num_pairs: usize,
) -> Vec<(usize, usize, usize)> {
    let mut circuit_pairs: Vec<(usize, usize, usize)> = Vec::with_capacity(num_pairs);

    let mut combo_iter = (0..junction_boxes.len())
        .flat_map(|idx| (idx + 1..junction_boxes.len()).map(move |jdx| (idx, jdx)));

    for _i in 0..num_pairs {
        if let Some((a, b)) = combo_iter.next() {
            let distance = junction_boxes[a].not_euclidean_distance(&junction_boxes[b]);
            circuit_pairs.push((a, b, distance));
        }
    }
    circuit_pairs.sort_unstable_by_key(|(_, _, distance)| *distance);
    for (a, b) in combo_iter {
        let distance = junction_boxes[a].not_euclidean_distance(&junction_boxes[b]);
        if distance <= circuit_pairs.last().unwrap().2 {
            circuit_pairs.pop();
            let new_idx = circuit_pairs.partition_point(|(_, _, d)| d < &distance);
            circuit_pairs.insert(new_idx, (a, b, distance));
        }
    }
    circuit_pairs
}

fn solve_p1(circuit_pairs: &[(usize, usize, usize)], num_pairs: usize) -> usize {
    let mut circuit_tree = DisjointSetUnion::new(num_pairs);

    for (a, b, _) in circuit_pairs {
        circuit_tree.add_circuit_pair(*a, *b);
    }

    circuit_tree.nodes.sort_by_key(|node| node.size);

    circuit_tree
        .nodes
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.size)
        .product()
}

fn solve_p2(
    junction_boxes: &[JunctionCoordinates],
    circuit_pairs: &[(usize, usize, usize)],
    num_pairs: usize,
) -> usize {
    let mut circuit_tree = DisjointSetUnion::new(num_pairs);

    let mut last_a = 0;
    let mut last_b = 0;
    for (a, b, _) in circuit_pairs {
        circuit_tree.add_circuit_pair(*a, *b);
        last_a = *a;
        last_b = *b;
    }

    junction_boxes[last_a].x * junction_boxes[last_b].x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex_input() {
        let result = playground("src/aoc_2025/inputs/day_8_ex.txt", 10);

        assert_eq!(result.0, 40);
        assert_eq!(result.1, 25272);
    }
}

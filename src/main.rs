mod aoc_2024;
mod aoc_2025;

fn main() {
    println!("Hello, world!");
    let (zero_stop, zero_click) =
        crate::aoc_2025::day_1::secret_password("src/aoc_2025/inputs/day_1_input.txt");
    println!("Stop at Zero count: {}", zero_stop);
    println!("Zero Click count: {}", zero_click);
}

mod aoc_2024;
mod aoc_2025;

fn main() {
    println!("Hello, world!");
    let (zero_stop, zero_click) =
        crate::aoc_2025::day_1::secret_password("src/aoc_2025/inputs/day_1_input.txt");
    println!("Stop at Zero count: {}", zero_stop);
    println!("Zero Click count: {}", zero_click);

    let invalid_sum = crate::aoc_2025::day_2::gift_shop("src/aoc_2025/inputs/day_2_input.txt");
    println!("Invalid ID sum part 1: {}", invalid_sum.0);
    println!("Invalid ID sum part 2: {}", invalid_sum.1);
}

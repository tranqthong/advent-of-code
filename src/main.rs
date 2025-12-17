mod aoc_2025;

fn main() {
    println!("Hello, world!");
    day_6();
}

#[allow(unused)]
fn day_6() {
    let grand_total = crate::aoc_2025::day_6::trash_compactor("src/aoc_2025/inputs/day_6.txt");
    println!("Day 6 part 1: {}", grand_total.0);
    println!("Day 6 part 2: {}", grand_total.1);
}

#[allow(unused)]
fn day_5() {
    let fresh_ingredients = crate::aoc_2025::day_5::cafeteria("src/aoc_2025/inputs/day_5.txt");
    println!("Day 5 part 1: {}", fresh_ingredients.0);
    println!("Day 5 part 2: {}", fresh_ingredients.1);
}

#[allow(unused)]
fn day_4() {
    let paper_rolls = crate::aoc_2025::day_4::printing_department("src/aoc_2025/inputs/day_4.txt");
    println!("Day 4 part 1: {}", paper_rolls.0);
    println!("Day 4 part 2: {}", paper_rolls.1);
}

#[allow(unused)]
fn day_3() {
    let total_joltage = crate::aoc_2025::day_3::lobby("src/aoc_2025/inputs/day_3_input.txt");
    println!("Total output joltage part 1: {}", total_joltage.0);
    println!("Total output joltage part 2: {}", total_joltage.1);
}

#[allow(unused)]
fn day_2() {
    let invalid_sum = crate::aoc_2025::day_2::gift_shop("src/aoc_2025/inputs/day_2_input.txt");
    println!("Invalid ID sum part 1: {}", invalid_sum.0);
    println!("Invalid ID sum part 2: {}", invalid_sum.1);
}

#[allow(unused)]
fn day_1() {
    let (zero_stop, zero_click) =
        crate::aoc_2025::day_1::secret_password("src/aoc_2025/inputs/day_1_input.txt");
    println!("Stop at Zero count: {}", zero_stop);
    println!("Zero Click count: {}", zero_click);
}

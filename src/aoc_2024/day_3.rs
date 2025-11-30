use regex::Regex;

fn day_three(instruct: &str) -> i32 {
    let mut mul_total = 0;
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for mul in mul_regex.captures_iter(instruct) {
        let x: i32 = mul[1].parse().expect("incorrect digits");
        let y: i32 = mul[2].parse().expect("incorrect digits");

        mul_total += x * y;
    }
    mul_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_three() {
        let test_instruct =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = day_three(test_instruct);

        assert_eq!(result, 161);
    }
}

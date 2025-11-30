fn day_two(data: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    for lvl in data {
        let result = is_all_reports_safe(lvl);
        if result {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn is_all_reports_safe(report: &[i32]) -> bool {
    if !is_within_range(report) {
        return false;
    }

    if !is_increasing(report) && !is_decreasing(report) {
        return false;
    }
    true
}

fn is_decreasing(report: &[i32]) -> bool {
    for i in 0..(report.len() - 1) {
        if report[i] < report[i + 1] {
            return false;
        }
    }
    true
}

fn is_increasing(report: &[i32]) -> bool {
    for i in 0..(report.len() - 1) {
        if report[i] > report[i + 1] {
            return false;
        }
    }
    true
}

fn is_within_range(report: &[i32]) -> bool {
    for i in 0..(report.len() - 1) {
        let diff = (report[i] - report[i + 1]).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_two() {
        let ex_data = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let expected_result = 2;

        let result = day_two(&ex_data);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_decreasing_success() {
        let test_report = vec![5, 4, 3, 2, 1];
        let result = is_decreasing(&test_report);

        assert!(result);
    }

    #[test]
    fn test_decreasing_fail() {
        let test_report = vec![5, 4, 7, 2, 1];
        let result = is_decreasing(&test_report);

        assert!(!result);
    }

    #[test]
    fn test_increasing_success() {
        let test_report = vec![1, 2, 3, 6, 8];
        let result = is_increasing(&test_report);

        assert!(result);
    }

    #[test]
    fn test_increasing_fail() {
        let test_report = vec![10, 8, 7, 6, 5, 4, 9];
        let result = is_increasing(&test_report);

        assert!(!result);
    }
}

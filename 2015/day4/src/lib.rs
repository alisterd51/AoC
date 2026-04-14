#[must_use]
pub fn solve_part_1(secret_key: &str) -> Option<u64> {
    for answer in 0..u64::MAX {
        let hash = md5::compute(format!("{secret_key}{answer}"));
        let hash = format!("{hash:x}");
        if hash.starts_with("00000") {
            return Some(answer);
        }
    }

    None
}

#[must_use]
pub fn solve_part_2(secret_key: &str) -> Option<u64> {
    for answer in 0..u64::MAX {
        let hash = md5::compute(format!("{secret_key}{answer}"));
        let hash = format!("{hash:x}");
        if hash.starts_with("000000") {
            return Some(answer);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from("abcdef");
        let result = solve_part_1(&input);
        assert_eq!(result, Some(609043));
        let input = String::from("pqrstuv");
        let result = solve_part_1(&input);
        assert_eq!(result, Some(1048970));
    }

    #[test]
    fn example_solve_part_2() {}
}

#[must_use]
pub fn parse_input(input: &str) -> Option<u64> {
    input.parse::<u64>().ok()
}

fn get_divisors(n: u64) -> Vec<u64> {
    let mut divisors = vec![];

    for divisor in 1..=n.isqrt() {
        if n.is_multiple_of(divisor) {
            divisors.push(divisor);

            let pair = n / divisor;
            if divisor != pair {
                divisors.push(pair);
            }
        }
    }
    divisors
}

#[must_use]
pub fn solve_part_1(min_presents: u64) -> u64 {
    let mut house_id = 0;
    let mut presents = 0;

    while presents < min_presents {
        house_id += 1;
        presents = get_divisors(house_id).iter().sum::<u64>() * 10;
    }

    house_id
}

#[must_use]
pub fn solve_part_2(min_presents: u64) -> u64 {
    let mut house_id = 0;
    let mut presents = 0;

    while presents < min_presents {
        house_id += 1;
        presents = 0;
        for divisor in get_divisors(house_id) {
            if house_id <= divisor * 50 {
                presents += divisor * 11;
            }
        }
    }

    house_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::unwrap_used)]
    #[test]
    fn example_solve_part_1() {
        let input = String::from("10");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 1);
        let input = String::from("20");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 2);
        let input = String::from("30");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 2);
        let input = String::from("40");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 3);
        let input = String::from("50");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 4);
        let input = String::from("60");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 4);
        let input = String::from("70");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 4);
        let input = String::from("80");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 6);
        let input = String::from("90");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 6);
        let input = String::from("100");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 6);
        let input = String::from("110");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 6);
        let input = String::from("120");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 6);
        let input = String::from("130");
        let input = parse_input(&input).unwrap();
        let result = solve_part_1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_solve_part_2() {}
}

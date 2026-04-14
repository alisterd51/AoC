#[must_use]
pub fn parse_directions(input: &str) -> Vec<i64> {
    let mut directions = vec![];
    for c in input.chars() {
        match c {
            '(' => directions.push(1),
            ')' => directions.push(-1),
            _ => {}
        }
    }

    directions
}

#[must_use]
pub fn solve_part_1(directions: &[i64]) -> i64 {
    let mut floor = 0;
    for direction in directions {
        floor += direction;
    }

    floor
}

#[must_use]
pub fn solve_part_2(directions: &[i64]) -> Option<usize> {
    let mut floor = 0;
    for (position, direction) in directions.iter().enumerate() {
        floor += direction;
        if floor == -1 {
            return Some(position + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "(())";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = "()()";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 0);
        let input = "(((";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 3);
        let input = "(()(()(";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 3);
        let input = "))(((((";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 3);
        let input = "())";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, -1);
        let input = "))(";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, -1);
        let input = ")))";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, -3);
        let input = ")())())";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, -3);
    }

    #[test]
    fn example_solve_part_2() {
        let input = ")";
        let input = parse_directions(input);
        let result = solve_part_2(&input);
        assert_eq!(result, Some(1));
        let input = "()())";
        let input = parse_directions(input);
        let result = solve_part_2(&input);
        assert_eq!(result, Some(5));
    }
}

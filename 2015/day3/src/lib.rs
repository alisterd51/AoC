use std::collections::HashSet;

pub enum Direction {
    North,
    South,
    East,
    West,
}

#[must_use]
pub fn parse_directions(input: &str) -> Vec<Direction> {
    let mut directions = vec![];
    for line in input.lines() {
        for c in line.chars() {
            let direction = match c {
                '^' => Direction::North,
                'v' => Direction::South,
                '>' => Direction::East,
                '<' => Direction::West,
                _ => continue,
            };
            directions.push(direction);
        }
    }

    directions
}

#[must_use]
pub fn solve_part_1(directions: &[Direction]) -> usize {
    let mut houses: HashSet<(i64, i64)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    houses.insert((x, y));

    for direction in directions {
        match direction {
            Direction::North => y += 1,
            Direction::South => y -= 1,
            Direction::East => x += 1,
            Direction::West => x -= 1,
        }
        houses.insert((x, y));
    }

    houses.len()
}

#[must_use]
pub fn solve_part_2(directions: &[Direction]) -> usize {
    let mut houses: HashSet<(i64, i64)> = HashSet::new();

    for n in 0..=1 {
        let mut x = 0;
        let mut y = 0;
        houses.insert((x, y));

        for direction in directions.iter().skip(n).step_by(2) {
            match direction {
                Direction::North => y += 1,
                Direction::South => y -= 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }
            houses.insert((x, y));
        }
    }

    houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = ">";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 2);
        let input = "^>v<";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 4);
        let input = "^v^v^v^v^v";
        let input = parse_directions(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_solve_part_2() {
        let input = "^v";
        let input = parse_directions(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 3);
        let input = "^>v<";
        let input = parse_directions(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 3);
        let input = "^v^v^v^v^v";
        let input = parse_directions(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 11);
    }
}

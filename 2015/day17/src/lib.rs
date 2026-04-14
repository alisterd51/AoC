use itertools::Itertools;

#[must_use]
pub fn parse_containers(input: &str) -> Vec<usize> {
    let mut containers = vec![];

    for line in input.lines() {
        if let Ok(container) = line.parse::<usize>() {
            containers.push(container);
        }
    }

    containers
}

#[must_use]
pub fn solve_part_1(containers: &[usize], liters: usize) -> usize {
    let mut solutions = 0;

    for combination in containers.iter().copied().powerset() {
        if combination.iter().sum::<usize>() == liters {
            solutions += 1;
        }
    }

    solutions
}

#[must_use]
pub fn solve_part_2(containers: &[usize], liters: usize) -> usize {
    let mut min_containers = usize::MAX;
    let mut solutions = 0;

    for combination in containers.iter().copied().powerset() {
        if combination.iter().sum::<usize>() == liters {
            if combination.len() < min_containers {
                min_containers = combination.len();
                solutions = 1;
            } else if combination.len() == min_containers {
                solutions += 1;
            }
        }
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "20
15
10
5
5",
        );
        let input = parse_containers(&input);
        let result = solve_part_1(&input, 25);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from(
            "20
15
10
5
5",
        );
        let input = parse_containers(&input);
        let result = solve_part_2(&input, 25);
        assert_eq!(result, 3);
    }
}

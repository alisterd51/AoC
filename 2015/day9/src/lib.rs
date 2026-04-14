use itertools::Itertools;
use std::collections::HashMap;

pub struct Path {
    a: String,
    b: String,
    distance: u64,
}

#[must_use]
pub fn parse_distances(input: &str) -> Vec<Path> {
    let mut paths = vec![];

    for line in input.lines() {
        if let Some((ab, distance)) = line.split_once(" = ")
            && let Some((a, b)) = ab.split_once(" to ")
            && let Ok(distance) = distance.parse::<u64>()
        {
            paths.push(Path {
                a: a.to_string(),
                b: b.to_string(),
                distance,
            });
        }
    }

    paths
}

fn build_graph(paths: &[Path]) -> HashMap<String, Vec<(String, u64)>> {
    let mut graph: HashMap<String, Vec<(String, u64)>> = HashMap::new();
    for path in paths {
        graph
            .entry(path.a.clone())
            .or_default()
            .push((path.b.clone(), path.distance));
        graph
            .entry(path.b.clone())
            .or_default()
            .push((path.a.clone(), path.distance));
    }

    graph
}

fn shortest_path(graph: &HashMap<String, Vec<(String, u64)>>) -> u64 {
    let nodes: Vec<String> = graph.keys().cloned().collect();
    let num_nodes = nodes.len();
    if num_nodes <= 1 {
        return 0;
    }
    let mut min_distance = u64::MAX;

    for path in nodes.into_iter().permutations(num_nodes) {
        let mut current_distance = 0;
        let mut is_valid_path = true;
        for window in path.windows(2) {
            let current_node = &window[0];
            let next_node = &window[1];
            let edge = graph
                .get(current_node)
                .and_then(|neighbors| neighbors.iter().find(|(n, _)| n == next_node));

            if let Some((_, dist)) = edge {
                current_distance += dist;
            } else {
                is_valid_path = false;
                break;
            }
        }
        if is_valid_path && current_distance < min_distance {
            min_distance = current_distance;
        }
    }
    min_distance
}

fn longest_path(graph: &HashMap<String, Vec<(String, u64)>>) -> u64 {
    let nodes: Vec<String> = graph.keys().cloned().collect();
    let num_nodes = nodes.len();
    if num_nodes <= 1 {
        return 0;
    }
    let mut max_distance = u64::MIN;

    for path in nodes.into_iter().permutations(num_nodes) {
        let mut current_distance = 0;
        let mut is_valid_path = true;
        for window in path.windows(2) {
            let current_node = &window[0];
            let next_node = &window[1];
            let edge = graph
                .get(current_node)
                .and_then(|neighbors| neighbors.iter().find(|(n, _)| n == next_node));

            if let Some((_, dist)) = edge {
                current_distance += dist;
            } else {
                is_valid_path = false;
                break;
            }
        }
        if is_valid_path && current_distance > max_distance {
            max_distance = current_distance;
        }
    }
    max_distance
}

#[must_use]
pub fn solve_part_1(paths: &[Path]) -> u64 {
    let graph = build_graph(paths);
    shortest_path(&graph)
}

#[must_use]
pub fn solve_part_2(paths: &[Path]) -> u64 {
    let graph = build_graph(paths);
    longest_path(&graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = String::from(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        );
        let input = parse_distances(&input);
        let result = solve_part_1(&input);
        assert_eq!(result, 605);
    }

    #[test]
    fn example_solve_part_2() {
        let input = String::from(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        );
        let input = parse_distances(&input);
        let result = solve_part_2(&input);
        assert_eq!(result, 982);
    }
}

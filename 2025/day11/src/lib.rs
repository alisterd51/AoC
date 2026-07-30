use std::collections::HashMap;

pub struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

struct FastGraph {
    adj: Vec<Vec<usize>>,
    names: Vec<String>,
    indices: HashMap<String, usize>,
}

impl Graph {
    #[must_use]
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, node: &str, connected_node: &str) {
        self.adjacency_list
            .entry(node.to_string())
            .or_default()
            .push(connected_node.to_string());
        self.adjacency_list
            .entry(connected_node.to_string())
            .or_default();
    }

    fn optimize(&self) -> FastGraph {
        let mut names = Vec::new();
        let mut indices = HashMap::new();

        for key in self.adjacency_list.keys() {
            indices.insert(key.clone(), names.len());
            names.push(key.clone());
        }

        let mut adj = vec![Vec::new(); names.len()];

        for (node, neighbors) in &self.adjacency_list {
            let node_index = indices[node];

            for neighbor in neighbors {
                if let Some(&neighbor_indice) = indices.get(neighbor) {
                    adj[node_index].push(neighbor_indice);
                }
            }
        }

        FastGraph {
            adj,
            names,
            indices,
        }
    }

    #[must_use]
    pub fn count_paths_via(&self, start: &str, end: &str, waypoints: Vec<&str>) -> u128 {
        let graph = self.optimize();
        let mut path_points = Vec::new();

        if let Some(&id) = graph.indices.get(start) {
            path_points.push(id);
        } else {
            return 0;
        }
        for waypoint in waypoints {
            if let Some(&id) = graph.indices.get(waypoint) {
                path_points.push(id);
            } else {
                return 0;
            }
        }
        if let Some(&id) = graph.indices.get(end) {
            path_points.push(id);
        } else {
            return 0;
        }

        let mut total_combinations: u128 = 1;

        for i in 0..path_points.len() - 1 {
            let current = path_points[i];
            let target = path_points[i + 1];
            let mut memo = vec![None; graph.names.len()];
            let segment_count = graph.count_paths_dp(current, target, &mut memo);

            if segment_count == 0 {
                return 0;
            }
            total_combinations *= segment_count;
        }

        total_combinations
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl FastGraph {
    fn count_paths_dp(&self, current: usize, target: usize, memo: &mut Vec<Option<u128>>) -> u128 {
        if current == target {
            return 1;
        }
        if let Some(res) = memo[current] {
            return res;
        }

        let mut count: u128 = 0;

        for &neighbor in &self.adj[current] {
            count = count.saturating_add(self.count_paths_dp(neighbor, target, memo));
        }
        memo[current] = Some(count);

        count
    }
}

#[must_use]
pub fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();

    for line in input.lines() {
        if let Some((node, connected_nodes)) = line.split_once(':') {
            for connected_node in connected_nodes.split_whitespace() {
                graph.add_edge(node, connected_node);
            }
        }
    }

    graph
}

#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn solve_part_1(graph: &Graph) -> u64 {
    graph.count_paths_via("you", "out", vec![]) as u64
}

#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn solve_part_2(graph: &Graph) -> u64 {
    graph.count_paths_via("svr", "out", vec!["fft", "dac"]) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let input = parse_graph(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_solve_part_2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let input = parse_graph(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 2);
    }
}

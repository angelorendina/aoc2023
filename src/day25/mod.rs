use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day25/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day25/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut connections = HashMap::<&str, HashSet<&str>>::new();
    for line in input.lines() {
        let (from, tos) = line.split_once(':').unwrap();
        for to in tos.split_whitespace() {
            connections.entry(from).or_default().insert(to);
            connections.entry(to).or_default().insert(from);
        }
    }

    let mut total_edge_weight = HashMap::<(&str, &str), usize>::new();
    for &start in connections.keys() {
        let mut visited = HashSet::<&str>::new();
        let mut edge_weight = HashMap::<(&str, &str), usize>::new();
        let mut queue = VecDeque::<(&str, usize)>::from([(start, 0)]);
        while let Some((node, w)) = queue.pop_front() {
            visited.insert(node);
            for &next in &connections[node] {
                if visited.contains(next) {
                    continue;
                }
                let entry = if node < next {
                    (node, next)
                } else {
                    (next, node)
                };
                let old_w = edge_weight.entry(entry).or_insert(usize::MAX);
                *old_w = (*old_w).min(w);
                queue.push_back((next, w + 1));
            }
        }
        for (e, w) in edge_weight {
            *total_edge_weight.entry(e).or_default() += w;
        }
    }

    let mut bridges = total_edge_weight.into_iter().collect::<Vec<_>>();
    bridges.sort_unstable_by_key(|(_, c)| *c);

    // bold assumption that the three bridges that need removing insist on 3 different
    // pairs of verticies, but at this point I know it's true and can't be bothered anymore
    let mut separated_nodes = HashSet::new();
    for ((a, b), _) in bridges {
        if separated_nodes.len() == 6 {
            break;
        }
        if separated_nodes.contains(a) || separated_nodes.contains(b) {
            continue;
        }
        separated_nodes.insert(a);
        separated_nodes.insert(b);
        connections.get_mut(a).unwrap().remove(b);
        connections.get_mut(b).unwrap().remove(a);
    }

    let mut visited = HashSet::<&str>::new();
    let mut queue = VecDeque::<&str>::from([connections.keys().next().copied().unwrap()]);
    while let Some(node) = queue.pop_front() {
        visited.insert(node);
        for &next in &connections[node] {
            if visited.contains(next) {
                continue;
            }
            queue.push_back(next);
        }
    }

    visited.len() * (connections.len() - visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 54);
    }
}

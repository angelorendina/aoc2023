use std::collections::HashMap;
use std::collections::VecDeque;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day20/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day20/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let mut incoming_connections = HashMap::<&str, Vec<&str>>::new();
    let mut circuit = input
        .lines()
        .map(|line| {
            let (id, to) = line.split_once(" -> ").unwrap();
            let to = to.split(", ").collect::<Vec<_>>();
            let (id, node) = match id {
                "broadcaster" => ("broadcaster", Node::Broadcaster { to }),
                flipflop if flipflop.starts_with('%') => (
                    flipflop.strip_prefix('%').unwrap(),
                    Node::FlipFlop { state: false, to },
                ),
                conjunction if conjunction.starts_with('&') => (
                    conjunction.strip_prefix('&').unwrap(),
                    Node::Conjunction {
                        from: HashMap::new(),
                        to,
                    },
                ),
                untyped => (untyped, Node::Untyped),
            };

            // record separately all connections x -> y
            // so we can populate the conjunctions
            match &node {
                Node::Broadcaster { to }
                | Node::FlipFlop { to, .. }
                | Node::Conjunction { to, .. } => {
                    for to in to {
                        incoming_connections.entry(to).or_default().push(id);
                    }
                }
                Node::Untyped => {}
            }

            (id, node)
        })
        .collect::<HashMap<_, _>>();

    // populate `from` field of conjunctions
    for (id, inputs) in incoming_connections {
        if let Node::Conjunction { from, .. } = circuit.entry(id).or_insert(Node::Untyped) {
            for input in inputs {
                from.insert(input, false);
            }
        }
    }

    let mut highs = 0;
    let mut lows = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from([("", "broadcaster", false)]);
        while let Some((id_from, id_to, is_high)) = queue.pop_front() {
            if is_high {
                highs += 1;
            } else {
                lows += 1;
            };
            match circuit.get_mut(id_to).unwrap() {
                Node::Broadcaster { to } => {
                    for to in to {
                        queue.push_back((id_to, to, is_high));
                    }
                }
                Node::FlipFlop { state, to } => {
                    if !is_high {
                        *state ^= true;
                        for to in to {
                            queue.push_back((id_to, to, *state));
                        }
                    }
                }
                Node::Conjunction { from, to } => {
                    *from.get_mut(id_from).unwrap() = is_high;
                    let all_highs_in = from.values().all(|h| *h);
                    for to in to {
                        queue.push_back((id_to, to, !all_highs_in));
                    }
                }
                Node::Untyped => {}
            }
        }
    }

    highs * lows
}

pub fn star_two() -> usize {
    const FILENAME: &str = "src/day20/input.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let mut incoming_connections = HashMap::<&str, Vec<&str>>::new();
    let mut circuit = input
        .lines()
        .map(|line| {
            let (id, to) = line.split_once(" -> ").unwrap();
            let to = to.split(", ").collect::<Vec<_>>();
            let (id, node) = match id {
                "broadcaster" => ("broadcaster", Node::Broadcaster { to }),
                flipflop if flipflop.starts_with('%') => (
                    flipflop.strip_prefix('%').unwrap(),
                    Node::FlipFlop { state: false, to },
                ),
                conjunction if conjunction.starts_with('&') => (
                    conjunction.strip_prefix('&').unwrap(),
                    Node::Conjunction {
                        from: HashMap::new(),
                        to,
                    },
                ),
                untyped => (untyped, Node::Untyped),
            };

            // record separately all connections x -> y
            // so we can populate the conjunctions
            match &node {
                Node::Broadcaster { to }
                | Node::FlipFlop { to, .. }
                | Node::Conjunction { to, .. } => {
                    for to in to {
                        incoming_connections.entry(to).or_default().push(id);
                    }
                }
                Node::Untyped => {}
            }

            (id, node)
        })
        .collect::<HashMap<_, _>>();

    // populate `from` field of conjunctions
    for (id, inputs) in &incoming_connections {
        if let Node::Conjunction { from, .. } = circuit.entry(id).or_insert(Node::Untyped) {
            for input in inputs {
                from.insert(input, false);
            }
        }
    }

    // the target should have exactly a single Conjuction node (deduced by looking at the input... lame)
    let &[last_node] = incoming_connections["rx"].as_slice() else {
        unreachable!()
    };
    assert!(matches!(&circuit[last_node], Node::Conjunction { .. }));

    let mut loops = vec![];
    let mut t = 0;
    loop {
        t += 1;
        let mut queue = VecDeque::from([("", "broadcaster", false)]);
        while let Some((id_from, id_to, is_high)) = queue.pop_front() {
            match circuit.get_mut(id_to).unwrap() {
                Node::Broadcaster { to } => {
                    for to in to {
                        queue.push_back((id_to, to, is_high));
                    }
                }
                Node::FlipFlop { state, to } => {
                    if !is_high {
                        *state ^= true;
                        for to in to {
                            queue.push_back((id_to, to, *state));
                        }
                    }
                }
                Node::Conjunction { from, to } => {
                    *from.get_mut(id_from).unwrap() = is_high;
                    let all_highs_in = from.values().all(|h| *h);
                    for to in to {
                        // every time the last node is high, record the time it took
                        if to == &last_node && !all_highs_in {
                            // the last node has 4 incoming submodule that loop independently
                            // (again seen from inspecting the input...)
                            // and we are looking for the right combination where all subloops synchronise
                            // so the LCM of the four periods.
                            loops.push(t);
                            if loops.len() == 4 {
                                return loops.into_iter().reduce(lcm).unwrap();
                            }
                        } else {
                            queue.push_back((id_to, to, !all_highs_in));
                        }
                    }
                }
                Node::Untyped => {}
            }
        }
    }
}

#[derive(Debug)]
enum Node<'a> {
    Broadcaster {
        to: Vec<&'a str>,
    },
    FlipFlop {
        state: bool,
        to: Vec<&'a str>,
    },
    Conjunction {
        from: HashMap<&'a str, bool>,
        to: Vec<&'a str>,
    },
    Untyped,
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 11687500);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 243221023462303);
    }
}

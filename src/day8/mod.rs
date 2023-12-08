use std::collections::HashMap;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day8/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day8/test_one.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut lines = input.lines();

    let directions = lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let (node, line) = line.split_once(" = (").unwrap();
        let line = line.strip_suffix(')').unwrap();
        let (left, right) = line.split_once(", ").unwrap();
        map.insert(node, (left, right));
    }

    let mut node = "AAA";
    let mut steps = 0;
    while node != "ZZZ" {
        let direction = steps as usize % directions.len();
        node = match &directions[direction..=direction] {
            "L" => map[node].0,
            "R" => map[node].1,
            _ => unreachable!(),
        };
        steps += 1;
    }

    steps
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day8/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day8/test_two.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut lines = input.lines();

    let directions = lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let (node, line) = line.split_once(" = (").unwrap();
        let line = line.strip_suffix(')').unwrap();
        let (left, right) = line.split_once(", ").unwrap();
        map.insert(node, (left, right));
    }

    let nodes = map
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();

    // WARNING: this assumes "simple loops" i.e.
    // (offset, offset + period, offset + 2 period, ...)
    // which is the case in the input, but not in general?
    let mut z_nodes_offset_periods = nodes
        .into_iter()
        .map(|mut node| {
            let mut steps = vec![];
            for step in 0u64.. {
                if node.ends_with('Z') {
                    steps.push(step);
                    if steps.len() == 2 {
                        break;
                    }
                }
                let direction = step as usize % directions.len();
                node = match &directions[direction..=direction] {
                    "L" => map[node].0,
                    "R" => map[node].1,
                    _ => unreachable!(),
                };
            }

            let offset = steps[0];
            let period = steps[1] - steps[0];

            (offset % period, period)
        })
        .collect::<Vec<_>>();

    // chinese remainder theorem computation by sieve
    // optimised by using larger periods first
    z_nodes_offset_periods.sort_by_key(|&(_, period)| period);
    let (mut steps, mut compound_period) = z_nodes_offset_periods.pop().unwrap();
    steps += compound_period;
    while let Some((offset, period)) = z_nodes_offset_periods.pop() {
        while steps % period != offset {
            steps += compound_period;
        }
        compound_period = lcm(compound_period, period);
    }

    steps
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
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
        assert_eq!(star_one(), 6);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 6);
    }
}

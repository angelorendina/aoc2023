use std::collections::BTreeMap;
use std::collections::BinaryHeap;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day17/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day17/test.txt";

    let map = Map::new(&std::fs::read_to_string(FILENAME).unwrap());

    // target cell, entered horizontally -> weight so far
    let mut explored = BTreeMap::<(isize, isize, bool), u64>::new();

    let mut queue = BinaryHeap::from([
        Q {
            row: 0,
            column: 0,
            total_weight: 0,
            horizontal: true,
        },
        Q {
            row: 0,
            column: 0,
            total_weight: 0,
            horizontal: false,
        },
    ]);

    let mut next_steps = Vec::with_capacity(6);
    while let Some(Q {
        row,
        column,
        total_weight,
        horizontal,
    }) = queue.pop()
    {
        let mut stepped_cost_minus = total_weight;
        let mut stepped_cost_plus = total_weight;
        for i in 1..=3 {
            match horizontal {
                true => {
                    if let Some(cost) = map.get(row - i, column) {
                        stepped_cost_minus += cost;
                        next_steps.push((row - i, column, stepped_cost_minus, false));
                    }
                    if let Some(cost) = map.get(row + i, column) {
                        stepped_cost_plus += cost;
                        next_steps.push((row + i, column, stepped_cost_plus, false));
                    }
                }
                false => {
                    if let Some(cost) = map.get(row, column - i) {
                        stepped_cost_minus += cost;
                        next_steps.push((row, column - i, stepped_cost_minus, true));
                    }
                    if let Some(cost) = map.get(row, column + i) {
                        stepped_cost_plus += cost;
                        next_steps.push((row, column + i, stepped_cost_plus, true));
                    }
                }
            }
        }

        for (next_row, next_column, next_cost, next_horizontal) in next_steps.drain(..) {
            let visited_weight = explored
                .entry((next_row, next_column, next_horizontal))
                .or_insert(u64::MAX);
            if next_cost < *visited_weight {
                *visited_weight = next_cost;
                queue.push(Q {
                    row: next_row,
                    column: next_column,
                    total_weight: next_cost,
                    horizontal: next_horizontal,
                });
            }
        }
    }

    [true, false]
        .into_iter()
        .flat_map(|horizontal| {
            explored
                .get(&(map.rows as isize - 1, map.columns as isize - 1, horizontal))
                .copied()
        })
        .min()
        .unwrap()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day17/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day17/test.txt";

    let map = Map::new(&std::fs::read_to_string(FILENAME).unwrap());

    // target cell, entered horizontally -> weight so far
    let mut explored = BTreeMap::<(isize, isize, bool), u64>::new();

    let mut queue = BinaryHeap::from([
        Q {
            row: 0,
            column: 0,
            total_weight: 0,
            horizontal: true,
        },
        Q {
            row: 0,
            column: 0,
            total_weight: 0,
            horizontal: false,
        },
    ]);

    let mut next_steps = Vec::with_capacity(6);
    while let Some(Q {
        row,
        column,
        total_weight,
        horizontal,
    }) = queue.pop()
    {
        let mut stepped_cost_minus = total_weight;
        let mut stepped_cost_plus = total_weight;
        for i in 1..=3 {
            match horizontal {
                true => {
                    if let Some(cost) = map.get(row - i, column) {
                        stepped_cost_minus += cost;
                    }
                    if let Some(cost) = map.get(row + i, column) {
                        stepped_cost_plus += cost;
                    }
                }
                false => {
                    if let Some(cost) = map.get(row, column - i) {
                        stepped_cost_minus += cost;
                    }
                    if let Some(cost) = map.get(row, column + i) {
                        stepped_cost_plus += cost;
                    }
                }
            }
        }
        for i in 4..=10 {
            match horizontal {
                true => {
                    if let Some(cost) = map.get(row - i, column) {
                        stepped_cost_minus += cost;
                        next_steps.push((row - i, column, stepped_cost_minus, false));
                    }
                    if let Some(cost) = map.get(row + i, column) {
                        stepped_cost_plus += cost;
                        next_steps.push((row + i, column, stepped_cost_plus, false));
                    }
                }
                false => {
                    if let Some(cost) = map.get(row, column - i) {
                        stepped_cost_minus += cost;
                        next_steps.push((row, column - i, stepped_cost_minus, true));
                    }
                    if let Some(cost) = map.get(row, column + i) {
                        stepped_cost_plus += cost;
                        next_steps.push((row, column + i, stepped_cost_plus, true));
                    }
                }
            }
        }

        for (next_row, next_column, next_cost, next_horizontal) in next_steps.drain(..) {
            let visited_weight = explored
                .entry((next_row, next_column, next_horizontal))
                .or_insert(u64::MAX);
            if next_cost < *visited_weight {
                *visited_weight = next_cost;
                queue.push(Q {
                    row: next_row,
                    column: next_column,
                    total_weight: next_cost,
                    horizontal: next_horizontal,
                });
            }
        }
    }

    [true, false]
        .into_iter()
        .flat_map(|horizontal| {
            explored
                .get(&(map.rows as isize - 1, map.columns as isize - 1, horizontal))
                .copied()
        })
        .min()
        .unwrap()
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Map {
    rows: usize,
    columns: usize,
    data: Vec<u64>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Self {
            rows: 0,
            columns: 0,
            data: vec![],
        };

        for (_, line) in input.lines().enumerate() {
            map.rows += 1;
            map.columns = 0;
            for (_, c) in line.chars().enumerate() {
                map.columns += 1;
                map.data.push(c as u64 - 48);
            }
        }

        map
    }

    #[inline]
    fn get(&self, row: isize, column: isize) -> Option<u64> {
        let Ok(row) = usize::try_from(row) else {
            return None;
        };
        let Ok(column) = usize::try_from(column) else {
            return None;
        };
        if row >= self.rows {
            return None;
        }
        if column >= self.columns {
            return None;
        }
        self.data.get(self.columns * row + column).copied()
    }
}

#[derive(PartialEq, Eq)]
struct Q {
    row: isize,
    column: isize,
    total_weight: u64,
    horizontal: bool,
}

impl Ord for Q {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_weight.cmp(&other.total_weight).reverse()
    }
}

impl PartialOrd for Q {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 102);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 94);
    }
}

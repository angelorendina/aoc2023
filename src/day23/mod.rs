use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day23/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day23/test.txt";

    Map::new(&std::fs::read_to_string(FILENAME).unwrap()).get_longest_trek::<true>()
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day23/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day23/test.txt";

    Map::new(&std::fs::read_to_string(FILENAME).unwrap()).get_longest_trek::<false>()
}

enum Cell {
    Ground,
    Wall,
    SlideNorth,
    SlideSouth,
    SlideWest,
    SlideEast,
}

struct Map {
    rows: usize,
    columns: usize,
    data: Vec<Cell>,
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
                map.data.push(match c {
                    '.' => Cell::Ground,
                    '#' => Cell::Wall,
                    '>' => Cell::SlideEast,
                    '<' => Cell::SlideWest,
                    'v' => Cell::SlideSouth,
                    '^' => Cell::SlideNorth,
                    _ => unreachable!(),
                });
            }
        }

        map
    }

    #[inline]
    fn get_mut(&mut self, row: isize, column: isize) -> Option<&mut Cell> {
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
        self.data.get_mut(self.columns * row + column)
    }

    fn get_longest_trek<const SLIPPERY: bool>(mut self) -> usize {
        let mut junctions = BTreeMap::<(isize, isize), BTreeSet<(isize, isize)>>::new();
        for row in 0..self.rows as isize {
            for column in 0..self.columns as isize {
                match self.get_mut(row, column) {
                    None | Some(Cell::Wall) => {
                        continue;
                    }
                    _ => {}
                };
                let mut openings = BTreeSet::new();
                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    match self.get_mut(row + dr, column + dc) {
                        None | Some(Cell::Wall) => {}
                        Some(_) => {
                            openings.insert((row + dr, column + dc));
                        }
                    }
                }
                if openings.len() > 2
                    || (row, column) == (0, 1)
                    || (row, column) == (self.rows as isize - 1, self.columns as isize - 2)
                {
                    junctions.insert((row, column), openings);
                }
            }
        }

        let junction_to_index = |junction: &(isize, isize)| {
            junctions
                .keys()
                .enumerate()
                .find_map(|(i, j)| (j == junction).then_some(i))
                .unwrap()
        };
        let index_to_junction = junctions.keys().copied().collect::<Vec<_>>();

        // from -> { to -> len }
        let mut paths = BTreeMap::<(isize, isize), BTreeMap<(isize, isize), usize>>::new();
        for (&junction, openings) in &junctions {
            for opening in openings {
                let mut cell_at = *opening;
                let mut path_steps = 1;
                let mut last_cell = junction;
                'walk_path: loop {
                    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        if SLIPPERY {
                            // enforce slides
                            match self.get_mut(cell_at.0, cell_at.1).unwrap() {
                                Cell::SlideNorth => {
                                    if (dr, dc) != (-1, 0) {
                                        continue;
                                    }
                                }
                                Cell::SlideSouth => {
                                    if (dr, dc) != (1, 0) {
                                        continue;
                                    }
                                }
                                Cell::SlideWest => {
                                    if (dr, dc) != (0, -1) {
                                        continue;
                                    }
                                }
                                Cell::SlideEast => {
                                    if (dr, dc) != (0, 1) {
                                        continue;
                                    }
                                }
                                _ => {}
                            }
                        }
                        let next_cell = (cell_at.0 + dr, cell_at.1 + dc);
                        match self.get_mut(next_cell.0, next_cell.1) {
                            None | Some(Cell::Wall) => {}
                            Some(_) => {
                                if next_cell != last_cell {
                                    if junctions.contains_key(&next_cell) {
                                        // reached the exit of this path
                                        paths
                                            .entry(junction)
                                            .or_default()
                                            .insert(next_cell, path_steps + 1);
                                        break 'walk_path;
                                    } else {
                                        // keep walking down the path
                                        last_cell = cell_at;
                                        cell_at = next_cell;
                                        path_steps += 1;
                                        continue 'walk_path;
                                    }
                                }
                            }
                        }
                    }
                    // dead end!
                    break 'walk_path;
                }
            }
        }

        // prune suboptimal paths
        let mut explored = BTreeMap::<u128, usize>::new();

        // explore all paths as sequence of junctions
        let mut max_steps = 0;
        // (visited bitmask, junction index at, steps so far)
        let mut queue = VecDeque::<(u128, usize, usize)>::from([(1, 0, 0)]);
        while let Some((walk_bitmask, index_at, steps_so_far)) = queue.pop_front() {
            // prune suboptimal paths
            // this only works because we are doing DFS
            // in a BFS we might be pruning an initially-losing-but-eventually-winning walk
            let ex = explored.entry(walk_bitmask).or_default();
            if *ex > steps_so_far {
                continue;
            }
            *ex = steps_so_far;

            let junction_at = index_to_junction[index_at];
            for (next_junction, &distance) in paths.get(&junction_at).unwrap() {
                if next_junction == &(self.rows as isize - 1, self.columns as isize - 2) {
                    let walk_steps = steps_so_far + distance;
                    max_steps = max_steps.max(walk_steps);
                    continue;
                }
                let next_index = junction_to_index(next_junction);
                let next_bitmask = 1u128 << next_index;
                if walk_bitmask & next_bitmask > 0 {
                    continue;
                }
                queue.push_front((
                    walk_bitmask | next_bitmask,
                    next_index,
                    steps_so_far + distance,
                ));
            }
        }

        max_steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 94);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 154);
    }
}

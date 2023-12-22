use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day22/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day22/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut placed_bricks = vec![];
    let dropping_bricks: BTreeSet<Brick> = input.lines().map(Brick::new).collect();

    // fall into place (z-ordered to avoid premature intersections)
    for mut brick in dropping_bricks {
        loop {
            if brick.start[2] == 1 {
                placed_bricks.push(brick);
                break;
            }

            let dropped = brick.dropped();
            if placed_bricks.iter().any(|b| b.intersects(&dropped)) {
                placed_bricks.push(brick);
                break;
            }
            brick = dropped;
        }
    }

    // for each brick, mark THE ONE it relies on (if that's the case)
    let mut mandatory_bricks: BTreeSet<Brick> = BTreeSet::new();
    for brick in &placed_bricks {
        let mut supported_by = vec![];
        let dropped = brick.dropped();
        for other_brick in placed_bricks.iter().filter(|b| b != &brick) {
            if other_brick.intersects(&dropped) {
                supported_by.push(other_brick);
            }
        }
        if let &[&support] = supported_by.as_slice() {
            mandatory_bricks.insert(support);
        }
    }

    placed_bricks.len() - mandatory_bricks.len()
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day22/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day22/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut placed_bricks = vec![];
    let dropping_bricks: BTreeSet<Brick> = input.lines().map(Brick::new).collect();

    // fall into place (z-ordered to avoid premature intersections)
    for mut brick in dropping_bricks {
        loop {
            if brick.start[2] == 1 {
                placed_bricks.push(brick);
                break;
            }

            let dropped = brick.dropped();
            if placed_bricks.iter().any(|b| b.intersects(&dropped)) {
                placed_bricks.push(brick);
                break;
            }
            brick = dropped;
        }
    }

    // brick -> {bricks below it}
    let mut supported_by = BTreeMap::<Brick, BTreeSet<Brick>>::new();
    // brick -> {bricks above it}
    let mut supports = BTreeMap::<Brick, BTreeSet<Brick>>::new();

    // populate brick dependencies
    for brick in &placed_bricks {
        let supported_by = supported_by.entry(*brick).or_default();
        let dropped = brick.dropped();
        for brick_below in placed_bricks.iter().filter(|b| b != &brick) {
            let supports = supports.entry(*brick_below).or_default();
            if brick_below.intersects(&dropped) {
                supported_by.insert(*brick_below);
                supports.insert(*brick);
            }
        }
    }

    // BFS of chain reactions
    let mut falls = 0;
    for candidate in &placed_bricks {
        let mut queue = BTreeSet::<Brick>::from([*candidate]);
        let mut explored = BTreeSet::<Brick>::new();
        let mut removed = BTreeSet::<Brick>::from([*candidate]);
        while let Some(candidate) = queue.pop_first() {
            for brick_above in &supports[&candidate] {
                if supported_by[brick_above].difference(&removed).count() == 0 {
                    if explored.contains(brick_above) {
                        continue;
                    }
                    explored.insert(*brick_above);
                    falls += 1;
                    removed.insert(*brick_above);
                    queue.insert(*brick_above);
                }
            }
        }
    }

    falls
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Brick {
    start: [isize; 3],
    end: [isize; 3],
}

// z-sorted
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start[2].cmp(&other.start[2]) {
            std::cmp::Ordering::Equal => match self.start[0].cmp(&other.start[0]) {
                std::cmp::Ordering::Equal => self.start[1].cmp(&other.start[1]),
                o => o,
            },
            o => o,
        }
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Brick {
    fn new(line: &str) -> Self {
        let (a, b) = line.split_once('~').unwrap();
        let mut a = a.split(',');
        let mut b = b.split(',');
        let a: [isize; 3] = std::array::from_fn(|_| a.next().unwrap().parse().unwrap());
        let b: [isize; 3] = std::array::from_fn(|_| b.next().unwrap().parse().unwrap());

        let start = [
            isize::min(a[0], b[0]),
            isize::min(a[1], b[1]),
            isize::min(a[2], b[2]),
        ];
        let end = [
            isize::max(a[0], b[0]),
            isize::max(a[1], b[1]),
            isize::max(a[2], b[2]),
        ];

        Self { start, end }
    }

    fn intersects(&self, other: &Self) -> bool {
        let start = [
            isize::max(self.start[0], other.start[0]),
            isize::max(self.start[1], other.start[1]),
            isize::max(self.start[2], other.start[2]),
        ];
        let end = [
            isize::min(self.end[0], other.end[0]),
            isize::min(self.end[1], other.end[1]),
            isize::min(self.end[2], other.end[2]),
        ];

        start[0] <= end[0] && start[1] <= end[1] && start[2] <= end[2]
    }

    fn dropped(self) -> Self {
        let mut dropped = self;
        dropped.start[2] -= 1;
        dropped.end[2] -= 1;
        dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 5);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 7);
    }
}

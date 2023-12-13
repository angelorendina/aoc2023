use std::collections::BTreeMap;

pub fn star_one() -> isize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day13/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day13/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let mut sum = 0;

    let mut map = BTreeMap::<(isize, isize), bool>::new();
    let mut row = 0isize;
    for line in input.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            sum += measure_symmetry::<false>(&map);
            map.clear();
            row = 0;
        } else {
            for (column, x) in line.chars().enumerate() {
                map.insert((row, column as isize), x == '#');
            }
            row += 1;
        }
    }

    sum
}

pub fn star_two() -> isize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day13/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day13/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let mut sum = 0;

    let mut map = BTreeMap::<(isize, isize), bool>::new();
    let mut row = 0isize;
    for line in input.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            sum += measure_symmetry::<true>(&map);
            map.clear();
            row = 0;
        } else {
            for (column, x) in line.chars().enumerate() {
                map.insert((row, column as isize), x == '#');
            }
            row += 1;
        }
    }

    sum
}

fn measure_symmetry<const SMUDGED: bool>(map: &BTreeMap<(isize, isize), bool>) -> isize {
    let (&corner, _) = map.last_key_value().unwrap();

    for symmetry_row in 1..=corner.0 {
        let mut reflection_errors = 0;
        for column in 0..=corner.1 {
            for dr in 1.. {
                let above = map.get(&(symmetry_row - dr, column));
                let Some(above) = above else { break };
                let below = map.get(&(symmetry_row + dr - 1, column));
                let Some(below) = below else { break };
                if above != below {
                    reflection_errors += 1;
                }
            }
        }
        if SMUDGED && reflection_errors == 1 {
            return 100 * symmetry_row;
        }
        if !SMUDGED && reflection_errors == 0 {
            return 100 * symmetry_row;
        }
    }

    for symmetry_column in 1..=corner.1 {
        let mut reflection_errors = 0;
        for row in 0..=corner.0 {
            for dc in 1.. {
                let left = map.get(&(row, symmetry_column - dc));
                let Some(left) = left else { break };
                let right = map.get(&(row, symmetry_column + dc - 1));
                let Some(right) = right else { break };
                if left != right {
                    reflection_errors += 1;
                }
            }
        }
        if SMUDGED && reflection_errors == 1 {
            return symmetry_column;
        }
        if !SMUDGED && reflection_errors == 0 {
            return symmetry_column;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 405);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 400);
    }
}

use std::collections::BTreeSet;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day11/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day11/test.txt";

    let map = std::fs::read_to_string(FILENAME).unwrap();

    let mut dimensions = (
        map.lines().count(),
        map.lines().last().unwrap().chars().count(),
    );

    let mut map =
        map.lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(column, c)| {
                    if c == '#' {
                        Some((row, column))
                    } else {
                        None
                    }
                })
            })
            .collect::<BTreeSet<_>>();

    let empty_rows = (0..dimensions.0)
        .filter(|&row| !map.iter().any(|&point| point.0 == row))
        .collect::<Vec<_>>();
    let empty_columns = (0..dimensions.1)
        .filter(|&column| !map.iter().any(|&point| point.1 == column))
        .collect::<Vec<_>>();

    for row in empty_rows.into_iter().rev() {
        map = map
            .into_iter()
            .map(|point| (if point.0 > row { point.0 + 1 } else { point.0 }, point.1))
            .collect();
        dimensions.0 += 1;
    }
    for column in empty_columns.into_iter().rev() {
        map = map
            .into_iter()
            .map(|point| {
                (
                    point.0,
                    if point.1 > column {
                        point.1 + 1
                    } else {
                        point.1
                    },
                )
            })
            .collect();
        dimensions.1 += 1;
    }

    let mut sum = 0;
    for &from in map.iter() {
        for &to in map.iter() {
            if to > from {
                let dr = usize::abs_diff(from.0, to.0);
                let dc = usize::abs_diff(from.1, to.1);
                sum += (dr + dc) as u64;
            }
        }
    }

    sum
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day11/input.txt";
    #[cfg(not(test))]
    const EXPANSION_FACTOR: usize = 1000000;
    #[cfg(test)]
    const FILENAME: &str = "src/day11/test.txt";
    #[cfg(test)]
    const EXPANSION_FACTOR: usize = 10;

    let map = std::fs::read_to_string(FILENAME).unwrap();

    let mut dimensions = (
        map.lines().count(),
        map.lines().last().unwrap().chars().count(),
    );

    let mut map =
        map.lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(column, c)| {
                    if c == '#' {
                        Some((row, column))
                    } else {
                        None
                    }
                })
            })
            .collect::<BTreeSet<_>>();

    let empty_rows = (0..dimensions.0)
        .filter(|&row| !map.iter().any(|&point| point.0 == row))
        .collect::<Vec<_>>();
    let empty_columns = (0..dimensions.1)
        .filter(|&column| !map.iter().any(|&point| point.1 == column))
        .collect::<Vec<_>>();

    for row in empty_rows.into_iter().rev() {
        map = map
            .into_iter()
            .map(|point| {
                (
                    if point.0 > row {
                        point.0 + EXPANSION_FACTOR - 1
                    } else {
                        point.0
                    },
                    point.1,
                )
            })
            .collect();
        dimensions.0 += 1;
    }
    for column in empty_columns.into_iter().rev() {
        map = map
            .into_iter()
            .map(|point| {
                (
                    point.0,
                    if point.1 > column {
                        point.1 + EXPANSION_FACTOR - 1
                    } else {
                        point.1
                    },
                )
            })
            .collect();
        dimensions.1 += 1;
    }

    let mut sum = 0;
    for &from in map.iter() {
        for &to in map.iter() {
            if to > from {
                let dr = usize::abs_diff(from.0, to.0);
                let dc = usize::abs_diff(from.1, to.1);
                sum += (dr + dc) as u64;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 374);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 1030);
    }
}

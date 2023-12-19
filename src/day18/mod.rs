use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn star_one() -> isize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day18/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day18/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let mut row_segmentation = BTreeSet::<isize>::new();
    let mut column_segmentation = BTreeSet::<isize>::new();

    let mut max_coords = (0, 0);
    let mut min_coords = (0, 0);

    let mut digger = (0, 0);
    for line in input.lines() {
        let (direction, line) = line.split_once(' ').unwrap();
        let (steps, _colour) = line.split_once(' ').unwrap();
        let steps = steps.parse::<isize>().unwrap();

        row_segmentation.insert(digger.0);
        row_segmentation.insert(digger.0 + 1);
        column_segmentation.insert(digger.1);
        column_segmentation.insert(digger.1 + 1);
        match direction {
            "U" => {
                digger.0 -= steps;
            }
            "D" => {
                digger.0 += steps;
            }
            "R" => {
                digger.1 += steps;
            }
            "L" => {
                digger.1 -= steps;
            }
            _ => unreachable!(),
        }

        max_coords.0 = max_coords.0.max(digger.0);
        max_coords.1 = max_coords.1.max(digger.1);
        min_coords.0 = min_coords.0.min(digger.0);
        min_coords.1 = min_coords.1.min(digger.1);
    }
    row_segmentation.insert(min_coords.0 - 1);
    column_segmentation.insert(min_coords.1 - 1);

    let mut areas = BTreeMap::<(isize, isize), char>::new();

    digger = (0, 0);
    for line in input.lines() {
        let (direction, line) = line.split_once(' ').unwrap();
        let (steps, _colour) = line.split_once(' ').unwrap();
        let steps = steps.parse::<isize>().unwrap();

        let start_digger = digger;

        match direction {
            "U" => {
                digger.0 -= steps;
            }
            "D" => {
                digger.0 += steps;
            }
            "R" => {
                digger.1 += steps;
            }
            "L" => {
                digger.1 -= steps;
            }
            _ => unreachable!(),
        }

        let top_left = (start_digger.0.min(digger.0), start_digger.1.min(digger.1));
        let bottom_right = (
            start_digger.0.max(digger.0) + 1,
            start_digger.1.max(digger.1) + 1,
        );

        for &row in row_segmentation.range(top_left.0..bottom_right.0) {
            for &column in column_segmentation.range(top_left.1..bottom_right.1) {
                areas.insert((row, column), '#');
            }
        }
    }

    let mut flood_fill = vec![(min_coords.0 - 1, min_coords.1 - 1)];
    while let Some((row, column)) = flood_fill.pop() {
        if areas.get(&(row, column)).is_none() {
            areas.insert((row, column), '.');
            let next_row = row_segmentation.range(row..).nth(1);
            let next_column = column_segmentation.range(column..).nth(1);
            let previous_row = row_segmentation.range(..row).last();
            let previous_column = column_segmentation.range(..column).last();
            [
                (previous_row, previous_column),
                (previous_row, Some(&column)),
                (previous_row, next_column),
                (Some(&row), previous_column),
                (Some(&row), Some(&column)),
                (Some(&row), next_column),
                (next_row, previous_column),
                (next_row, Some(&column)),
                (next_row, next_column),
            ]
            .into_iter()
            .filter_map(|(r, c)| {
                if let Some(&r) = r {
                    if let Some(&c) = c {
                        Some((r, c))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .for_each(|p| {
                flood_fill.push(p);
            });
        }
    }

    let mut sum = 0;
    for i in 0..row_segmentation.len() - 1 {
        let &row = row_segmentation.iter().nth(i).unwrap();
        let &end_row = row_segmentation.iter().nth(i + 1).unwrap();
        for j in 0..column_segmentation.len() - 1 {
            let &column = column_segmentation.iter().nth(j).unwrap();
            let &end_column = column_segmentation.iter().nth(j + 1).unwrap();

            match areas.get(&(row, column)) {
                Some('#') | None => {
                    sum += (end_row - row) * (end_column - column);
                }
                _ => {}
            }
        }
    }

    sum
}

pub fn star_two() -> isize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day18/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day18/test.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();

    let mut row_segmentation = BTreeSet::<isize>::new();
    let mut column_segmentation = BTreeSet::<isize>::new();

    let mut max_coords = (0, 0);
    let mut min_coords = (0, 0);

    let mut digger = (0, 0);
    for line in input.lines() {
        let (_direction, line) = line.split_once(' ').unwrap();
        let (_steps, colour) = line.split_once(' ').unwrap();
        let colour = colour.strip_prefix("(#").unwrap();
        let colour = colour.strip_suffix(')').unwrap();
        let direction = &colour[colour.len() - 1..];
        let steps = &colour[..colour.len() - 1];
        let steps = isize::from_str_radix(steps, 16).unwrap();

        row_segmentation.insert(digger.0);
        row_segmentation.insert(digger.0 + 1);
        column_segmentation.insert(digger.1);
        column_segmentation.insert(digger.1 + 1);
        match direction {
            "3" => {
                digger.0 -= steps;
            }
            "1" => {
                digger.0 += steps;
            }
            "0" => {
                digger.1 += steps;
            }
            "2" => {
                digger.1 -= steps;
            }
            _ => unreachable!(),
        }

        max_coords.0 = max_coords.0.max(digger.0);
        max_coords.1 = max_coords.1.max(digger.1);
        min_coords.0 = min_coords.0.min(digger.0);
        min_coords.1 = min_coords.1.min(digger.1);
    }
    row_segmentation.insert(min_coords.0 - 1);
    column_segmentation.insert(min_coords.1 - 1);

    let mut areas = BTreeMap::<(isize, isize), char>::new();

    digger = (0, 0);
    for line in input.lines() {
        let (_direction, line) = line.split_once(' ').unwrap();
        let (_steps, colour) = line.split_once(' ').unwrap();
        let colour = colour.strip_prefix("(#").unwrap();
        let colour = colour.strip_suffix(')').unwrap();
        let direction = &colour[colour.len() - 1..];
        let steps = &colour[..colour.len() - 1];
        let steps = isize::from_str_radix(steps, 16).unwrap();

        let start_digger = digger;

        match direction {
            "3" => {
                digger.0 -= steps;
            }
            "1" => {
                digger.0 += steps;
            }
            "0" => {
                digger.1 += steps;
            }
            "2" => {
                digger.1 -= steps;
            }
            _ => unreachable!(),
        }

        let top_left = (start_digger.0.min(digger.0), start_digger.1.min(digger.1));
        let bottom_right = (
            start_digger.0.max(digger.0) + 1,
            start_digger.1.max(digger.1) + 1,
        );

        for &row in row_segmentation.range(top_left.0..bottom_right.0) {
            for &column in column_segmentation.range(top_left.1..bottom_right.1) {
                areas.insert((row, column), '#');
            }
        }
    }

    let mut flood_fill = vec![(min_coords.0 - 1, min_coords.1 - 1)];
    while let Some((row, column)) = flood_fill.pop() {
        if areas.get(&(row, column)).is_none() {
            areas.insert((row, column), '.');
            let next_row = row_segmentation.range(row..).nth(1);
            let next_column = column_segmentation.range(column..).nth(1);
            let previous_row = row_segmentation.range(..row).last();
            let previous_column = column_segmentation.range(..column).last();
            [
                (previous_row, previous_column),
                (previous_row, Some(&column)),
                (previous_row, next_column),
                (Some(&row), previous_column),
                (Some(&row), Some(&column)),
                (Some(&row), next_column),
                (next_row, previous_column),
                (next_row, Some(&column)),
                (next_row, next_column),
            ]
            .into_iter()
            .filter_map(|(r, c)| {
                if let Some(&r) = r {
                    if let Some(&c) = c {
                        Some((r, c))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .for_each(|p| {
                flood_fill.push(p);
            });
        }
    }

    let mut sum = 0;
    for i in 0..row_segmentation.len() - 1 {
        let &row = row_segmentation.iter().nth(i).unwrap();
        let &end_row = row_segmentation.iter().nth(i + 1).unwrap();
        for j in 0..column_segmentation.len() - 1 {
            let &column = column_segmentation.iter().nth(j).unwrap();
            let &end_column = column_segmentation.iter().nth(j + 1).unwrap();

            match areas.get(&(row, column)) {
                Some('#') | None => {
                    sum += (end_row - row) * (end_column - column);
                }
                _ => {}
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
        assert_eq!(star_one(), 62);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 952408144115);
    }
}

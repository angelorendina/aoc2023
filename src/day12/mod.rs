use std::collections::HashMap;
use std::ops::Range;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day12/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day12/test.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(|line| {
            let (cells, lengths) = line.split_once(' ').unwrap();
            let cells = cells.chars().collect::<Vec<_>>();
            let lengths = lengths
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            count_arrangements(&cells, &lengths)
        })
        .sum()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day12/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day12/test.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(|line| {
            let (cells, lengths) = line.split_once(' ').unwrap();
            let original_cells = cells.chars().collect::<Vec<_>>();
            let mut cells = vec![];
            cells.extend_from_slice(&original_cells);
            cells.push('?');
            cells.extend_from_slice(&original_cells);
            cells.push('?');
            cells.extend_from_slice(&original_cells);
            cells.push('?');
            cells.extend_from_slice(&original_cells);
            cells.push('?');
            cells.extend(original_cells);

            let original_lengths = lengths
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut lengths = vec![];
            lengths.extend_from_slice(&original_lengths);
            lengths.extend_from_slice(&original_lengths);
            lengths.extend_from_slice(&original_lengths);
            lengths.extend_from_slice(&original_lengths);
            lengths.extend(original_lengths);

            count_arrangements(&cells, &lengths)
        })
        .sum()
}

fn count_arrangements(cells: &[char], lengths: &[usize]) -> u64 {
    let mut memo = HashMap::<(Range<*const char>, Range<*const usize>), u64>::new();
    do_count_arrangements(cells, lengths, &mut memo)
}

fn get_memoised(
    cells: &[char],
    lengths: &[usize],
    memo: &HashMap<(Range<*const char>, Range<*const usize>), u64>,
) -> Option<u64> {
    let cells = cells.as_ptr_range();
    let lengths = lengths.as_ptr_range();
    memo.get(&(cells, lengths)).copied()
}

fn set_memoised(
    cells: &[char],
    lengths: &[usize],
    memo: &mut HashMap<(Range<*const char>, Range<*const usize>), u64>,
    value: u64,
) {
    let cells = cells.as_ptr_range();
    let lengths = lengths.as_ptr_range();
    memo.insert((cells, lengths), value);
}

fn do_count_arrangements(
    cells: &[char],
    lengths: &[usize],
    memo: &mut HashMap<(Range<*const char>, Range<*const usize>), u64>,
) -> u64 {
    if let Some(v) = get_memoised(cells, lengths, memo) {
        return v;
    }

    // No more lengths, valid only if there are no #
    if lengths.is_empty() {
        return if cells.iter().any(|&c| c == '#') {
            0
        } else {
            1
        };
    }

    // Lengths remaining but no space!
    if cells.is_empty() {
        return 0;
    }

    let v = match cells.first().unwrap() {
        '.' => do_count_arrangements(&cells[1..], lengths, memo),
        '#' => do_count_arrangements_tagged(cells, lengths, memo),
        '?' => {
            do_count_arrangements(&cells[1..], lengths, memo)
                + do_count_arrangements_tagged(cells, lengths, memo)
        }
        _ => unreachable!(),
    };
    set_memoised(cells, lengths, memo, v);
    v
}

fn do_count_arrangements_tagged(
    cells: &[char],
    lengths: &[usize],
    memo: &mut HashMap<(Range<*const char>, Range<*const usize>), u64>,
) -> u64 {
    if let Some(v) = get_memoised(cells, lengths, memo) {
        return v;
    }

    if cells.len() < lengths[0] {
        return 0;
    }

    for c in cells.iter().take(lengths[0]) {
        if *c == '.' {
            return 0;
        }
    }

    let v = if cells.len() > lengths[0] {
        if cells[lengths[0]] == '#' {
            return 0;
        }
        do_count_arrangements(&cells[lengths[0] + 1..], &lengths[1..], memo)
    } else {
        do_count_arrangements(&[], &lengths[1..], memo)
    };
    set_memoised(cells, lengths, memo, v);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 21);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 525152);
    }
}

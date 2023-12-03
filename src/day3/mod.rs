pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day3/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day3/test.txt";

    let map = std::fs::read_to_string(FILENAME).unwrap();

    // coordinates of all symbols
    let mut symbols_positions = vec![];
    for (row, line) in map.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c.is_numeric() {
                continue;
            }
            symbols_positions.push((row, column));
        }
    }

    // compute result
    let mut sum = 0;
    for (row, line) in map.lines().enumerate() {
        let spans = NumberSpan::decode_from_row(row, line);
        for span in spans {
            // if span is next to a symbol, include it in the sum
            if symbols_positions
                .iter()
                .any(|&(r, c)| span.is_adjacent_to(r, c))
            {
                sum += span.value;
            }
        }
    }

    sum
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day3/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day3/test.txt";

    let map = std::fs::read_to_string(FILENAME).unwrap();

    // find all spans
    let mut spans = vec![];
    for (row, line) in map.lines().enumerate() {
        spans.extend(NumberSpan::decode_from_row(row, line));
    }

    // compute result
    let mut sum = 0;
    for (row, line) in map.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            // for each gear
            if c == '*' {
                // determine all adjacent values
                let adjacent_values = spans
                    .iter()
                    .filter(|span| span.is_adjacent_to(row, column))
                    .map(|span| span.value)
                    .collect::<Vec<_>>();
                // but only consider them if exactly two
                if let [x, y] = adjacent_values[..] {
                    sum += x * y;
                }
            }
        }
    }

    sum
}

struct NumberSpan {
    row: usize,
    start: usize, // inclusive
    end: usize,   // exclusive
    value: u64,
}

impl NumberSpan {
    fn decode_from_row(row: usize, line: &str) -> Vec<Self> {
        let mut positions = vec![];
        // include a spurious '.' at the end to correctly detect digits at EOL
        for (i, c) in line.chars().chain(std::iter::once('.')).enumerate() {
            // is reading digits if there is a start position but no end, hence odd many
            let is_reading_digits = positions.len() % 2 > 0;
            // detect changes (digit <-> nondigit) when reading
            if c.is_numeric() != is_reading_digits {
                positions.push(i);
            }
        }

        positions
            .chunks_exact(2)
            .map(|chunk| {
                let &[start, end] = chunk else {
                    unreachable!("chunks have exact size 2")
                };
                let value = line[start..end].parse().unwrap();
                Self {
                    row,
                    start,
                    end,
                    value,
                }
            })
            .collect()
    }

    fn is_adjacent_to(&self, row: usize, column: usize) -> bool {
        // point is far in rows
        if usize::abs_diff(self.row, row) > 1 {
            return false;
        }

        // point is too far left
        if column + 1 < self.start {
            return false;
        }

        // point is too far right
        if column > self.end {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 4361);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 467835);
    }
}

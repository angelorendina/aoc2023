use std::collections::BTreeMap;
use std::collections::HashSet;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day4/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day4/test.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(ScratchCard::parse_line)
        .map(|s| (1 << s.matches()) >> 1)
        .sum()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day4/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day4/test.txt";

    // counts how many copies of the same card there are by id (original + extra from previous wins)
    let mut copies = BTreeMap::<usize, u64>::new();

    for line in std::fs::read_to_string(FILENAME).unwrap().lines() {
        let card = ScratchCard::parse_line(line);

        // account for the original card, in addition to the previous wins
        let copy = copies.entry(card.id).or_default();
        *copy += 1;
        let copy = *copy;

        // when winning, grant extra copies to the next cards
        for i in 1..=card.matches() {
            *copies.entry(card.id + i).or_default() += copy;
        }
    }

    copies.values().sum()
}

struct ScratchCard<'a> {
    id: usize,
    winning_numbers: HashSet<&'a str>,
    actual_numbers: HashSet<&'a str>,
}

impl<'a> ScratchCard<'a> {
    fn parse_line(line: &'a str) -> Self {
        let (id, line) = line.split_once(':').unwrap();
        let id = id.split_whitespace().last().unwrap().parse().unwrap();
        let (winning_numbers, actual_numbers) = line.split_once('|').unwrap();
        let winning_numbers = winning_numbers.split_whitespace().collect();
        let actual_numbers = actual_numbers.split_whitespace().collect();

        Self {
            id,
            winning_numbers,
            actual_numbers,
        }
    }

    /// How many matches between the winning and actual numbers.
    fn matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.actual_numbers)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 13);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 30);
    }
}

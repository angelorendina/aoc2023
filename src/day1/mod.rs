pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day1/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day1/test_one.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(decode_line::<false>)
        .sum()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day1/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day1/test_two.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(decode_line::<true>)
        .sum()
}

/// First digit in line is tens, and last digit in line is units
fn decode_line<const WITH_WORDS: bool>(line: &str) -> u64 {
    let d = first_digit::<WITH_WORDS>(line, false).unwrap();
    let u = first_digit::<WITH_WORDS>(line, true).unwrap();
    d * 10 + u
}

/// Find first digit from start or end
fn first_digit<const WITH_WORDS: bool>(line: &str, rtl: bool) -> Option<u64> {
    const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const WORDS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // for all tails
    for i in 0..=line.len() {
        // iterating from the start or the end
        let i = if rtl { line.len() - i } else { i };
        let (_, line) = line.split_at(i);

        // return any digit found
        for (w, j) in DIGITS.into_iter().zip(0..) {
            if line.starts_with(w) {
                return Some(j);
            }
        }

        // or the digit if spelled out
        if WITH_WORDS {
            for (w, j) in WORDS.into_iter().zip(0..) {
                if line.starts_with(w) {
                    return Some(j);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 142);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 281);
    }
}

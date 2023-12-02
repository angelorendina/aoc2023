pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day2/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day2/test.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(Game::parse)
        .filter(Game::is_possible)
        .map(|game| game.id)
        .sum()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day2/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day2/test.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(Game::parse)
        .map(|game| game.minimal_content())
        .map(|cubes| cubes.0 * cubes.1 * cubes.2)
        .sum()
}

struct Game {
    id: u64,
    hands: Vec<(u64, u64, u64)>,
}

impl Game {
    const CONTENT: (u64, u64, u64) = (12, 13, 14);

    fn minimal_content(&self) -> (u64, u64, u64) {
        self.hands.iter().fold((0, 0, 0), |content, hand| {
            (
                u64::max(content.0, hand.0),
                u64::max(content.1, hand.1),
                u64::max(content.2, hand.2),
            )
        })
    }

    fn is_possible(&self) -> bool {
        self.hands.iter().all(|hand| {
            hand.0 <= Self::CONTENT.0 && hand.1 <= Self::CONTENT.1 && hand.2 <= Self::CONTENT.2
        })
    }

    fn parse(line: &str) -> Self {
        let line = line.strip_prefix("Game ").unwrap();
        let (id, line) = line.split_once(':').unwrap();
        let hands = line.split(';').map(|hand| {
            let mut cubes = (0u64, 0u64, 0u64);
            let hand = hand.split(',');
            for cube in hand {
                let (cube, colour) = cube.trim().split_once(' ').unwrap();
                let cube = cube.parse().unwrap();
                match colour {
                    "red" => {
                        cubes.0 = cube;
                    }
                    "green" => {
                        cubes.1 = cube;
                    }
                    "blue" => {
                        cubes.2 = cube;
                    }
                    _ => unreachable!(),
                }
            }

            cubes
        });

        Self {
            id: id.parse().unwrap(),
            hands: hands.collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 8);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 2286);
    }
}

use std::collections::BTreeMap;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day7/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day7/test.txt";

    rank_and_score::<false>(std::fs::read_to_string(FILENAME).unwrap())
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day7/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day7/test.txt";

    rank_and_score::<true>(std::fs::read_to_string(FILENAME).unwrap())
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    One,
    Two,
    Pairs,
    Three,
    Full,
    Four,
    Five,
}

struct Hand {
    cards: [u8; 5],
    bid: u64,
}

impl Hand {
    fn parse_line<const JOKERS: bool>(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let mut cards = cards.chars();
        Self {
            cards: std::array::from_fn(|_| match cards.next().unwrap() {
                'T' => 10,
                'J' => {
                    if JOKERS {
                        1
                    } else {
                        11
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                n => n as u8 - 48,
            }),
            bid: bid.parse().unwrap(),
        }
    }

    fn kind(&self) -> Kind {
        let mut copies = BTreeMap::<u8, u8>::new();
        for card in self.cards {
            *copies.entry(card).or_default() += 1;
        }

        let mut jokers = 0u8;
        let mut bunches_multiplicity = BTreeMap::<u8, u8>::new();
        for (card, bunch) in copies {
            if card == 1 {
                jokers = bunch;
            } else {
                *bunches_multiplicity.entry(bunch).or_default() += 1;
            }
        }

        if bunches_multiplicity.contains_key(&5) {
            return Kind::Five;
        }
        if bunches_multiplicity.contains_key(&4) {
            return match jokers {
                0 => Kind::Four,
                1 => Kind::Five,
                _ => unreachable!(),
            };
        }
        if bunches_multiplicity.contains_key(&3) && bunches_multiplicity.contains_key(&2) {
            return Kind::Full;
        }
        if bunches_multiplicity.contains_key(&3) {
            return match jokers {
                0 => Kind::Three,
                1 => Kind::Four,
                2 => Kind::Five,
                _ => unreachable!(),
            };
        }
        if bunches_multiplicity.get(&2).unwrap_or(&0) == &2 {
            return match jokers {
                0 => Kind::Pairs,
                1 => Kind::Full,
                _ => unreachable!(),
            };
        }
        if bunches_multiplicity.get(&2).unwrap_or(&0) == &1 {
            return match jokers {
                0 => Kind::Two,
                1 => Kind::Three,
                2 => Kind::Four,
                3 => Kind::Five,
                _ => unreachable!(),
            };
        }

        match jokers {
            0 => Kind::One,
            1 => Kind::Two,
            2 => Kind::Three,
            3 => Kind::Four,
            4 => Kind::Five,
            5 => Kind::Five,
            _ => unreachable!(),
        }
    }
}

fn rank_and_score<const JOKERS: bool>(input: String) -> u64 {
    let mut hands_by_kind = BTreeMap::<Kind, BTreeMap<[u8; 5], u64>>::new();
    for hand in input.lines().map(Hand::parse_line::<JOKERS>) {
        let rank = hand.kind();
        let hands_by_cards = hands_by_kind.entry(rank).or_default();
        hands_by_cards.insert(hand.cards, hand.bid);
    }

    hands_by_kind
        .into_iter()
        .flat_map(|(_, hands_by_card)| hands_by_card)
        .map(|(_, bid)| bid)
        .zip(1u64..)
        .map(|(rank, bid)| rank * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 6440);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 5905);
    }
}

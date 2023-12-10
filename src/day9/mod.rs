pub fn star_one() -> i64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day9/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day9/test.txt";

    let mut sum = 0;
    for line in std::fs::read_to_string(FILENAME).unwrap().lines() {
        let mut values = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .map(Binomial::from)
            .collect::<Vec<_>>();
        values.push(Binomial { constant: 0, x: 1 });

        loop {
            let mut all_zeroes = true;
            for i in 0..values.len() - 1 {
                let diff = &values[i + 1] - &values[i];
                if diff.x == 0 {
                    all_zeroes &= diff.constant == 0;
                }
                values[i] = diff;
            }
            values.pop();
            if all_zeroes {
                break;
            }
        }

        sum += values.pop().unwrap().get_zero();
    }

    sum
}

pub fn star_two() -> i64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day9/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day9/test.txt";

    let mut sum = 0;
    for line in std::fs::read_to_string(FILENAME).unwrap().lines() {
        let mut values = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .map(Binomial::from)
            .collect::<Vec<_>>();
        values.insert(0, Binomial { constant: 0, x: 1 });

        loop {
            let mut all_zeroes = true;
            for i in 0..values.len() - 1 {
                let diff = &values[i + 1] - &values[i];
                if diff.x == 0 {
                    all_zeroes &= diff.constant == 0;
                }
                values[i] = diff;
            }
            values.pop();
            if all_zeroes {
                break;
            }
        }

        sum += values.swap_remove(0).get_zero();
    }

    sum
}

struct Binomial {
    constant: i64,
    x: i64,
}

impl std::ops::Sub<&Binomial> for &Binomial {
    type Output = Binomial;

    fn sub(self, rhs: &Binomial) -> Self::Output {
        Binomial {
            constant: self.constant - rhs.constant,
            x: self.x - rhs.x,
        }
    }
}

impl From<i64> for Binomial {
    fn from(value: i64) -> Self {
        Self {
            constant: value,
            x: 0,
        }
    }
}

impl Binomial {
    fn get_zero(self) -> i64 {
        -self.constant / self.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 114);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 2);
    }
}

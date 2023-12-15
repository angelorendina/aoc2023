pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day15/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day15/test.txt";

    std::fs::read_to_string(FILENAME)
        .unwrap()
        .split(',')
        .map(hash)
        .sum()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day15/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day15/test.txt";

    let mut map = std::array::from_fn::<Vec<(u8, &str)>, 256, _>(|_| vec![]);

    let input = std::fs::read_to_string(FILENAME).unwrap();

    for line in input.split(',') {
        if let Some((label, value)) = line.split_once('=') {
            let value = value.parse().unwrap();
            let boxed = map.get_mut(hash(label) as usize).unwrap();
            if let Some((v, _)) = boxed.iter_mut().find(|(_, l)| l == &label) {
                *v = value;
            } else {
                boxed.push((value, label));
            }
        } else {
            let label = line.strip_suffix('-').unwrap();
            let boxed = map.get_mut(hash(label) as usize).unwrap();
            boxed.retain(|(_, l)| l != &label);
        }
    }

    let mut sum = 0;
    for (l, boxed) in map.into_iter().enumerate() {
        for (pos, (lens, _)) in boxed.into_iter().enumerate() {
            sum += (l as u64 + 1) * (pos as u64 + 1) * (lens as u64);
        }
    }

    sum
}

fn hash(line: &str) -> u64 {
    let mut hash = 0;
    for c in line.chars() {
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 1320);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 145);
    }
}

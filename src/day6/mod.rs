pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day6/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day6/test.txt";

    let lines = std::fs::read_to_string(FILENAME).unwrap();
    let mut lines = lines.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            // 0 <= charge <= time and 0 <= run <= time
            // time = charge + run and  distance = run * charge
            // hence
            // distance = charge * (time - charge)
            // want charge * (time - charge) >= max_distance i.e.
            // c^2 - tc + d <= 0
            let delta = (time * time - 4.0 * distance).sqrt();
            let x0 = f64::floor((time - delta) * 0.5) as u64;
            let x1 = f64::ceil((time + delta) * 0.5) as u64;
            x1 - x0 - 1
        })
        .product()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day6/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day6/test.txt";

    let lines = std::fs::read_to_string(FILENAME).unwrap();
    let mut lines = lines.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |mut s, t| {
            s.push_str(t);
            s
        })
        .parse::<f64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |mut s, t| {
            s.push_str(t);
            s
        })
        .parse::<f64>()
        .unwrap();

    // as above but bigger numbers
    let delta = (time * time - 4.0 * distance).sqrt();
    let x0 = f64::floor((time - delta) * 0.5) as u64;
    let x1 = f64::ceil((time + delta) * 0.5) as u64;
    x1 - x0 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 288);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 71503);
    }
}

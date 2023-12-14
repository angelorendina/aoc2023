use std::collections::HashMap;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day14/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day14/test.txt";

    let mut map = Map::new(&std::fs::read_to_string(FILENAME).unwrap());

    map.roll_north();

    map.load()
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day14/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day14/test.txt";

    let mut map = Map::new(&std::fs::read_to_string(FILENAME).unwrap());

    let mut visited = HashMap::<Map, usize>::new();
    for i in 0.. {
        map.roll_north();
        map.roll_west();
        map.roll_south();
        map.roll_east();

        if let Some(&n) = visited.get(&map) {
            let loop_length = i - n;
            let looping_at = n + (1_000_000_000 - 1 - n) % loop_length;
            for (visited_map, &visited_time) in &visited {
                if visited_time == looping_at {
                    return visited_map.load();
                }
            }
        } else {
            visited.insert(map.clone(), i);
        }
    }

    unreachable!()
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Map {
    rows: usize,
    columns: usize,
    data: Vec<char>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Self {
            rows: 0,
            columns: 0,
            data: vec![],
        };

        for (_, line) in input.lines().enumerate() {
            map.rows += 1;
            map.columns = 0;
            for (_, c) in line.chars().enumerate() {
                map.columns += 1;
                map.data.push(c);
            }
        }

        map
    }

    #[inline]
    fn get(&self, row: usize, column: usize) -> &char {
        self.data.get(self.columns * row + column).unwrap()
    }

    #[inline]
    fn get_mut(&mut self, row: usize, column: usize) -> &mut char {
        self.data.get_mut(self.rows * row + column).unwrap()
    }

    fn load(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.rows {
            for column in 0..self.columns {
                if self.get(row, column) == &'O' {
                    sum += self.rows - row;
                }
            }
        }

        sum
    }

    fn roll_north(&mut self) {
        for column in 0..self.columns {
            for row in 0..self.rows {
                match self.get_mut(row, column) {
                    c if c == &'O' => *c = '.',
                    _ => {
                        continue;
                    }
                };
                for dr in 1.. {
                    if row < dr || self.get(row - dr, column) != &'.' {
                        *self.get_mut(row + 1 - dr, column) = 'O';
                        break;
                    }
                }
            }
        }
    }

    fn roll_west(&mut self) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                match self.get_mut(row, column) {
                    c if c == &'O' => *c = '.',
                    _ => {
                        continue;
                    }
                };
                for dc in 1.. {
                    if column < dc || self.get(row, column - dc) != &'.' {
                        *self.get_mut(row, column + 1 - dc) = 'O';
                        break;
                    }
                }
            }
        }
    }

    fn roll_south(&mut self) {
        for column in 0..self.columns {
            for row in (0..self.rows).rev() {
                match self.get_mut(row, column) {
                    c if c == &'O' => *c = '.',
                    _ => {
                        continue;
                    }
                };
                for dr in 1.. {
                    if row + dr >= self.rows || self.get(row + dr, column) != &'.' {
                        *self.get_mut(row + dr - 1, column) = 'O';
                        break;
                    }
                }
            }
        }
    }

    fn roll_east(&mut self) {
        for row in 0..self.rows {
            for column in (0..self.columns).rev() {
                match self.get_mut(row, column) {
                    c if c == &'O' => *c = '.',
                    _ => {
                        continue;
                    }
                };
                for dc in 1.. {
                    if column + dc >= self.columns || self.get(row, column + dc) != &'.' {
                        *self.get_mut(row, column + dc - 1) = 'O';
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 136);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 64);
    }
}

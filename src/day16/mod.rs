use std::collections::BTreeMap;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day16/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day16/test.txt";

    let map = Map::new(&std::fs::read_to_string(FILENAME).unwrap());

    map.evaluate(Ray {
        start_row: 0,
        start_column: -1,
        direction: RayDirection::East,
    })
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day16/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day16/test.txt";

    let map = Map::new(&std::fs::read_to_string(FILENAME).unwrap());

    let mut max = 0;
    for row in 0..map.rows {
        let energy = map.evaluate(Ray {
            start_row: row as isize,
            start_column: -1,
            direction: RayDirection::East,
        });
        max = max.max(energy);
        let energy = map.evaluate(Ray {
            start_row: row as isize,
            start_column: map.columns as isize,
            direction: RayDirection::West,
        });
        max = max.max(energy);
    }
    for column in 0..map.columns {
        let energy = map.evaluate(Ray {
            start_row: -1,
            start_column: column as isize,
            direction: RayDirection::South,
        });
        max = max.max(energy);
        let energy = map.evaluate(Ray {
            start_row: map.rows as isize,
            start_column: column as isize,
            direction: RayDirection::North,
        });
        max = max.max(energy);
    }

    max
}

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
    fn get(&self, row: isize, column: isize) -> Option<&char> {
        if row < 0 || column < 0 || row as usize >= self.rows || column as usize >= self.columns {
            return None;
        }
        let t = self.columns as isize * row + column;
        match usize::try_from(t) {
            Ok(t) => self.data.get(t),
            Err(_) => None,
        }
    }

    fn emit(&self, ray: Ray) -> Vec<Ray> {
        let (row, column) = match ray.direction {
            RayDirection::North => (ray.start_row - 1, ray.start_column),
            RayDirection::South => (ray.start_row + 1, ray.start_column),
            RayDirection::East => (ray.start_row, ray.start_column + 1),
            RayDirection::West => (ray.start_row, ray.start_column - 1),
        };
        match self.get(row, column) {
            None => vec![],
            Some('.') => vec![Ray {
                start_row: row,
                start_column: column,
                direction: ray.direction,
            }],
            Some('|') => match ray.direction {
                RayDirection::North | RayDirection::South => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: ray.direction,
                }],
                RayDirection::East | RayDirection::West => vec![
                    Ray {
                        start_row: row,
                        start_column: column,
                        direction: RayDirection::North,
                    },
                    Ray {
                        start_row: row,
                        start_column: column,
                        direction: RayDirection::South,
                    },
                ],
            },
            Some('-') => match ray.direction {
                RayDirection::North | RayDirection::South => vec![
                    Ray {
                        start_row: row,
                        start_column: column,
                        direction: RayDirection::East,
                    },
                    Ray {
                        start_row: row,
                        start_column: column,
                        direction: RayDirection::West,
                    },
                ],
                RayDirection::East | RayDirection::West => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: ray.direction,
                }],
            },
            Some('\\') => match ray.direction {
                RayDirection::North => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::West,
                }],
                RayDirection::South => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::East,
                }],
                RayDirection::East => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::South,
                }],
                RayDirection::West => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::North,
                }],
            },
            Some('/') => match ray.direction {
                RayDirection::North => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::East,
                }],
                RayDirection::South => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::West,
                }],
                RayDirection::East => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::North,
                }],
                RayDirection::West => vec![Ray {
                    start_row: row,
                    start_column: column,
                    direction: RayDirection::South,
                }],
            },
            Some(_) => unreachable!(),
        }
    }

    fn evaluate(&self, ray: Ray) -> usize {
        let mut energised = BTreeMap::<(isize, isize), u8>::new();

        let mut exploration = vec![ray];
        while let Some(ray) = exploration.pop() {
            let energy = energised
                .entry((ray.start_row, ray.start_column))
                .or_default();
            let mask = match ray.direction {
                RayDirection::North => 1 << 1,
                RayDirection::South => 1 << 2,
                RayDirection::East => 1 << 3,
                RayDirection::West => 1 << 4,
            };
            if *energy & mask > 0 {
                continue;
            }
            *energy |= mask;
            let new_rays = self.emit(ray);
            exploration.extend(new_rays);
        }

        energised.len() - 1
    }
}

struct Ray {
    start_row: isize,
    start_column: isize,
    direction: RayDirection,
}

enum RayDirection {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 46);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 51);
    }
}

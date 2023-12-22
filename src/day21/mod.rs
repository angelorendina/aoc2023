pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day21/input.txt";
    #[cfg(not(test))]
    const STEPS: usize = 64;
    #[cfg(test)]
    const FILENAME: &str = "src/day21/test.txt";
    #[cfg(test)]
    const STEPS: usize = 6;

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut map = Map::new(&input);

    let mut froms = vec![];
    for _ in 0..STEPS {
        for row in 0..map.rows as isize {
            for column in 0..map.columns as isize {
                if let Some(cell) = map.get_mut(row, column) {
                    if let Cell::Visitor = cell {
                        *cell = Cell::Garden;
                        froms.push((row, column));
                    }
                }
            }
        }
        for (row, column) in froms.drain(..) {
            for (row, column) in [
                (row - 1, column),
                (row + 1, column),
                (row, column - 1),
                (row, column + 1),
            ] {
                match map.get_mut(row, column) {
                    Some(cell) if matches!(cell, Cell::Garden) => {
                        *cell = Cell::Visitor;
                    }
                    _ => {}
                }
            }
        }
    }

    map.count()
}

pub fn star_two() -> isize {
    const FILENAME: &str = "src/day21/input.txt";

    let input = std::fs::read_to_string(FILENAME).unwrap();
    let mut base_map = Map::new(&input);
    let mut start = (0, 0);
    base_map.data.iter_mut().enumerate().for_each(|(i, c)| {
        if let Cell::Visitor = c {
            start = (i.div_euclid(base_map.rows), i.rem_euclid(base_map.columns));
            *c = Cell::Garden;
        }
    });

    let mut map = Map {
        rows: base_map.rows * 5,
        columns: base_map.columns * 5,
        data: (0..base_map.rows * base_map.columns * 25)
            .map(|_| Cell::Garden)
            .collect(),
    };
    for i in 0..5 {
        for j in 0..5 {
            for r in 0..base_map.rows {
                for c in 0..base_map.columns {
                    let row = r + i * base_map.rows;
                    let column = c + j * base_map.columns;
                    *map.get_mut(row as isize, column as isize).unwrap() =
                        *base_map.get_mut(r as isize, c as isize).unwrap();
                }
            }
        }
    }
    *map.get_mut(
        (start.0 + 2 * base_map.rows) as isize,
        (start.1 + 2 * base_map.columns) as isize,
    )
    .unwrap() = Cell::Visitor;

    let mut values = vec![];

    for _ in 0..65 {
        map.tick();
    }
    values.push(map.count() as isize);
    for _ in 0..131 {
        map.tick();
    }
    values.push(map.count() as isize);
    for _ in 0..131 {
        map.tick();
    }
    values.push(map.count() as isize);

    // values:
    // [
    //  f(65),
    //  f(65 + 131),
    //  f(65 + 262),
    // ]
    // where f(n) = count of visitors after n steps

    // search g(x) = Ax^2 + Bx + C
    // where g(x) = values[x] = f(65 + 131x)
    // and we want g(202'300) = f(26'501'365)
    // g(0) = v[0] = C
    // g(1) = v[1] = A + B + C
    // g(2) = v[2] = 4A + 2B + C

    // C = v[0]
    // A + B = v[1] - v[0]
    // 4A + 2B = v[2] - v[0]

    // C = v[0]
    // 2A + 2B = 2v[1] - 2v[0]
    // 2A = v[2] - 2v[1] + v[0]

    // C = v[0]
    // A = (v[2] - 2v[1] + v[0]) / 2
    // B = (-v[2] + 4v[1] - 3v[0]) / 2

    // g(202'300)
    (values[2] - 2 * values[1] + values[0]) * 202300 * 101150
        + (-values[2] + 4 * values[1] - 3 * values[0]) * 101150
        + values[0]
}

#[derive(Clone, Copy)]
enum Cell {
    Garden,
    Rock,
    Visitor,
}

struct Map {
    rows: usize,
    columns: usize,
    data: Vec<Cell>,
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
                map.data.push(match c {
                    '.' => Cell::Garden,
                    '#' => Cell::Rock,
                    'S' => Cell::Visitor,
                    _ => unreachable!(),
                });
            }
        }

        map
    }

    #[inline]
    fn get_mut(&mut self, row: isize, column: isize) -> Option<&mut Cell> {
        let Ok(row) = usize::try_from(row) else {
            return None;
        };
        let Ok(column) = usize::try_from(column) else {
            return None;
        };
        if row >= self.rows {
            return None;
        }
        if column >= self.columns {
            return None;
        }
        self.data.get_mut(self.columns * row + column)
    }

    fn count(&self) -> usize {
        self.data
            .iter()
            .filter(|c| matches!(c, Cell::Visitor))
            .count()
    }

    fn tick(&mut self) {
        let mut froms = vec![];
        for row in 0..self.rows as isize {
            for column in 0..self.columns as isize {
                if let Some(cell) = self.get_mut(row, column) {
                    if let Cell::Visitor = cell {
                        *cell = Cell::Garden;
                        froms.push((row, column));
                    }
                }
            }
        }
        for (row, column) in froms.drain(..) {
            for (row, column) in [
                (row - 1, column),
                (row + 1, column),
                (row, column - 1),
                (row, column + 1),
            ] {
                match self.get_mut(row, column) {
                    Some(cell) if matches!(cell, Cell::Garden) => {
                        *cell = Cell::Visitor;
                    }
                    _ => {}
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
        assert_eq!(star_one(), 16);
    }
}

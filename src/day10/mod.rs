use std::collections::BTreeMap;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day10/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day10/test_one.txt";

    let map = std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(Cell::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut start_row, mut start_column) = (0, 0);
    's: for (row, line) in map.iter().enumerate() {
        for (column, c) in line.iter().enumerate() {
            if c == &Cell::S {
                start_row = row;
                start_column = column;
                break 's;
            }
        }
    }

    let mut next = vec![];
    if let Some(r) = start_row.checked_sub(1) {
        if map[r][start_column].has_south() {
            next.push((r, start_column));
        }
    }
    if let Some(c) = start_column.checked_sub(1) {
        if map[start_row][c].has_east() {
            next.push((start_row, c));
        }
    }
    if let Some(line) = map.get(start_row + 1) {
        if line[start_column].has_north() {
            next.push((start_row + 1, start_column));
        }
    }
    if let Some(&cell) = map[start_row].get(start_column + 1) {
        if cell.has_west() {
            next.push((start_row, start_column + 1));
        }
    }
    let [a, b] = next[..] else { unreachable!() };

    let mut paths = [
        [(start_row, start_column), a],
        [(start_row, start_column), b],
    ];

    for steps in 1.. {
        for [from, to] in paths.iter_mut() {
            let [a, b] = map[to.0][to.1].neightbours(to.0, to.1);
            if *from == a {
                *from = *to;
                *to = b;
            } else {
                *from = *to;
                *to = a;
            }
        }

        if paths[0][1] == paths[1][1] {
            return steps + 1;
        }
    }

    unreachable!()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day10/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day10/test_two.txt";

    let mut map = std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(Cell::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut start_row, mut start_column) = (0, 0);
    's: for (row, line) in map.iter().enumerate() {
        for (column, c) in line.iter().enumerate() {
            if c == &Cell::S {
                start_row = row;
                start_column = column;
                break 's;
            }
        }
    }

    let mut directions = [false; 4];
    let mut next = vec![];
    if let Some(r) = start_row.checked_sub(1) {
        if map[r][start_column].has_south() {
            next.push((r, start_column));
            directions[0] = true;
        }
    }
    if let Some(c) = start_column.checked_sub(1) {
        if map[start_row][c].has_east() {
            next.push((start_row, c));
            directions[1] = true;
        }
    }
    if let Some(line) = map.get(start_row + 1) {
        if line[start_column].has_north() {
            next.push((start_row + 1, start_column));
            directions[2] = true;
        }
    }
    if let Some(&cell) = map[start_row].get(start_column + 1) {
        if cell.has_west() {
            next.push((start_row, start_column + 1));
            directions[3] = true;
        }
    }
    let [neightbour, _] = next[..] else {
        unreachable!()
    };
    map[start_row][start_column] = if directions[0] {
        if directions[1] {
            Cell::NW
        } else if directions[2] {
            Cell::NS
        } else {
            Cell::NE
        }
    } else if directions[1] {
        if directions[2] {
            Cell::SW
        } else {
            Cell::EW
        }
    } else {
        Cell::ES
    };

    let mut masked_map = map
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(column, _)| ((row as isize, column as isize), 0usize))
        })
        .collect::<BTreeMap<_, _>>();
    let mut from = (start_row as isize, start_column as isize);
    let mut to = (neightbour.0 as isize, neightbour.1 as isize);
    loop {
        masked_map.insert(from, usize::MAX);
        if to.0 > from.0 {
            // moving south
            masked_map.entry((from.0, from.1 - 1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
            masked_map.entry((from.0 + 1, from.1 - 1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
        } else if to.0 < from.0 {
            // moving north
            masked_map.entry((from.0 - 1, from.1 + 1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
            masked_map.entry((from.0, from.1 + 1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
        } else if to.1 < from.1 {
            // moving west
            masked_map.entry((from.0 - 1, from.1 - 1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
            masked_map.entry((from.0 - 1, from.1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
        } else {
            // moving east
            masked_map.entry((from.0 + 1, from.1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
            masked_map.entry((from.0 + 1, from.1 + 1)).and_modify(|v| {
                if *v < usize::MAX {
                    *v = 1;
                }
            });
        }

        if to == (start_row as isize, start_column as isize) {
            break;
        }
        let [a, b] = map[to.0 as usize][to.1 as usize].neightbours(to.0 as usize, to.1 as usize);
        if from == (a.0 as isize, a.1 as isize) {
            from = to;
            to = (b.0 as isize, b.1 as isize);
        } else {
            from = to;
            to = (a.0 as isize, a.1 as isize);
        }
    }

    loop {
        let points_to_paint = masked_map
            .iter()
            .filter(|(_, &mask)| mask == 1)
            .flat_map(|(&(row, column), _)| {
                [
                    (row - 1, column - 1),
                    (row - 1, column),
                    (row - 1, column + 1),
                    (row, column - 1),
                    (row, column + 1),
                    (row + 1, column - 1),
                    (row + 1, column),
                    (row + 1, column + 1),
                ]
                .into_iter()
                .filter_map(|(row, column)| {
                    if let Some(&mask) = masked_map.get(&(row, column)) {
                        if mask == 0 {
                            Some((row, column))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        if points_to_paint.is_empty() {
            break;
        }

        for point in points_to_paint {
            masked_map.insert(point, 1);
        }
    }

    let mut outer_mask = 0;
    let &bottom_right = masked_map.keys().last().unwrap();
    for (&point, &mask) in masked_map.iter() {
        if (point.0 == 0 || point.0 == bottom_right.0 || point.1 == 0 || point.1 == bottom_right.1)
            && mask == 1
        {
            outer_mask = 1;
            break;
        }
    }

    masked_map
        .into_iter()
        .filter(|(_, mask)| *mask < usize::MAX && *mask != outer_mask)
        .count() as u64
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    O,
    S,
    NE,
    NS,
    NW,
    ES,
    EW,
    SW,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::O,
            'S' => Self::S,
            'L' => Self::NE,
            '|' => Self::NS,
            'J' => Self::NW,
            'F' => Self::ES,
            '-' => Self::EW,
            '7' => Self::SW,
            _ => unreachable!(),
        }
    }
}

impl Cell {
    fn has_south(self) -> bool {
        match self {
            Cell::O => false,
            Cell::S => false,
            Cell::NE => false,
            Cell::NS => true,
            Cell::NW => false,
            Cell::ES => true,
            Cell::EW => false,
            Cell::SW => true,
        }
    }

    fn has_east(self) -> bool {
        match self {
            Cell::O => false,
            Cell::S => false,
            Cell::NE => true,
            Cell::NS => false,
            Cell::NW => false,
            Cell::ES => true,
            Cell::EW => true,
            Cell::SW => false,
        }
    }

    fn has_west(self) -> bool {
        match self {
            Cell::O => false,
            Cell::S => false,
            Cell::NE => false,
            Cell::NS => false,
            Cell::NW => true,
            Cell::ES => false,
            Cell::EW => true,
            Cell::SW => true,
        }
    }

    fn has_north(self) -> bool {
        match self {
            Cell::O => false,
            Cell::S => false,
            Cell::NE => true,
            Cell::NS => true,
            Cell::NW => true,
            Cell::ES => false,
            Cell::EW => false,
            Cell::SW => false,
        }
    }

    fn neightbours(self, row: usize, column: usize) -> [(usize, usize); 2] {
        match self {
            Cell::O => unimplemented!(),
            Cell::S => unimplemented!(),
            Cell::NE => [(row - 1, column), (row, column + 1)],
            Cell::NS => [(row - 1, column), (row + 1, column)],
            Cell::NW => [(row - 1, column), (row, column - 1)],
            Cell::ES => [(row, column + 1), (row + 1, column)],
            Cell::EW => [(row, column + 1), (row, column - 1)],
            Cell::SW => [(row + 1, column), (row, column - 1)],
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
        assert_eq!(star_two(), 10);
    }
}

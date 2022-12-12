use pathfinding::prelude::dijkstra;

#[derive(Debug)]
pub struct Grid {
    start: Pos,
    destination: Pos,
    width: u8,
    height: u8,
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn at(&self, pos: &Pos) -> u8 {
        self.rows[pos.y as usize][pos.x as usize]
    }

    fn is_in(&self, p: &Pos) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width as i16 && p.y < self.height as i16
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    fn up(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Self {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }
}
#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Grid {
    let alphabet = ('a'..='z').collect::<Vec<_>>();
    let mut start = Pos { x: 0, y: 0 };
    let mut destination = Pos { x: 0, y: 0 };
    let rows = input
        .lines()
        .map(|l| l.trim())
        .enumerate()
        .map(|(ri, v)| {
            v.char_indices()
                .map(|(ci, c)| {
                    if c == 'S' {
                        start = Pos {
                            x: ci as i16,
                            y: ri as i16,
                        };
                        0
                    } else if c == 'E' {
                        destination = Pos {
                            x: ci as i16,
                            y: ri as i16,
                        };
                        25
                    } else {
                        alphabet
                            .iter()
                            .enumerate()
                            .find(|(_, v)| **v == c)
                            .unwrap()
                            .0 as u8
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Grid {
        start,
        destination,
        width: rows.first().unwrap().len() as u8,
        height: rows.len() as u8,
        rows,
    }
}

pub fn possible_moves(pos: &Pos, elevation: &u8, grid: &Grid) -> Vec<(Pos, usize)> {
    vec![pos.up(), pos.down(), pos.left(), pos.right()]
        .iter()
        .filter(|o| grid.is_in(o))
        .filter(|p| grid.at(p) as i16 - *elevation as i16 <= 1)
        .map(|p| (*p, 1))
        .collect::<Vec<_>>()
}

#[aoc(day12, part1)]
pub fn solve_part1(grid: &Grid) -> usize {
    let result = dijkstra(
        &grid.start,
        |p| possible_moves(p, &grid.at(&p), grid),
        |p| *p == grid.destination,
    );
    result.unwrap().0.len() - 1
}

#[aoc(day12, part2)]
pub fn solve_part2(grid: &Grid) -> usize {
    let mut starting_points: Vec<Pos> = Vec::new();
    for (ir, row) in grid.rows.iter().enumerate() {
        for (ic, col) in row.iter().enumerate() {
            if *col == 0 {
                starting_points.push(Pos {
                    x: ic as i16,
                    y: ir as i16,
                })
            }
        }
    }
    let mut results = starting_points
        .into_iter()
        .map(|s| {
            dijkstra(
                &s,
                |p| possible_moves(p, &grid.at(&p), grid),
                |p| *p == grid.destination,
            )
        })
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .map(|r| r.0.len() - 1)
        .collect::<Vec<_>>();
    results.sort();
    *results.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    #[test]
    fn it_should_generate_input() {
        let input = input_generator(EXAMPLE_INPUT);

        assert_eq!(input.width, 8);
        assert_eq!(input.height, 5);
        assert_eq!(input.start, Pos { x: 0, y: 0 });
        assert_eq!(input.destination, Pos { x: 5, y: 2 });
        assert_eq!(input.rows[0][0], 0);
        assert_eq!(input.rows[2][5], 25);
        assert_eq!(input.rows[4][7], 8);
    }

    #[test]
    fn it_should_get_possible_moves() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = possible_moves(&input.start, &input.at(&input.start), &input);
        assert_eq!(result.len(), 2);
        assert_eq!(result.contains(&(Pos { x: 1, y: 0 }, 1)), true);
        assert_eq!(result.contains(&(Pos { x: 0, y: 1 }, 1)), true);
    }

    #[test]
    fn it_should_get_possible_moves_1() {
        let input = input_generator(EXAMPLE_INPUT);

        let pos = Pos { x: 3, y: 1 };
        let result = possible_moves(&pos, &input.at(&pos), &input);
        assert_eq!(result.len(), 3);
        assert_eq!(result.contains(&(pos.up(), 1)), true);
        assert_eq!(result.contains(&(pos.down(), 1)), true);
        assert_eq!(result.contains(&(pos.left(), 1)), true);
        assert_eq!(result.contains(&(pos.right(), 1)), false);
    }

    #[test]
    fn it_should_solve_part1() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = solve_part1(&input);
        assert_eq!(result, 31);
    }
}

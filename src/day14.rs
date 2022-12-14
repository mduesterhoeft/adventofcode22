use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn diagonal_left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn diagonal_right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let split = s.trim().split(",").collect::<Vec<_>>();
        Point {
            x: split[0].parse().unwrap(),
            y: split[1].parse().unwrap(),
        }
    }
}

const SOURCE: Point = Point { x: 500, y: 0 };

fn line_to_points(l: &str) -> Vec<Point> {
    l.split(" -> ")
        .map(|p| Point::from(p))
        .collect::<Vec<_>>()
        .windows(2)
        .flat_map(|w| {
            let a = &w[0];
            let b = &w[1];
            if a.x != b.x {
                let diffx = b.x - a.x;
                let range = min(diffx, 0)..max(1, diffx + 1);
                range
                    .map(|p| Point { x: a.x + p, y: a.y })
                    .collect::<Vec<_>>()
            } else if a.y != b.y {
                let diffy = b.y - a.y;
                let range = min(diffy, 0)..max(1, diffy);
                range
                    .map(|p| Point { x: a.x, y: a.y + p })
                    .collect::<Vec<_>>()
            } else {
                vec![a.clone()]
            }
        })
        .collect::<Vec<_>>()
}
#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> HashSet<Point> {
    let lines = input.lines().collect::<Vec<&str>>();
    let all_points = lines
        .into_iter()
        .flat_map(|l| line_to_points(l))
        .collect::<Vec<_>>();
    HashSet::from_iter(all_points.into_iter())
}

fn next_position(points_concrete: &HashSet<Point>, point: &Point) -> Option<Point> {
    let possible_moves = vec![point.down(), point.diagonal_left(), point.diagonal_right()];
    possible_moves
        .into_iter()
        .find(|m| points_concrete.get(m).is_none())
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &HashSet<Point>) -> u32 {
    let mut points_concrete = input.clone();
    let max_y = input.iter().map(|p| p.y).max().unwrap();
    let mut count_units = 0;

    loop {
        let mut current_unit = SOURCE.clone();
        let mut next_move_possible = true;
        while next_move_possible {
            match next_position(&points_concrete, &current_unit) {
                Some(p) => current_unit = p,
                None => next_move_possible = false,
            }
            if current_unit.y > max_y {
                return count_units;
            }
        }
        points_concrete.insert(current_unit.clone());
        count_units += 1;
    }
}

fn horizontal_line_at_bottom(input: &HashSet<Point>) -> HashSet<Point> {
    let max_y = input.iter().map(|p| p.y).max().unwrap();
    (0..1000)
        .map(|i| Point { x: i, y: max_y + 2 })
        .collect::<HashSet<_>>()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &HashSet<Point>) -> u32 {
    let mut points_concrete = input
        .union(&horizontal_line_at_bottom(&input))
        .map(|p| p.clone())
        .collect::<HashSet<_>>();
    let mut count_units = 1;

    loop {
        let mut current_unit = SOURCE.clone();
        let mut next_move_possible = true;
        while next_move_possible {
            match next_position(&points_concrete, &current_unit) {
                Some(p) => current_unit = p,
                None => next_move_possible = false,
            }
            if current_unit.y == 0 {
                return count_units;
            }
        }
        points_concrete.insert(current_unit.clone());
        count_units += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn it_should_generate_input_1() {
        let input = input_generator("498,4 -> 498,6 -> 496,6");
        dbg!(&input);
        assert_eq!(input.len(), 5);
        assert_eq!(input.contains(&Point { x: 498, y: 4 }), true);
        assert_eq!(input.contains(&Point { x: 498, y: 5 }), true);
        assert_eq!(input.contains(&Point { x: 498, y: 6 }), true);
        assert_eq!(input.contains(&Point { x: 497, y: 6 }), true);
        assert_eq!(input.contains(&Point { x: 496, y: 6 }), true);
    }

    #[test]
    fn it_should_generate_input_2() {
        let input = input_generator("503,4 -> 502,4 -> 502,9 -> 494,9");
        dbg!(&input);
        assert_eq!(input.len(), 15);
        assert_eq!(input.contains(&Point { x: 503, y: 4 }), true);
        assert_eq!(input.contains(&Point { x: 502, y: 4 }), true);
        assert_eq!(input.contains(&Point { x: 502, y: 5 }), true);
        assert_eq!(input.contains(&Point { x: 502, y: 6 }), true);
        assert_eq!(input.contains(&Point { x: 502, y: 7 }), true);
        assert_eq!(input.contains(&Point { x: 502, y: 8 }), true);
        assert_eq!(input.contains(&Point { x: 502, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 501, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 500, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 499, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 498, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 497, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 496, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 496, y: 9 }), true);
        assert_eq!(input.contains(&Point { x: 494, y: 9 }), true);
    }

    #[test]
    fn it_should_generate_input_3() {
        let input = input_generator("486,146 -> 490,146");
        dbg!(&input);
        assert_eq!(input.len(), 5);
        assert_eq!(input.contains(&Point { x: 486, y: 146 }), true);
        assert_eq!(input.contains(&Point { x: 487, y: 146 }), true);
        assert_eq!(input.contains(&Point { x: 488, y: 146 }), true);
        assert_eq!(input.contains(&Point { x: 489, y: 146 }), true);
        assert_eq!(input.contains(&Point { x: 490, y: 146 }), true);
    }

    #[test]
    fn it_should_solve_part_1() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = solve_part1(&input);

        assert_eq!(result, 24);
    }

    #[test]
    fn it_should_solve_part_2() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = solve_part2(&input);

        assert_eq!(result, 93);
    }
}

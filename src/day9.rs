use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, Copy)]
pub enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn start() -> Self {
        Pos { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    head_pos: Pos,
    tail_pos: Pos,
    tail_pos_visited: HashSet<Pos>,
}

pub struct SnakeState {
    head_position: Pos,
    rest_positions: Vec<Pos>,
    tail_positions_visited: HashSet<Pos>,
}

impl fmt::Display for SnakeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..25).rev() {
            for k in 0..25 {
                if self.head_position.x == k && self.head_position.y == i {
                    write!(f, "H")?
                } else {
                    write!(
                        f,
                        "{}",
                        self.rest_positions
                            .iter()
                            .enumerate()
                            .find(|(_, p)| p.x == k && p.y == i)
                            .map(|p| (p.0 + 1).to_string())
                            .unwrap_or(".".to_string())
                    )?;
                }
            }
            write!(f, "\n")?
        }
        writeln!(f, "---")
    }
}
impl State {
    fn start_state() -> Self {
        Self {
            head_pos: Pos::start(),
            tail_pos: Pos::start(),
            tail_pos_visited: HashSet::new(),
        }
    }
}

impl SnakeState {
    fn start_state() -> Self {
        Self {
            head_position: Pos::start(),
            rest_positions: (0..9).map(|_| Pos::start()).collect(),
            tail_positions_visited: HashSet::new(),
        }
    }
}
#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input.lines().flat_map(|l| line_to_moves(l)).collect()
}

fn line_to_moves(line: &str) -> Vec<Move> {
    let mut split = line.split_whitespace();
    let direction_str = split.next().unwrap();
    let direction = match direction_str {
        "R" => Move::RIGHT,
        "L" => Move::LEFT,
        "U" => Move::UP,
        "D" => Move::DOWN,
        _ => panic!("unexpected move"),
    };
    let times: u32 = split.next().unwrap().parse().unwrap();
    (0..times).map(|_| direction).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Move>) -> usize {
    let state = input.into_iter().fold(State::start_state(), |state, m| {
        let new_head_pos = move_head(&state.head_pos, m);
        let new_tail_pos = move_tail(&new_head_pos, &state.tail_pos);

        let mut new_tail_pos_visited = HashSet::from(state.tail_pos_visited);
        new_tail_pos_visited.insert(new_tail_pos);

        let new_state = State {
            head_pos: new_head_pos,
            tail_pos: new_tail_pos,
            tail_pos_visited: new_tail_pos_visited,
        };
        new_state
    });
    state.tail_pos_visited.len()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Move>) -> usize {
    let mut state = SnakeState::start_state();
    for m in input {
        state.head_position = move_head(&state.head_position, m);
        let mut last = state.head_position.clone();
        state.rest_positions = state
            .rest_positions
            .into_iter()
            .map(|p| {
                let new_pos = move_tail(&last, &p);
                last = new_pos.clone();
                new_pos
            })
            .collect();
        state
            .tail_positions_visited
            .insert(*state.rest_positions.last().unwrap());
    }

    state.tail_positions_visited.len()
}

fn move_tail(head_pos: &Pos, tail_pos: &Pos) -> Pos {
    let distance_x = head_pos.x as i32 - tail_pos.x as i32;
    let distance_y = head_pos.y as i32 - tail_pos.y as i32;
    let abs_distance_x = i32::abs(distance_x);
    let abs_distance_y = i32::abs(distance_y);

    if abs_distance_x <= 1 && abs_distance_y <= 1 {
        // still neighbors do not move
        return tail_pos.clone();
    } else {
        let x = match i32::abs(distance_x) {
            2 => tail_pos.x + (distance_x / 2),
            1 => tail_pos.x + (distance_x * 1),
            0 => tail_pos.x,
            _ => panic!("invalid x"),
        };
        let y = match i32::abs(distance_y) {
            2 => tail_pos.y + (distance_y / 2),
            1 => tail_pos.y + (distance_y * 1),
            0 => tail_pos.y ,
            _ => panic!("invalid y"),
        };
        return Pos { x, y };
    }
}

fn move_head(pos: &Pos, m: &Move) -> Pos {
    match m {
        Move::UP => Pos {
            x: pos.x,
            y: pos.y + 1,
        },
        Move::DOWN => Pos {
            x: pos.x,
            y: pos.y - 1,
        },
        Move::LEFT => Pos {
            x: pos.x - 1,
            y: pos.y,
        },
        Move::RIGHT => Pos {
            x: pos.x + 1,
            y: pos.y,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_part1() {
        let input = input_generator("R 1\nR 1");
        let result = solve_part1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn it_should_solve_part1_1() {
        let input = input_generator("R 2\nU 2\nD 2\nL 2");
        let result = solve_part1(&input);

        assert_eq!(result, 3);
    }

    #[test]
    fn it_should_solve_part1_example() {
        let input = input_generator(
            "R 4
                U 4
                L 3
                D 1
                R 4
                D 1
                L 5\nR 2",
        );
        let result = solve_part1(&input);

        assert_eq!(result, 13);
    }
    #[test]
    fn it_should_solve_part2_example() {
        let input = input_generator(
            "R 5
                U 8
                L 8
                D 3
                R 17
                D 10
                L 25
                U 20",
        );
        let result = solve_part2(&input);

        assert_eq!(result, 36);
    }
}

use std::collections::VecDeque;

#[derive(Debug)]
pub struct SupplyStacks {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    count: u8,
    from: u8,
    to: u8,
}
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> SupplyStacks {
    let lines = input.lines().collect::<Vec<&str>>();
    let stacks_lines = lines.iter().take(8).map(|s| s.chars().collect::<Vec<_>>()).map(|s| vec![s.get(1).unwrap().clone(), s.get(5).unwrap().clone(), s.get(9).unwrap().clone(), s.get(13).unwrap().clone(), s.get(17).unwrap().clone(), s.get(21).unwrap().clone(),s.get(25).unwrap().clone(),s.get(29).unwrap().clone(),s.get(33).unwrap().clone()]).collect::<Vec<_>>();
    let moves = lines.iter().skip(10).map(|l| l.split_whitespace().collect::<Vec<_>>()).map(|l| Move { count: l.get(1).unwrap().parse::<u8>().unwrap(), from: l.get(3).unwrap().parse::<u8>().unwrap(), to:l.get(5).unwrap().parse::<u8>().unwrap()}).collect::<Vec<_>>();

    let stacks = (0..9).map(|i| {stacks_lines.iter().map(|s| *s.get(i).unwrap()).filter(|c| *c != ' ').collect::<VecDeque<_>>()}).collect::<Vec<_>>();
    SupplyStacks { stacks, moves }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &SupplyStacks) -> String {
    let moves = input.moves.clone();
    let mut stacks = input.stacks.clone().iter().map(|v| v.to_owned()).collect::<Vec<_>>();

    for m in moves {
        (0..m.count).for_each(|_i| {
            let taken =  stacks.get_mut(usize::from(m.from - 1)).unwrap().pop_front();
            if let Some(v) = taken {
                stacks.get_mut(usize::from(m.to - 1)).unwrap().insert(0, v)
            }
        })
    }

    String::from_iter(stacks.iter().map(|s| s.front().unwrap())).replace(" ", "")
}

#[cfg(test)]
mod tests {
    use crate::day5::{solve_part1, SupplyStacks, Move};
    use std::collections::VecDeque;
    #[test]
    fn it_should_solve_part1() {
        let stacks = SupplyStacks {
            stacks: vec![VecDeque::from(['N', 'Z']), VecDeque::from(['D', 'C', 'M']),VecDeque::from(['P'])],
            moves: vec![
                Move { count: 1, from: 2, to: 1},
                Move { count: 3, from: 1, to: 3},
                Move { count: 2, from: 2, to: 1},
                Move { count: 1, from: 1, to: 2},
            ],

        };
        let result = solve_part1(&stacks);
        assert_eq!(result, "CMZ".to_string());
    }
}
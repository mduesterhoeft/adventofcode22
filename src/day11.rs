use std::{collections::HashMap, error::Error};

use sscanf::sscanf;

#[derive(Debug)]
pub enum Operation {
    ADD,
    MULTIPLY,
}
#[derive(Debug)]
pub struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    operation_value: String,
    test_divisible_by: usize,
    test_true_monkey: usize,
    test_false_monkey: usize,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();
    let monkey_lines = lines.chunks(7);
    let mut monkeys: Vec<Monkey> = Vec::new();
    for chunk in monkey_lines {
        let id = sscanf!(chunk[0], "Monkey {usize}:").unwrap();
        let items = chunk[1]
            .trim()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let operation_string = chunk[2];
        let operation_value = operation_string
            .split_whitespace()
            .last()
            .unwrap()
            .to_string();

        let monkey = Monkey {
            id,
            items,
            operation: if operation_string.contains("+") {
                Operation::ADD
            } else if operation_string.contains("*") {
                Operation::MULTIPLY
            } else {
                panic!("invalid operation")
            },
            operation_value,
            test_divisible_by: chunk[3].split_whitespace().last().unwrap().parse().unwrap(),
            test_true_monkey: chunk[4].split_whitespace().last().unwrap().parse().unwrap(),
            test_false_monkey: chunk[5].split_whitespace().last().unwrap().parse().unwrap(),
        };
        monkeys.push(monkey);
    }
    monkeys
}

fn worry_level(
        item: usize,
    operation: &Operation,
    operation_value: &String,
    divide: bool,
        ) -> usize {
    let value: usize = if operation_value == "old" {
        item
    } else {
        operation_value.parse().unwrap()
    };
    let worry_level_handling = match operation {
        Operation::ADD => item + value,
        Operation::MULTIPLY => item * value,
    };

    if divide {
        worry_level_handling / 3
    } else {
        worry_level_handling
    }
}

fn throw_item(
    monkey_items: &mut HashMap<usize, Vec<usize>>,
    item: usize,
    new_item: usize,
    from: usize,
    to: usize,
) {
    let from_items = monkey_items.get_mut(&from).unwrap();
    let remove_from_idx = from_items
        .iter()
        .enumerate()
        .filter(|(i, v)| **v == item)
        .map(|(i, _)| i)
        .next()
        .unwrap();
    from_items.remove(remove_from_idx);

    let to_items = monkey_items.get_mut(&to).unwrap();
    to_items.push(new_item);
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> usize {
    let monkeys = input.clone();
    let mut monkey_items: HashMap<usize, Vec<usize>> = HashMap::new();
    monkeys.iter().for_each(|m| {
        monkey_items.insert(m.id, m.items.clone());
    });
    let mut monkey_count: HashMap<usize, usize> = HashMap::new();
    for round in 1..=20 {
        dbg!(round);
        for monkey in monkeys {
            let monkey_items_copy = monkey_items.clone();
            let items = monkey_items_copy.get(&monkey.id).unwrap();
            for item in items {
                let worry_level =
                    worry_level(*item, &monkey.operation, &monkey.operation_value, true);
                throw_item(
                    &mut monkey_items,
                    *item,
                    worry_level,
                    monkey.id,
                    if worry_level % monkey.test_divisible_by == 0 {
                        monkey.test_true_monkey
                    } else {
                        monkey.test_false_monkey
                    },
                );
                monkey_count
                    .entry(monkey.id)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }
    let mut counts = monkey_count.iter().collect::<Vec<_>>();
    counts.sort_by(|a, b| b.1.cmp(a.1));
    counts.iter().take(2).map(|v| v.1).fold(1, |a, b| a * b)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Monkey]) -> usize {
    let monkeys = input.clone();
    let mut monkey_items: HashMap<usize, Vec<usize>> = HashMap::new();
    monkeys.iter().for_each(|m| {
        monkey_items.insert(m.id, m.items.clone());
    });
    let mut monkey_count: HashMap<usize, usize> = HashMap::new();
    for round in 1..=10000 {
        dbg!(round);
        for monkey in monkeys {
            let monkey_items_copy = monkey_items.clone();
            let items = monkey_items_copy.get(&monkey.id).unwrap();
            for item in items {
                let worry_level =
                    worry_level(*item, &monkey.operation, &monkey.operation_value, false);
                throw_item(
                    &mut monkey_items,
                    *item,
                    worry_level,
                    monkey.id,
                    if worry_level % monkey.test_divisible_by == 0 {
                        monkey.test_true_monkey
                    } else {
                        monkey.test_false_monkey
                    },
                );
                monkey_count
                    .entry(monkey.id)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }
    let mut counts = monkey_count.iter().collect::<Vec<_>>();
    counts.sort_by(|a, b| b.1.cmp(a.1));
    counts.iter().take(2).map(|v| v.1).fold(1, |a, b| a * b)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

    Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

    Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

    Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn it_should_solve_part1() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = solve_part1(&input);

        assert_eq!(result, 10605)
    }

    #[test]
    fn it_should_solve_part2() {
        let input = input_generator(EXAMPLE_INPUT);

        let result = solve_part2(&input);

        assert_eq!(result, 2713310158)
    }
}

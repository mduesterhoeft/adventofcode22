use eval::{eval, Value};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Item {
    Number(i32),
    List(Vec<i32>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Number(n) => match other {
                Self::Number(o) => n.cmp(o),
                Self::List(l) => vec![*n].cmp(l),
            },
            Self::List(l) => match other {
                Self::Number(n) => l.cmp(&vec![*n]),
                Self::List(o) => compare_each(l, o),
            },
        }
    }
}

fn compare_each(left: &Vec<i32>, right: &Vec<i32>) -> Ordering {
    if left.len() == 0 && right.len() > 0 {
        return Ordering::Less;
    };
    for (index, ln) in left.iter().enumerate() {
        let item_result = match right.get(index) {
            Some(rn) => ln.cmp(rn),
            None => Ordering::Greater,
        };
        match item_result {
            Ordering::Greater | Ordering::Less => return item_result,
            _ => (),
        };
    }
    Ordering::Equal
}

fn to_items(line: &str) -> Vec<Item> {
    let e = eval(line.replace("[", "array(").replace("]", ")").as_str()).unwrap();
    let values = e.as_array().unwrap();
    values
        .iter()
        .map(|v| match v {
            eval::Value::Array(i) => Item::List(
                i.iter()
                    .map(|a| {
                        println!("{:?}", a);
                        a.as_u64().unwrap() as i32
                    })
                    .collect(),
            ),
            eval::Value::Number(n) => Item::Number(n.as_i64().unwrap() as i32),
            _ => panic!("unexpected value type"),
        })
        .collect()
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Vec<Item>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| to_items(l))
        .collect::<Vec<_>>()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Vec<Vec<Item>>) -> usize {
    let mut sum = 0;
    for (i, v) in input.chunks(2).enumerate() {
        if v[0].cmp(&v[1]) == Ordering::Less {
            sum += i + 1
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn it_should_generate_input() {}
}

use std::collections::HashSet;

#[aoc_generator(day3, part1)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    let compartment_tuples = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .collect::<Vec<_>>();
    compartment_tuples
        .iter()
        .map(|(l, r)| Rucksack {
            compartment1: String::from(*l),
            compartment2: String::from(*r),
        })
        .collect::<Vec<_>>()
}

pub fn input_generator_part2(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<_>>()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> u32 {
    input.iter().map(|b| b.score()).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let groups = lines.len() / 3;
    let mut lines_iter = lines.iter();
    let s = (0..groups)
        .map(|_| {
            let set1 = lines_iter.next().unwrap().chars().collect::<HashSet<_>>();
            let set2 = lines_iter.next().unwrap().chars().collect::<HashSet<_>>();
            let set3 = lines_iter.next().unwrap().chars().collect::<HashSet<_>>();
            let intersect1 = set1.intersection(&set2).collect::<HashSet<_>>();
            let intersect2 = set2.intersection(&set3).collect::<HashSet<_>>();
            let common = intersect1
                .intersection(&intersect2)
                .map(|i| *i)
                .collect::<Vec<_>>();
            score(&common)
        })
        .sum();
    s
}

#[derive(Debug)]
pub struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl Rucksack {
    fn score(&self) -> u32 {
        let set1 = self.compartment1.chars().collect::<HashSet<_>>();
        let set2 = self.compartment2.chars().collect::<HashSet<_>>();
        let common_items = set1.intersection(&set2).collect::<Vec<_>>();
        score(&common_items)
    }
}

fn score(items: &Vec<&char>) -> u32 {
    let alphabet = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .filter_map(|c| {
            let c = c as char; // Convert to char
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            } // Filter only alphabetic chars
        })
        .collect::<Vec<_>>();
    items.iter().map(|i| score_item(i, &alphabet)).sum()
}

fn score_item(item: &char, alphabet: &Vec<char>) -> u32 {
    match alphabet.iter().position(|c| c == item) {
        Some(p) => (p + 1).try_into().unwrap(),
        None => panic!("char not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part2, Rucksack};
    #[test]
    fn it_should_parse_input() {
        let result = input_generator("jNNBMTNzvTqhQLhQLMQL\nVCwnVRCGHHJTdsLtrdhrGdsq");
        assert_eq!(result.len(), 2);

        let first = result.first().unwrap();
        assert_eq!(first.compartment1, "jNNBMTNzvT");
        assert_eq!(first.compartment2, "qhQLhQLMQL");
    }

    #[test]
    fn it_should_compute_score() {
        let b = Rucksack {
            compartment1: "jNNBMTNzvT".to_string(),
            compartment2: "qhQLhQLMQL".to_string(),
        };

        assert_eq!(b.score(), 39);
    }

    #[test]
    fn it_should_solve_part2_1() {
        let score = solve_part2("AB\nAC\nAD");

        assert_eq!(score, 27);
    }

    #[test]
    fn it_should_solve_part2_2() {
        let score = solve_part2("ba\nbc\nbd");

        assert_eq!(score, 2);
    }

    #[test]
    fn it_should_solve_part2_3() {
        let score = solve_part2("AB\nAC\nAD\nba\nbc\nbd");

        assert_eq!(score, 27 + 2);
    }
}

use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    first_different_n(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    first_different_n(input, 14)
}

fn first_different_n(input: &str, n: usize) -> usize {
    for i in 0..input.len() {
        let window_end = i + n;
        let window: &str = &input[i..window_end];
        let window_set: HashSet<char> = HashSet::from_iter(window.chars());
        if n == window_set.len() {
            return window_end;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_part1_1() {
        let result = solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 5);
    }

    #[test]
    fn it_should_solve_part1_2() {
        let result = solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 11);
    }
}

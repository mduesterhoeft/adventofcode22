#[aoc_generator(day4, par1)]
pub fn input_generator(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input.lines().map(|l| parse_line(l)).collect::<Vec<_>>()
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut range_vecs = line.split(",").map(|r| {
        let mut range_parts = r.split("-");
        let lower = range_parts
            .next()
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap();
        let upper = range_parts
            .next()
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap();
        (lower..=upper).collect::<Vec<_>>()
    });
    (range_vecs.next().unwrap(), range_vecs.next().unwrap())
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    input
        .iter()
        .map(|item| {
            if contains_all(&item.0, &item.1) || contains_all(&item.1, &item.0) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    input
        .iter()
        .map(|item| {
            if contains_any(&item.0, &item.1) || contains_any(&item.1, &item.0) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn contains_all(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    a.iter().all(|i| b.contains(i))
}

fn contains_any(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    a.iter().any(|i| b.contains(i))
}

#[cfg(test)]
mod tests {
    use crate::day4::solve_part1;

    use super::input_generator;
    #[test]
    fn it_should_parse_input() {
        let result = input_generator("8-82,3-96");
        assert_eq!(result.len(), 1);
        let first = result.first().unwrap();
        assert_eq!(first.0, (8..=82).collect::<Vec<_>>());
        assert_eq!(first.1, (3..=96).collect::<Vec<_>>());
    }

    #[test]
    fn it_should_count_many_included() {
        let input = input_generator("1-5,2-3\n1-5,2-5\n3-3,1-5");

        let count = solve_part1(&input);

        assert_eq!(count, 3);
    }

    #[test]
    fn it_should_count_one_included() {
        let input = input_generator("1-5,2-5");

        let count = solve_part1(&input);

        assert_eq!(count, 1);
    }

    #[test]
    fn it_should_count_one_included_1() {
        let input = input_generator("3-3,1-5");

        let count = solve_part1(&input);

        assert_eq!(count, 1);
    }

    #[test]
    fn it_should_not_count_not_included() {
        let input = input_generator("1-5,2-6");
        assert_eq!(input.len(), 1);
        let count = solve_part1(&input);

        assert_eq!(count, 0);
    }

    #[test]
    fn it_should_count_same() {
        let input = input_generator("10-10,10-10");
        assert_eq!(input.len(), 1);
        println!("{:?}", input);
        let count = solve_part1(&input);

        assert_eq!(count, 1);
    }
}

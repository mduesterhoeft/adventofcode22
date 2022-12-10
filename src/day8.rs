use std::collections::HashSet;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Vec<u8>>) -> usize {
    let mut trees_seen: HashSet<(u8, u8)> = HashSet::new();
    for (row_number, row) in input.iter().enumerate() {
        visible_in_sequence_left_to_right(&row)
            .iter()
            .for_each(|i| {
                trees_seen.insert((*i, row_number as u8));
            });

        visible_in_sequence_right_to_left(&row)
            .iter()
            .for_each(|i| {
                trees_seen.insert((*i, row_number as u8));
            });
    }
    for col in 0..input.first().unwrap().len() {
        let column = col_at(col as u8, &input);
        visible_in_sequence_left_to_right(&column)
            .iter()
            .for_each(|i| {
                trees_seen.insert((col as u8, *i));
            });

        visible_in_sequence_right_to_left(&column)
            .iter()
            .for_each(|i| {
                trees_seen.insert((col as u8, *i));
            });
    }
    trees_seen.len()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Vec<u8>>) -> u32 {
    let mut max = 0;
    for row_number in 0..input.len() {
        for col_number in 0..input[0].len() {
            let score = scenic_score((col_number, row_number), &input);
            if score > max {
                max = score
            }
        }
    }
    max
}

fn scenic_score(tree: (usize, usize), matrix: &Vec<Vec<u8>>) -> u32 {
    let height = matrix[tree.1][tree.0];
    let col = col_at(tree.0 as u8, &matrix);
    let row = matrix.get(tree.1).unwrap();

    let down = view_to_right(height, &col.as_slice()[tree.1 + 1..]);
    let up = view_to_left(height, &col.as_slice()[0..tree.1]);
    let right = view_to_right(height, &row.as_slice()[tree.0 + 1..]);
    let left = view_to_left(height, &row.as_slice()[0..tree.0]);

    left * right * up * down
}

fn view_to_right(height: u8, neighbors: &[u8]) -> u32 {
    let mut count = 0;
    for n in neighbors {
        count += 1;
        if *n >= height {
            return count;
        }
    }

    count
}

fn view_to_left(height: u8, neighbors: &[u8]) -> u32 {
    let mut count = 0;
    for n in neighbors.iter().rev() {
        count += 1;
        if *n >= height {
            return count;
        }
    }

    count
}

fn col_at(col: u8, trees: &Vec<Vec<u8>>) -> Vec<u8> {
    trees
        .iter()
        .map(|i| *i.get(col as usize).unwrap())
        .collect()
}

pub fn visible_in_sequence_left_to_right(trees: &Vec<u8>) -> Vec<u8> {
    let mut max: i32 = -1;
    let mut visible: Vec<u8> = Vec::new();
    for (i, v) in trees.iter().enumerate() {
        if *v as i32 > max {
            max = *v as i32;
            visible.push(i as u8);
        }
    }
    visible
}

pub fn visible_in_sequence_right_to_left(trees: &Vec<u8>) -> Vec<u8> {
    let mut rev = trees.clone();
    rev.reverse();
    visible_in_sequence_left_to_right(&rev)
        .iter()
        .map(|i| trees.len() as u8 - i - 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_visible() {
        let result = visible_in_sequence_left_to_right(&vec![2u8, 5u8, 5u8, 1u8, 2u8]);
        assert_eq!(result, vec![0, 1]);

        let result = visible_in_sequence_left_to_right(&vec![5u8, 5u8, 5u8, 1u8, 2u8]);
        assert_eq!(result, vec![0]);

        let result = visible_in_sequence_left_to_right(&vec![0u8, 2u8, 3u8, 1u8, 2u8]);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn should_find_visible_r_to_l() {
        let result = visible_in_sequence_right_to_left(&vec![2u8, 5u8, 5u8, 1u8, 2u8]);
        assert_eq!(result, vec![4, 2]);

        let result = visible_in_sequence_right_to_left(&vec![5u8, 5u8, 5u8, 3u8, 1u8]);
        assert_eq!(result, vec![4, 3, 2]);

        let result = visible_in_sequence_right_to_left(&vec![0u8, 4u8, 3u8, 1u8, 0u8]);
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn should_find_col() {
        let input = input_generator("30373\n25512\n65332\n33549\n35390");
        assert_eq!(col_at(1, &input), vec![0, 5, 5, 3, 5]);
        assert_eq!(col_at(4, &input), vec![3, 2, 2, 9, 0]);
    }

    #[test]
    fn should_solve_part1() {
        let input = input_generator("30373\n25512\n65332\n33549\n35390");
        let result = solve_part1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn should_compute_view() {
        let input = input_generator("30373\n25512\n65332\n33549\n35390");
        let result = view_to_right(4, &[3u8, 4u8]);
        assert_eq!(result, 2);
    }

    #[test]
    fn should_solve_part2() {
        let input = input_generator("30373\n25512\n65332\n33549\n35390");
        let result = solve_part2(&input);
        assert_eq!(result, 8);
    }
}

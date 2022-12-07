use std::collections::HashMap;

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut current_dir: Vec<&str> = Vec::new();
    let mut iter = input.lines().into_iter();

    while let Some(line) = iter.next() {
        if !line.starts_with("$") {
            if !line.starts_with("dir") {
                let size: u32 = line
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap();
                dirs.entry(current_dir.join("-"))
                    .and_modify(|e| *e += size)
                    .or_insert(size);
            }
        };

        if line.starts_with("$ cd ..") {
            current_dir = current_dir.split_last().unwrap().1.to_vec();
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("$ cd") {
            let dir = line.split_whitespace().last().unwrap();
            current_dir.push(dir);
            dirs.entry(current_dir.join("-")).or_insert(0);
        };
    }
    println!("{:?}", count_big_dirs(&dirs));
    count_big_dirs(&dirs)
}

fn count_big_dirs(dirs: &HashMap<String, u32>) -> u32 {
    dirs.iter()
        .map(|(k, _)| sum_size(sub_dirs(k, dirs), dirs))
        .filter(|v| *v <= 100000)
        .sum()
}

fn sum_size(dirs: Vec<String>, all_dirs: &HashMap<String, u32>) -> u32 {
    all_dirs
        .into_iter()
        .filter(|(k, _)| dirs.contains(k))
        .map(|(_, v)| *v)
        .sum()
}
fn sub_dirs(dir: &String, dirs: &HashMap<String, u32>) -> Vec<String> {
    dirs.into_iter()
        .filter(|(k, _)| k.starts_with(dir))
        .map(|(k, _)| k.clone())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_part1_1() {
        let result = solve_part1(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );
        assert_eq!(result, 95437);
    }
}

use camino::*;
use id_tree::*;
pub struct Dir {
    path: Utf8PathBuf,
    size: usize,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Tree<Dir> {
    use id_tree::InsertBehavior::*;
    let mut tree: Tree<Dir> = TreeBuilder::new().build();

    let mut root_path = Utf8PathBuf::new();
    root_path.push("/");

    let mut current_node: NodeId = tree
        .insert(
            Node::new(Dir {
                path: root_path,
                size: 0,
            }),
            AsRoot,
        )
        .unwrap();

    let mut iter = input.lines();
    iter.next().unwrap();

    while let Some(line) = iter.next() {
        if !line.starts_with("$") {
            if !line.starts_with("dir") {
                let split = line.split_whitespace().collect::<Vec<_>>();
                let size: usize = split.first().unwrap().to_string().parse().unwrap();

                current_node = tree
                    .insert(
                        Node::new(Dir {
                            path: split.last().unwrap().into(),
                            size,
                        }),
                        UnderNode(&current_node),
                    )
                    .unwrap();
            }
        };

        if line.starts_with("$ cd ..") {
            current_node = tree.get(&current_node).unwrap().parent().unwrap().clone();
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("$ cd") {
            let dir = line.split_whitespace().last().unwrap();
            let node = tree.get(&current_node).unwrap();
            let mut new_path = node.data().path.clone();
            new_path.push(dir);

            current_node = tree
                .insert(
                    Node::new(Dir {
                        path: new_path,
                        size: 0,
                    }),
                    UnderNode(&current_node),
                )
                .unwrap()
        };
    }
    tree
}

#[aoc(day7, part1)]
pub fn solve_part1(_dirs: &Tree<Dir>) -> u32 {
    1
}

#[aoc(day7, part2)]
pub fn solve_part2(tree: &Tree<Dir>) -> usize {
    const TOTAL_SPACE: usize = 70000000;
    let total_size = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .fold(0, |a, i| a + i.data().size);
    let needed_space = TOTAL_SPACE - total_size;
    total_size - needed_space
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_part1() {
        let input = input_generator(
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
        let result = solve_part1(&input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn it_should_solve_part2() {
        let input = input_generator(
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
        let result = solve_part2(&input);
        assert_eq!(result, 24933642);
    }
}

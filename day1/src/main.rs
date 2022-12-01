use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Elf {
    position: usize,
    score: u32,
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let l = lines.map(|i| i.unwrap());
        let mut elfes = parse_elfes(l.collect());
        elfes.sort_by(|e1, e2| e1.score.cmp(&e2.score));
        elfes.reverse();

        let top = elfes.split_at(3).0;
        println!("top 3 elfes");
        for e in top {
            println!("{:?}", e);
        }
        println!(
            "top 3 elf calories {:?}",
            top.iter().map(|i| i.score).sum::<u32>()
        );

        if let Some(t) = elfes.first() {
            println!(
                "elf with most calories pos {:?} calories {:?}",
                t.position, t.score
            );
        }
    } else {
        println!("error reading file")
    }
}

fn parse_elfes(lines: Vec<String>) -> Vec<Elf> {
    let calories_per_elf = lines.split(|l| l.is_empty());
    let a = calories_per_elf.enumerate().map(|(i, v)| Elf {
        position: i,
        score: v.iter().map(|s| s.parse::<u32>().unwrap()).sum(),
    });
    return a.collect();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

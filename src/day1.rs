#[derive(Debug)]
pub struct Elf {
    score: u32,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    let lines = input.lines().collect::<Vec<&str>>();
    let calories_per_elf = lines.split(|l| l.is_empty());
    let a = calories_per_elf.map(|v| Elf {
        score: v.iter().map(|s| s.parse::<u32>().unwrap()).sum(),
    });
    return a.collect();
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Elf]) -> u32 {
    match elfes_sorted(input).first() {
        Some(c) => c.score,
        None => 0,
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Elf]) -> u32 {
    let elfes = elfes_sorted(input);
    let (top3, _) = elfes.split_at(3);
    top3.iter().map(|e| e.score).sum()
}

fn elfes_sorted(elfes: &[Elf]) -> Vec<&Elf> {
    let mut result = elfes.iter().collect::<Vec<&Elf>>();
    result.sort_by(|e1, e2| e1.score.cmp(&e2.score));
    result.reverse();
    result
}

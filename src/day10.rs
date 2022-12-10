pub enum Operation {
    ADDX(i32),
    NOOP,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            match split.next() {
                Some(v) if v == "noop" => Operation::NOOP,
                Some(v) if v == "addx" => {
                    Operation::ADDX(split.next().unwrap().parse::<i32>().unwrap())
                }
                _ => panic!("invalid input"),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Operation]) -> i32 {
    let cycle_values: Vec<i32> = cycle_values(input);

    let during_20 = cycle_values[18] * 20;
    println!("during 20 {:?}", during_20);
    let during_60 = cycle_values[58] * 60;
    let during_100 = cycle_values[98] * 100;
    let during_140 = cycle_values[138] * 140;
    let during_180 = cycle_values[178] * 180;
    let during_220 = cycle_values[218] * 220;
    during_220 + during_180 + during_140 + during_100 + during_60 + during_20
}

fn cycle_values(input: &[Operation]) -> Vec<i32> {
    let mut register_value = 1;
    let mut cycle_values: Vec<i32> = Vec::new();
    for op in input {
        match op {
            Operation::NOOP => cycle_values.push(register_value),
            Operation::ADDX(v) => {
                cycle_values.push(register_value);
                cycle_values.push(register_value);
                register_value += v;
            }
        }
    }
    cycle_values
}
#[aoc(day10, part2)]
pub fn solve_part2(input: &[Operation]) -> i32 {
    let cycle_values: Vec<i32> = cycle_values(input);
    let mut row = 0;
    cycle_values.iter().enumerate().for_each(|(i, v)| {
        let pos = i - (row * 40);
        let sprite_range_start = if pos == 0 { 0 } else { *v - 1 };
        let sprite_range = sprite_range_start..=(sprite_range_start + 2);

        //println!("value {:?} , i {:?}, pos {:?}, sprit {:?}", v, i, pos, sprite_range);

        if sprite_range.contains(&(pos as i32)) {
            print!("#")
        } else {
            print!(".")
        };
        if (i + 1) % 40 == 0 {
            print!("\n");
            row += 1;
        }
    });
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_part1() {
        let input = input_generator(
            "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop",
        );
        let result = solve_part1(&input);

        assert_eq!(result, 13140);
    }

    #[test]
    fn it_should_solve_part2() {
        let input = input_generator(
            "addx 15
                addx -11
                addx 6
                addx -3
                addx 5
                addx -1
                addx -8
                addx 13
                addx 4
                noop
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx -35
                addx 1
                addx 24
                addx -19
                addx 1
                addx 16
                addx -11
                noop
                noop
                addx 21
                addx -15
                noop
                noop
                addx -3
                addx 9
                addx 1
                addx -3
                addx 8
                addx 1
                addx 5
                noop
                noop
                noop
                noop
                noop
                addx -36
                noop
                addx 1
                addx 7
                noop
                noop
                noop
                addx 2
                addx 6
                noop
                noop
                noop
                noop
                noop
                addx 1
                noop
                noop
                addx 7
                addx 1
                noop
                addx -13
                addx 13
                addx 7
                noop
                addx 1
                addx -33
                noop
                noop
                noop
                addx 2
                noop
                noop
                noop
                addx 8
                noop
                addx -1
                addx 2
                addx 1
                noop
                addx 17
                addx -9
                addx 1
                addx 1
                addx -3
                addx 11
                noop
                noop
                addx 1
                noop
                addx 1
                noop
                noop
                addx -13
                addx -19
                addx 1
                addx 3
                addx 26
                addx -30
                addx 12
                addx -1
                addx 3
                addx 1
                noop
                noop
                noop
                addx -9
                addx 18
                addx 1
                addx 2
                noop
                noop
                addx 9
                noop
                noop
                noop
                addx -1
                addx 2
                addx -37
                addx 1
                addx 3
                noop
                addx 15
                addx -21
                addx 22
                addx -6
                addx 1
                noop
                addx 2
                addx 1
                noop
                addx -10
                noop
                noop
                addx 20
                addx 1
                addx 2
                addx 2
                addx -6
                addx -11
                noop
                noop
                noop",
        );
        let result = solve_part2(&input);

        assert_eq!(result, 13140);
    }
}

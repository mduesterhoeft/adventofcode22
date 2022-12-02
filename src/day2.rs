use std::str::FromStr;

pub struct Round {
    you: Sign,
    opponent: Sign,
}

impl Round {
    fn score(&self) -> u32 {
        self.win_score() + self.you.sign_score()
    }

    fn win_score(&self) -> u32 {
        match self.you {
            Sign::ROCK if self.opponent == Sign::SCISSORS => SCORE_WIN,
            Sign::PAPER if self.opponent == Sign::ROCK => SCORE_WIN,
            Sign::SCISSORS if self.opponent == Sign::PAPER => SCORE_WIN,
            _ if self.you == self.opponent => SCORE_DRAW,
            _ => SCORE_LOST,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Sign {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Sign {
    fn sign_score(&self) -> u32 {
        match self {
            Self::ROCK => 1,
            Self::PAPER => 2,
            Self::SCISSORS => 3,
        }
    }
}

impl FromStr for Sign {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Sign::ROCK),
            "B" | "Y" => Ok(Sign::PAPER),
            "C" | "Z" => Ok(Sign::SCISSORS),
            _ => Err(()),
        }
    }
}
const SCORE_DRAW: u32 = 3;
const SCORE_LOST: u32 = 0;
const SCORE_WIN: u32 = 6;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    let lines = input.lines().collect::<Vec<&str>>();
    lines.iter().map(|l| parse_line(l)).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Round]) -> u32 {
    input.iter().map(|r| r.score()).sum()
}

fn parse_line(line: &str) -> Round {
    let mut split = line.split_whitespace();
    let opponent = Sign::from_str(split.next().unwrap()).unwrap();
    let you = Sign::from_str(split.next().unwrap()).unwrap();
    Round { you, opponent }
}

#[cfg(test)]
mod tests {
    use super::{Round, Sign};
    #[test]
    fn it_should_compute_sign_score() {
        assert_eq!(Sign::ROCK.sign_score(), 1);
        assert_eq!(Sign::PAPER.sign_score(), 2);
        assert_eq!(Sign::SCISSORS.sign_score(), 3);
    }

    #[test]
    fn it_should_compute_round_score_win() {
        let round = Round {
            you: Sign::PAPER,
            opponent: Sign::ROCK,
        };
        assert_eq!(round.win_score(), 6);
        assert_eq!(round.score(), round.win_score() + round.you.sign_score());
    }

    #[test]
    fn it_should_compute_round_score_draw() {
        let round = Round {
            you: Sign::ROCK,
            opponent: Sign::ROCK,
        };
        assert_eq!(round.win_score(), 3);
        assert_eq!(round.score(), round.win_score() + round.you.sign_score());
    }

    #[test]
    fn it_should_compute_round_score_loose() {
        let round = Round {
            you: Sign::ROCK,
            opponent: Sign::PAPER,
        };
        assert_eq!(round.win_score(), 0);
        assert_eq!(round.score(), round.win_score() + round.you.sign_score());
    }
}

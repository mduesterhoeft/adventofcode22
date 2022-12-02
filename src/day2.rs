use enum_iterator::{all, Sequence};
use std::str::FromStr;

pub struct Round {
    you: Sign,
    opponent: Sign,
}

impl Round {
    fn score(&self) -> u32 {
        self.outcome().score() + self.you.sign_score()
    }

    fn outcome(&self) -> Outcome {
        match self.you {
            Sign::ROCK if self.opponent.looses_against() == Sign::ROCK => Outcome::WIN,
            Sign::PAPER if self.opponent.looses_against() == Sign::PAPER => Outcome::WIN,
            Sign::SCISSORS if self.opponent.looses_against() == Sign::SCISSORS => Outcome::WIN,
            _ if self.you == self.opponent => Outcome::DRAW,
            _ => Outcome::LOOSE,
        }
    }
}

enum Outcome {
    WIN,
    LOOSE,
    DRAW,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::WIN => 6,
            Self::LOOSE => 0,
            Self::DRAW => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(Outcome::LOOSE),
            "Y" => Ok(Outcome::DRAW),
            "Z" => Ok(Outcome::WIN),
            _ => Err(()),
        }
    }
}
pub struct RoundPart2 {
    opponent: Sign,
    outcome: Outcome,
}

impl RoundPart2 {
    fn score(&self) -> u32 {
        let own_sign = self.own_sign();
        let round = Round {
            opponent: self.opponent,
            you: own_sign,
        };
        round.outcome().score() + own_sign.sign_score()
    }

    fn own_sign(&self) -> Sign {
        match self.outcome {
            Outcome::DRAW => self.opponent,
            Outcome::LOOSE => self.opponent.wins_against(),
            Outcome::WIN => self.opponent.looses_against(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Sequence)]
pub enum Sign {
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

    fn wins_against(&self) -> Sign {
        match self {
            Self::ROCK => Self::SCISSORS,
            Self::PAPER => Self::ROCK,
            Self::SCISSORS => Self::PAPER,
        }
    }

    fn looses_against(&self) -> Self {
        let wins = self.wins_against();
        for sign in all::<Self>().collect::<Vec<_>>() {
            if sign != wins && sign != *self {
                return sign;
            }
        }
        panic!("")
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

impl FromStr for Round {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split = input.split_whitespace();
        let opponent = Sign::from_str(split.next().unwrap())?;
        let you = Sign::from_str(split.next().unwrap())?;
        Ok(Round { you, opponent })
    }
}

impl FromStr for RoundPart2 {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split = input.split_whitespace();
        let opponent = Sign::from_str(split.next().unwrap())?;
        let outcome = Outcome::from_str(split.next().unwrap())?;
        Ok(RoundPart2 { opponent, outcome })
    }
}

#[aoc_generator(day2, part1)]
pub fn input_generator(input: &str) -> Vec<Round> {
    let lines = input.lines().collect::<Vec<&str>>();
    lines.iter().map(|l| Round::from_str(l).unwrap()).collect()
}

#[aoc_generator(day2, part2)]
pub fn input_generator2(input: &str) -> Vec<RoundPart2> {
    let lines = input.lines().collect::<Vec<&str>>();
    lines
        .iter()
        .map(|l| RoundPart2::from_str(l).unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Round]) -> u32 {
    input.iter().map(|r| r.score()).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[RoundPart2]) -> u32 {
    input.iter().map(|r| r.score()).sum()
}

#[cfg(test)]
mod tests {
    use super::{FromStr, Outcome, Round, RoundPart2, Sign};
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
        assert_eq!(round.outcome().score(), 6);
        assert_eq!(
            round.score(),
            round.outcome().score() + round.you.sign_score()
        );
    }

    #[test]
    fn it_should_compute_round_score_draw() {
        let round = Round {
            you: Sign::ROCK,
            opponent: Sign::ROCK,
        };
        assert_eq!(round.outcome().score(), 3);
        assert_eq!(
            round.score(),
            round.outcome().score() + round.you.sign_score()
        );
    }

    #[test]
    fn it_should_compute_round_score_loose() {
        let round = Round {
            you: Sign::ROCK,
            opponent: Sign::PAPER,
        };
        assert_eq!(round.outcome().score(), 0);
        assert_eq!(
            round.score(),
            round.outcome().score() + round.you.sign_score()
        );
    }

    #[test]
    fn it_should_parse_sign() {
        let line = "A X";
        let round = Round::from_str(line).unwrap();
        assert_eq!(round.you, Sign::ROCK);
        assert_eq!(round.opponent, Sign::ROCK);
    }

    #[test]
    fn it_score_round_2_loose() {
        let line = "A X";
        let round = RoundPart2::from_str(line).unwrap();
        assert_eq!(
            round.score(),
            Outcome::LOOSE.score() + Sign::SCISSORS.sign_score()
        );
    }

    #[test]
    fn it_score_round_2_win() {
        let line = "B Z";
        let round = RoundPart2::from_str(line).unwrap();
        assert_eq!(
            round.score(),
            Outcome::WIN.score() + Sign::SCISSORS.sign_score()
        );
    }

    #[test]
    fn it_score_round_2_draw() {
        let line = "C Y";
        let round = RoundPart2::from_str(line).unwrap();
        assert_eq!(
            round.score(),
            Outcome::DRAW.score() + Sign::SCISSORS.sign_score()
        );
    }
}

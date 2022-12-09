use std::str::FromStr;

#[derive(Debug)]
struct ParseStategyErr {}
enum OutCome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for OutCome {
    type Err = ParseStategyErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            &_ => return Err(ParseStategyErr {}),
        };
        Ok(res)
    }
}

enum Shapes {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shapes {
    type Err = ParseStategyErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            &_ => return Err(ParseStategyErr {}),
        };
        Ok(shape)
    }
}

mod game {
    use super::*;

    pub fn play_one(round: &str) -> u32 {
        // skip empty lines
        if round.is_empty() {
            return 0;
        }
        let moves: Vec<&str> = round.split(' ').collect();
        let opponent: Shapes = moves[0].parse().unwrap();
        let you: Shapes = moves[1].parse().unwrap();
        let match_score = match you {
            Shapes::Rock => match opponent {
                Shapes::Rock => OutCome::Draw,
                Shapes::Paper => OutCome::Loose,
                Shapes::Scissors => OutCome::Win,
            },
            Shapes::Paper => match opponent {
                Shapes::Rock => OutCome::Win,
                Shapes::Paper => OutCome::Draw,
                Shapes::Scissors => OutCome::Loose,
            },
            Shapes::Scissors => match opponent {
                Shapes::Rock => OutCome::Loose,
                Shapes::Paper => OutCome::Win,
                Shapes::Scissors => OutCome::Draw,
            },
        };
        match_score as u32 + you as u32
    }

    pub fn play_two(round: &str) -> u32 {
        // skip empty lines
        if round.is_empty() {
            return 0;
        }
        let moves: Vec<&str> = round.split(' ').collect();
        let opponent: Shapes = moves[0].parse().unwrap();
        let outcome: OutCome = moves[1].parse().unwrap();
        let your_shape = match opponent {
            Shapes::Rock => match outcome {
                OutCome::Win => Shapes::Paper,
                OutCome::Draw => Shapes::Rock,
                OutCome::Loose => Shapes::Scissors,
            },
            Shapes::Paper => match outcome {
                OutCome::Win => Shapes::Scissors,
                OutCome::Draw => Shapes::Paper,
                OutCome::Loose => Shapes::Rock,
            },
            Shapes::Scissors => match outcome {
                OutCome::Win => Shapes::Rock,
                OutCome::Draw => Shapes::Scissors,
                OutCome::Loose => Shapes::Paper,
            },
        };
        outcome as u32 + your_shape as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input.split('\n').map(|round| game::play_one(round)).sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input.split('\n').map(|round| game::play_two(round)).sum();
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

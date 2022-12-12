use std::{str::FromStr, string::ParseError};

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        Ok(Self {
            start: parts.next().unwrap().parse::<u32>().unwrap(),
            end: parts.next().unwrap().parse::<u32>().unwrap(),
        })
    }
}

impl Range {
    fn fully_contain(self, other: Self) -> bool {
        (self.start >= other.start && self.end <= other.end)
            || (other.start >= self.start && other.end <= self.end)
    }

    fn overlap_at_all(self, other: Self) -> bool {
        (other.start <= self.end && self.start <= other.end)
            || (self.start <= other.end && other.start <= self.end)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let num_overlaps = input
        .lines()
        .map(|l| {
            let parts: Vec<Range> = l.split(',').map(|p| p.parse::<Range>().unwrap()).collect();
            if parts[0].fully_contain(parts[1]) {
                1
            } else {
                0
            }
        })
        .sum();
    Some(num_overlaps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_overlaps = input
        .lines()
        .map(|l| {
            let parts: Vec<Range> = l.split(',').map(|p| p.parse::<Range>().unwrap()).collect();
            if parts[0].overlap_at_all(parts[1]) {
                1
            } else {
                0
            }
        })
        .sum();
    Some(num_overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_no_overlap() {
        let input = "2-4,5-6";
        assert_eq!(part_two(input), Some(0))
    }

    #[test]
    fn test_overlap() {
        let input = "2-7,1-6";
        assert_eq!(part_two(input), Some(1))
    }

    #[test]
    fn test_degenerated() {
        let input = "7-2,1-6";
        assert_eq!(part_two(input), Some(0))
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

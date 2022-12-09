use std::collections::HashSet;

fn appears_in_both(input: &str) -> char {
    // split the string in two and check which char is in both
    let middle = input.len() / 2;
    let first: HashSet<_> = HashSet::from_iter(input[..middle].chars());
    let second: HashSet<_> = HashSet::from_iter(input[middle..input.len()].chars());
    let duplicated_items: Vec<&char> = first.intersection(&second).collect();
    duplicated_items[0].to_owned()
}

fn get_item_priority(item: char) -> u32 {
    match item.is_ascii_uppercase() {
        true => item as u32 - 'A' as u32 + 27,
        false => item as u32 - 'a' as u32 + 1,
    }
}

fn get_group_badge(elves: &[&str]) -> char {
    let first: HashSet<_> = HashSet::from_iter(elves[0].chars());
    let second: HashSet<_> = HashSet::from_iter(elves[1].chars());
    let third: HashSet<_> = HashSet::from_iter(elves[2].chars());
    let intersect: HashSet<_> =
        HashSet::from_iter(second.intersection(&third).collect::<String>().chars());
    let common: Vec<&char> = intersect.intersection(&first).collect();
    common[0].to_owned()
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_priority = input
        .trim()
        .split('\n')
        .map(|sack| get_item_priority(appears_in_both(sack)))
        .sum();
    Some(total_priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves: Vec<&str> = input.trim().split('\n').collect();
    let total_badges = elves
        .chunks(3)
        .map(|group| get_item_priority(get_group_badge(group)))
        .sum();
    Some(total_badges)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appears_in_both_p() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(appears_in_both(&input), 'p');
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('b'), 2);
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('Z'), 52);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

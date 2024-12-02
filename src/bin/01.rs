use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            (
                a.trim().parse::<u32>().unwrap(),
                b.trim().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let res = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| b.abs_diff(*a))
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            (
                a.trim().parse::<u32>().unwrap(),
                b.trim().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    right.sort_unstable();

    let mut occurences = HashMap::new();
    let res = left
        .iter()
        .map(|n| {
            let find_match_to_n = || {
                right
                    .iter()
                    .take_while(|x| **x <= *n)
                    .filter(|x| **x == *n)
                    .count()
            };
            let x = occurences.entry(n).or_insert(find_match_to_n());

            *n * *x as u32
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

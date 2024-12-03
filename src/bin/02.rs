use itertools::Itertools;

advent_of_code::solution!(2);

fn str_to_i32(input: &str) -> i32 {
    input.parse::<i32>().expect("str not a valid u32")
}

fn is_safe(input: &[i32]) -> bool {
    let sorted = input.is_sorted() || input.is_sorted_by(|a, b| a > b);
    if sorted {
        let mut iter = input.iter().peekable();

        while let Some(&val) = iter.next() {
            if let Some(&&next_val) = iter.peek() {
                if !(1..=3).contains(&val.abs_diff(next_val)) {
                    return false;
                }
            }
        }
    }

    sorted
}

pub fn part_one(input: &str) -> Option<u32> {
    let n = input
        .lines()
        .filter(|line| {
            let line = line.split_whitespace().map(str_to_i32).collect_vec();

            is_safe(&line)
        })
        .count();

    Some(n as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let n = input
        .lines()
        .filter(|line| {
            let line = line.split_whitespace().map(str_to_i32).collect_vec();

            for i in 0..(line.len()) {
                let input = if i < line.len() {
                    [&line[..i], &line[(i + 1)..]].concat()
                } else {
                    line[..i].into()
                };
                if is_safe(&input) {
                    return true;
                }
            }

            false
        })
        .count();

    Some(n as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

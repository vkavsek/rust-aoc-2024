use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").expect("invalid regex!");
    let result: usize = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            let a: usize = atoi::atoi(a.as_bytes()).unwrap();
            let b: usize = atoi::atoi(b.as_bytes()).unwrap();

            a * b
        })
        .sum();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re =
        Regex::new(r"(mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\))|(?<do>do\(\))|(?<dont>don\'t\(\))")
            .expect("invalid regex!");

    let mut calculate = true;
    let result: usize = re
        .captures_iter(input)
        .map(|caps| {
            if caps.name("dont").is_some() {
                calculate = false;
            }
            if caps.name("do").is_some() {
                calculate = true;
            }

            if calculate {
                if let Some(a) = caps.name("a") {
                    if let Some(b) = caps.name("b") {
                        let a: usize = atoi::atoi(a.as_str().as_bytes()).unwrap();
                        let b: usize = atoi::atoi(b.as_str().as_bytes()).unwrap();

                        return a * b;
                    }
                }
            }

            0
        })
        .sum();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}

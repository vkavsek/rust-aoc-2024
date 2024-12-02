use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Inc,
    Dec,
}

fn compute_dir(a: u32, b: u32) -> Option<Dir> {
    match a.cmp(&b) {
        Ordering::Less => Some(Dir::Inc),
        Ordering::Greater => Some(Dir::Dec),
        Ordering::Equal => None,
    }
}

fn str_to_u32(input: &str) -> u32 {
    input.parse::<u32>().expect("str not a valid u32")
}

fn is_valid_dif(x: u32, y: u32) -> bool {
    (1..=3).contains(&x.abs_diff(y))
}

pub fn part_one(input: &str) -> Option<u32> {
    let n = input
        .lines()
        .filter(|line| {
            let (a, b) = line
                .split_whitespace()
                .take(2)
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();

            if !(1..=3).contains(&a.abs_diff(b)) {
                return false;
            }

            let dir = compute_dir(a, b);
            let mut prev = b;

            for n in line.split_whitespace().skip(2) {
                let n = str_to_u32(n);
                let new_dir = compute_dir(prev, n);
                let valid_dir = dir == new_dir;

                if !valid_dir || !is_valid_dif(n, prev) {
                    return false;
                }

                prev = n;
            }

            true
        })
        .count();

    Some(n as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let n = input
    //     .lines()
    //     .filter(|line| {
    //         let first_vals = line
    //             .split_whitespace()
    //             .take(4)
    //             .map(str_to_u32)
    //             .collect_vec();
    //         let v = StartVals::find(first_vals.as_slice());
    //
    //         let mut dampened = 0;
    //         let mut dir = compute_dir(a, b);
    //         let mut prev = b;
    //
    //         if !(1..=3).contains(&a.abs_diff(b)) || dir.is_none() {
    //             dampened += 1;
    //             dir = None;
    //         }
    //
    //         for n in line.split_whitespace().skip(2) {
    //             let n = str_to_u32(n);
    //             let new_dir = compute_dir(prev, n);
    //             let valid_dir = dir == new_dir;
    //
    //             if dir.is_none() || !valid_dir {
    //                 if new_dir.is_some() {
    //                     dir = new_dir;
    //                 } else {
    //                     return false;
    //                 }
    //             }
    //
    //             let valid_diff = (1..=3).contains(&n.abs_diff(prev));
    //
    //             if !valid_dir || !valid_diff {
    //                 dampened += 1;
    //             } else {
    //                 prev = n;
    //             }
    //             if dampened > 1 {
    //                 dbg!(&prev);
    //                 dbg!(&n);
    //                 dbg!(line);
    //                 return false;
    //             }
    //         }
    //
    //         true
    //     })
    //     .count();

    None
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
        assert_eq!(result, Some(6));
    }
}

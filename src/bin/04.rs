advent_of_code::solution!(4);

use itertools::Itertools;

const DIRS: [(i32, i32); 8] = [
    (0, -1),  // -y UP
    (-1, -1), // -y -x UPLEFT
    (1, -1),  // -y +x UPRIGHT
    (0, 1),   // +y DOWN
    (-1, 1),  // +y -x DOWNLEFT
    (1, 1),   // +y +x DOWNRIGHT
    (-1, 0),  // -x LEFT
    (1, 0),   // +x RIGHT
];

fn find_xmas(input: &[Vec<char>], xpos: usize, linenum: usize) -> u32 {
    let xpos_i32: i32 = xpos.try_into().unwrap();
    let ypos_i32: i32 = linenum.try_into().unwrap();

    let mut result = 0;
    for dir in DIRS {
        let s = (1..=3)
            .filter_map(|i| {
                let x = xpos_i32 + (dir.0 * i);
                let y = ypos_i32 + (dir.1 * i);
                // Convert back to usize or return None
                let x: usize = x.try_into().ok()?;
                let y: usize = y.try_into().ok()?;

                // get elem
                input.get(y).and_then(|line| line.get(x))
            })
            .collect::<String>();

        if s == "MAS" {
            result += 1;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| line.to_uppercase().chars().collect_vec())
        .collect_vec();

    let mut result = 0;
    for (linenum, line) in input.iter().enumerate() {
        for xpos in line.iter().positions(|c| c == &'X') {
            let found = find_xmas(&input, xpos, linenum);
            result += found;
        }
    }

    Some(result)
}

// (s|m(-1, -1) && m|s(1, 1))   &&    (s|m(-1, 1) && m|s(1, -1))
fn find_x_mas(input: &[Vec<char>], xpos: usize, linenum: usize) -> bool {
    let xpos_i32: i32 = xpos.try_into().unwrap();
    let ypos_i32: i32 = linenum.try_into().unwrap();

    let mut buf = [' '; 4];
    let dirs = [1, -1];
    let mut i = 0;
    for ydir in dirs {
        for xdir in dirs {
            let x = xpos_i32 + xdir;
            let y = ypos_i32 + ydir;
            if x < 0 || y < 0 {
                return false;
            }

            // get elem or return
            let Some(elem) = input
                .get(y as usize)
                .and_then(|line| line.get(x as usize).copied())
            else {
                return false;
            };

            buf[i] = elem;

            i += 1;
        }
    }

    buf == ['S', 'S', 'M', 'M']
        || buf == ['S', 'M', 'S', 'M']
        || buf == ['M', 'S', 'M', 'S']
        || buf == ['M', 'M', 'S', 'S']
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| line.to_uppercase().chars().collect_vec())
        .collect_vec();

    let mut result: usize = 0;
    for (linenum, line) in input.iter().enumerate() {
        let matches = line
            .iter()
            .positions(|c| c == &'A')
            .filter(|&apos| find_x_mas(&input, apos, linenum))
            .count();

        result += matches;
    }

    Some(result.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

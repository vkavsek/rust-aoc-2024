use std::collections::HashSet;

use advent_of_code::utils::{Grid, ORTHO_DIRS};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let starts = grid.find_all(b'0');

    let res: usize = starts
        .par_iter()
        .map(|&start| {
            let mut positions = Vec::with_capacity(512);
            for dir in ORTHO_DIRS {
                let new_pos = start + dir;
                if grid.contains(new_pos) && grid[new_pos] == grid[start] + 1 {
                    positions.push(new_pos);
                }
            }

            let mut sum = 0;
            let mut operating_positions = Vec::with_capacity(512);
            let mut found_ends = HashSet::with_capacity(64);

            loop {
                if positions.is_empty() {
                    break;
                }

                for pos in positions.iter() {
                    for new_dir in ORTHO_DIRS.into_iter() {
                        let new_pos = *pos + new_dir;
                        if grid.contains(new_pos) && grid[new_pos] == grid[*pos] + 1 {
                            let new_val = grid[new_pos];
                            if new_val == b'9' {
                                if found_ends.insert(new_pos) {
                                    sum += 1;
                                }
                                continue;
                            }

                            operating_positions.push(new_pos);
                        }
                    }
                }

                std::mem::swap(&mut positions, &mut operating_positions);
                operating_positions.clear();
            }

            sum
        })
        .sum();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let starts = grid.find_all(b'0');

    let res: usize = starts
        .par_iter()
        .map(|&start| {
            let mut positions = Vec::with_capacity(512);
            for dir in ORTHO_DIRS {
                let new_pos = start + dir;
                if grid.contains(new_pos) && grid[new_pos] == grid[start] + 1 {
                    positions.push(new_pos);
                }
            }

            let mut sum = 0;
            let mut operating_positions = Vec::with_capacity(512);

            loop {
                if positions.is_empty() {
                    break;
                }

                for pos in positions.iter() {
                    for new_dir in ORTHO_DIRS.into_iter() {
                        let new_pos = *pos + new_dir;
                        if grid.contains(new_pos) && grid[new_pos] == grid[*pos] + 1 {
                            let new_val = grid[new_pos];
                            if new_val == b'9' {
                                sum += 1;
                                continue;
                            }

                            operating_positions.push(new_pos);
                        }
                    }
                }

                std::mem::swap(&mut positions, &mut operating_positions);
                operating_positions.clear();
            }

            sum
        })
        .sum();

    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

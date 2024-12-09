use indexmap::IndexSet;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::collections::HashSet;

// TODO: working but ugly and slow. improve?

advent_of_code::solution!(6);

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

type Map = Vec<Vec<Pos>>;

#[derive(Debug, Clone, Copy)]
enum Pos {
    Empty,
    Blocked,
}

impl From<char> for Pos {
    fn from(value: char) -> Self {
        if value == '#' {
            Pos::Blocked
        } else {
            Pos::Empty
        }
    }
}

fn step(origin: &(i32, i32), dir: &(i32, i32)) -> (i32, i32) {
    (origin.0 + dir.0, origin.1 + dir.1)
}

fn get_next_guard_loc(map: &Map, next_loc: (i32, i32)) -> Option<&Pos> {
    map.get(next_loc.1 as usize)
        .and_then(|line| line.get(next_loc.0 as usize))
}

fn map_and_guard_loc(input: &str) -> (Map, (i32, i32)) {
    let mut guard_loc = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        guard_loc = Some((x, y));
                    }
                    c.into()
                })
                .collect_vec()
        })
        .collect_vec();
    // assert to ensure correct conversion
    assert!(map.len() < (i32::MAX as usize) && map[0].len() < (i32::MAX as usize));
    let guard_loc = guard_loc
        .map(|(x, y)| (x as i32, y as i32))
        .expect("Guard location not found!");

    (map, guard_loc)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut guard_loc) = map_and_guard_loc(input);

    let mut locations = HashSet::from([guard_loc]);
    'out: loop {
        'dirs_loop: for dir in &DIRS {
            let mut next_loc = step(&guard_loc, dir);

            while let Some(next_guard_loc) = get_next_guard_loc(&map, next_loc) {
                if next_loc.0 < 0 || next_loc.1 < 0 {
                    break 'out;
                }

                match next_guard_loc {
                    Pos::Empty => {
                        guard_loc = next_loc;
                        locations.insert(guard_loc);
                        next_loc = step(&guard_loc, dir);
                    }
                    Pos::Blocked => continue 'dirs_loop,
                }
            }

            if get_next_guard_loc(&map, next_loc).is_none() {
                break 'out;
            }
        }
    }

    let result = locations.len().try_into().expect("need USIZE!");

    Some(result)
}

type LocationsAndDirs = IndexSet<((i32, i32), (i32, i32))>;

fn find_looping_paths(locs_and_dirs: &LocationsAndDirs, map: &Map) -> usize {
    let mut tested_locs = HashSet::new();
    let result = locs_and_dirs
        .iter()
        .progress()
        .filter(|(start_loc, start_dir)| {
            let mut encountered_locs_and_dirs = HashSet::new();
            let addpos = step(start_loc, start_dir);

            // if position is invalid in the first place do nothing
            if get_next_guard_loc(map, addpos).is_none() || addpos.0 < 0 || addpos.1 < 0 {
                return false;
            }
            // This position has been tested already so it can't be counted again
            if !tested_locs.insert(addpos) {
                return false;
            }

            // Insert into map clone
            let mut modified_map = map.clone();
            modified_map[addpos.1 as usize][addpos.0 as usize] = Pos::Blocked;
            let mut guard_loc = *start_loc;

            let dir_id = DIRS
                .iter()
                .position(|d| *d == *start_dir)
                .expect("invalid dir");
            let dir_id = if dir_id == DIRS.len() - 1 {
                0
            } else {
                dir_id + 1
            };
            let shifted_dirs = [&DIRS[dir_id..], &DIRS[..dir_id]].concat();

            loop {
                'dirs_loop: for dir in &shifted_dirs {
                    let mut next_loc = step(&guard_loc, dir);

                    while let Some(next_guard_loc) = get_next_guard_loc(&modified_map, next_loc) {
                        if !encountered_locs_and_dirs.insert((guard_loc, *dir)) {
                            return true;
                        }

                        if next_loc.0 < 0 || next_loc.1 < 0 {
                            return false;
                        }

                        match next_guard_loc {
                            Pos::Empty => {
                                guard_loc = next_loc;
                                next_loc = step(&guard_loc, dir);
                            }
                            Pos::Blocked => continue 'dirs_loop,
                        }
                    }

                    if get_next_guard_loc(map, next_loc).is_none() {
                        return false;
                    }
                }
            }
        })
        .count();

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, mut guard_loc) = map_and_guard_loc(input);

    let mut locations_with_dirs = IndexSet::new();
    'out: loop {
        'dirs_loop: for dir in &DIRS {
            let mut next_loc = step(&guard_loc, dir);

            while let Some(next_guard_loc) = get_next_guard_loc(&map, next_loc) {
                if next_loc.0 < 0 || next_loc.1 < 0 {
                    break 'out;
                }

                match next_guard_loc {
                    Pos::Empty => {
                        locations_with_dirs.insert((guard_loc, *dir));
                        guard_loc = next_loc;
                        next_loc = step(&guard_loc, dir);
                    }
                    Pos::Blocked => continue 'dirs_loop,
                }
            }

            if get_next_guard_loc(&map, next_loc).is_none() {
                break 'out;
            }
        }
    }

    let result = find_looping_paths(&locations_with_dirs, &map);

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

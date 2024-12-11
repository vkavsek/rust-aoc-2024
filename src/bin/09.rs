use std::{
    collections::HashSet,
    iter,
    ops::{Deref, DerefMut},
};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut repr = generate_memory_representation(input);
    let mut last_file = repr.len();
    let mut first_empty = 0;

    loop {
        last_file = repr[..last_file]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, v)| v.map(|_| i))
            .expect("invalid input: missing Some");

        first_empty += repr[first_empty..]
            .iter()
            .enumerate()
            .find_map(|(i, v)| if v.is_none() { Some(i) } else { None })
            .expect("invalid input: missing None");

        if first_empty >= last_file {
            break;
        }

        repr.swap(first_empty, last_file);
    }

    let res: usize = repr
        .iter()
        .enumerate()
        .filter_map(|(i, o)| o.map(|v| v * i))
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut repr = generate_memory_representation(input);
    // last file end range
    let mut last_file_er = repr.len();
    let mut checked_file_nums = HashSet::with_capacity(2000);

    'outer: loop {
        let mut operating_file_num = None;
        let Some(lf_er) = repr[..last_file_er]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, v)| {
                let x = (*v)?;

                if checked_file_nums.contains(&x) {
                    None
                } else {
                    operating_file_num = Some(x);
                    Some(i)
                }
            })
        else {
            break;
        };
        last_file_er = lf_er;

        let last_file_len = repr[..=last_file_er]
            .iter()
            .rev()
            .take_while(|mc| mc.is_some() && mc == &&operating_file_num)
            .count();

        let last_file_sr = last_file_er - (last_file_len - 1);

        if !checked_file_nums.insert(repr[last_file_sr].unwrap()) {
            last_file_er = last_file_sr;
            continue;
        }

        let mut find_empty_id = 0;
        'inner: loop {
            let Some(mut first_empty_sr) = repr[find_empty_id..]
                .iter()
                .enumerate()
                .find_map(|(i, v)| if v.is_none() { Some(i) } else { None })
            else {
                last_file_er = last_file_sr;
                continue 'outer;
            };

            first_empty_sr += find_empty_id;

            if first_empty_sr >= last_file_sr {
                last_file_er = last_file_sr;
                continue 'outer;
            }

            let first_empty_len = repr[first_empty_sr..]
                .iter()
                .take_while(|mc| mc.is_none())
                .count();

            let first_empty_er = first_empty_sr + (first_empty_len.saturating_sub(1));

            if last_file_len > first_empty_len {
                find_empty_id = first_empty_er + 1;
                continue 'inner;
            } else {
                // swap
                for i in 0..last_file_len {
                    let e_id = first_empty_sr + i;
                    let f_id = last_file_sr + i;
                    repr.swap(e_id, f_id);
                }

                last_file_er = last_file_sr;
                continue 'outer;
            }
        }
    }

    let res: usize = repr
        .iter()
        .enumerate()
        .filter_map(|(i, o)| o.map(|v| v * i))
        .sum();

    Some(res)
}

// STRUCTS
/// Each memory chunk is represented with:
///     - `Some(file_num)` if it's a file chunk
///     - `None` if it's a chunk of empty space
///
/// num_of_discrete_chunks represent the length of this array if we combine all continous repeating elements
/// into discrete chunks. ie: 1123444 -> 1234
#[derive(Debug, Clone, PartialEq, Eq)]
struct Memory(Vec<Option<usize>>);

impl Deref for Memory {
    type Target = Vec<Option<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// FUNCTIONS
fn generate_memory_representation(input: &str) -> Memory {
    let input = input.trim();
    let bytes_iter = input
        .as_bytes()
        .iter()
        .map(|&c| atoi::ascii_to_digit::<usize>(c).expect("unable to convert char to usize"));

    let len: usize = bytes_iter.clone().sum();

    let mut file_num: usize = 0;
    let mut res = Vec::with_capacity(len);
    for (i, digit) in bytes_iter.enumerate() {
        if i & 1 == 0 {
            res.extend(iter::repeat_n(Some(file_num), digit));
            file_num += 1;
        } else {
            res.extend(iter::repeat_n(None, digit));
        }
    }

    Memory(res)
}

#[allow(dead_code)]
fn debug_string_repr(repr: &[Option<usize>]) -> String {
    let mut s = String::with_capacity(repr.len());
    for val in repr.iter() {
        if let Some(val) = val {
            s.push_str(&format!("{val}"));
        } else {
            s.push('.');
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

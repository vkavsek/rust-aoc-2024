use std::{
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

pub fn part_two(input: &str) -> Option<u32> {
    let repr = generate_chunked_memory_representation(input);

    let _ = repr.iter().filter(|mc| mc.variant.is_some());

    None
}

// STRUCTS

/// Continous Memory Chunk
/// `variant`:
/// `Some(file_num)` symbolizes this is a collection of continous file chunks
/// `None` symbolizes this is a collection of continoous empty space chunks
/// `len`:
///  Represents the length of this collection of continous memory chunks
#[derive(Debug)]
struct ContMemoryChunk {
    variant: Option<usize>,
    len: usize,
}
impl ContMemoryChunk {
    fn new(variant: Option<usize>, len: usize) -> Self {
        ContMemoryChunk { variant, len }
    }
}

struct ChunkedMemory(Vec<ContMemoryChunk>);

impl From<Memory> for ChunkedMemory {
    fn from(value: Memory) -> Self {
        todo!()
    }
}
impl Deref for ChunkedMemory {
    type Target = Vec<ContMemoryChunk>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ChunkedMemory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Parsed input:
/// Each memory chunk is represented with:
///     - `Some(file_num)` if it's a file chunk
///     - `None` if it's a chunk of empty space
struct Memory(Vec<Option<usize>>);
impl Memory {
    fn rewrite_from_chunked(&mut self, chunked: ChunkedMemory) {
        todo!()
    }
}

impl From<ChunkedMemory> for Memory {
    fn from(value: ChunkedMemory) -> Self {
        todo!()
    }
}
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
fn generate_chunked_memory_representation(input: &str) -> ChunkedMemory {
    let input = input.trim();

    let bytes_iter = input
        .as_bytes()
        .iter()
        .map(|&c| atoi::ascii_to_digit::<usize>(c).expect("unable to convert char to usize"));

    let len: usize = bytes_iter.clone().count();

    let mut file_num: usize = 0;
    let mut res = Vec::with_capacity(len);
    for (i, chunk_len) in bytes_iter.enumerate() {
        if i & 1 == 0 {
            res.push(ContMemoryChunk::new(Some(file_num), chunk_len));
            file_num += 1;
        } else {
            res.push(ContMemoryChunk::new(None, chunk_len));
        }
    }

    ChunkedMemory(res)
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

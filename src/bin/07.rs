use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations = input.lines().map(Equation::from).collect_vec();

    let result: u64 = equations
        .par_iter()
        .filter_map(Equation::try_solve_p1)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = input.lines().map(Equation::from).collect_vec();

    let result: u64 = equations
        .par_iter()
        .filter_map(Equation::try_solve_p2)
        .sum();
    Some(result)
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Multi,
    Concat,
}

#[derive(Debug)]
struct Equation {
    solution: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn try_solve_p1(&self) -> Option<u64> {
        let n_of_ops = self.numbers.len() - 1;
        let mut permutations = (0..n_of_ops)
            .map(|_| [Op::Plus, Op::Multi].into_iter())
            .multi_cartesian_product();

        let is_valid =
            permutations.any(|p| {
                let r = self.numbers.iter().skip(1).enumerate().fold(
                    self.numbers[0],
                    |acc, (i, &val)| match p[i] {
                        Op::Plus => acc + val,
                        Op::Multi => acc * val,
                        Op::Concat => unreachable!(),
                    },
                );

                r == self.solution
            });

        if is_valid {
            Some(self.solution)
        } else {
            None
        }
    }

    fn try_solve_p2(&self) -> Option<u64> {
        let n_of_ops = self.numbers.len() - 1;
        let mut permutations = (0..n_of_ops)
            .map(|_| [Op::Plus, Op::Multi, Op::Concat].into_iter())
            .multi_cartesian_product();

        let is_valid =
            permutations.any(|p| {
                let r = self.numbers.iter().skip(1).enumerate().fold(
                    self.numbers[0],
                    |acc, (i, &val)| match p[i] {
                        Op::Plus => acc + val,
                        Op::Multi => acc * val,
                        Op::Concat => acc * 10u64.pow(val.ilog10() + 1) + val,
                    },
                );

                r == self.solution
            });

        if is_valid {
            Some(self.solution)
        } else {
            None
        }
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (solution, numbers) = value.split_once(':').expect("invalid input");
        let solution = str_to_u64(solution);
        let numbers = numbers.split_whitespace().map(str_to_u64).collect_vec();

        Self { solution, numbers }
    }
}

#[inline]
fn str_to_u64(input: &str) -> u64 {
    input.parse().expect("unable to parse to u64")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    solve_n_times(input, 25)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_n_times(input, 75)
}

/// track how many times each unique number appears
fn solve_n_times(input: &str, n: usize) -> Option<usize> {
    // Tracks each unique numbers amount index, handles number splitting
    let mut stones = Vec::with_capacity(5000);
    // Tracks stone numbers we are currently operating on
    let mut current_stones = Vec::with_capacity(200);
    // Tracks unique stone numbers with their index (as they appear in `amounts`).
    let mut stones_ids = HashMap::new();
    // Tracks amount of stone numbers we have already seen
    let mut amounts: Vec<usize> = Vec::with_capacity(5000);

    // initialize
    for in_num in input
        .split_whitespace()
        .map(|n| atoi::atoi::<u64>(n.as_bytes()).expect("can't convert to u64!"))
    {
        if let Some(&stone_index) = stones_ids.get(&in_num) {
            amounts[stone_index] += 1;
        } else {
            stones_ids.insert(in_num, stones_ids.len());
            current_stones.push(in_num);
            amounts.push(1);
        }
    }

    for _ in 0..n {
        let numbers = current_stones;
        current_stones = Vec::with_capacity(200);

        let mut get_index = |x: u64| {
            let len = stones_ids.len();
            *stones_ids.entry(x).or_insert_with(|| {
                // If entry wasn't present queue for processing on the next iter
                current_stones.push(x);
                len
            })
        };

        // rules for each new number
        for num in numbers {
            let (first_id, second_id) = if num == 0 {
                (get_index(1), None)
            } else {
                let num_of_digits = num.ilog10() + 1;
                let pow = 10u64.pow(num_of_digits / 2);
                if num_of_digits % 2 == 0 {
                    (get_index(num / pow), Some(get_index(num % pow)))
                } else {
                    (get_index(num * 2024), None)
                }
            };

            stones.push((first_id, second_id));
        }

        // Create vector that can contain each unique point number's amount
        let mut next_amounts = vec![0; stones_ids.len()];

        // on 1st iteration if we have 3 unique numbers, we also have 3 amounts.
        // then we split the numbers and modify the amounts
        // example: say we have a number 34 and we have seen 3 before but not 4, 34 gets split,
        // and both amounts get increased by 1. amount of 3's is now: 2. amount of 4's is now 1.
        assert_eq!(stones.len(), amounts.len());
        // Split into one or two stones
        for ((first_id, second_id), amount) in stones.iter().zip(amounts) {
            next_amounts[*first_id] += amount;
            if let Some(second_id) = second_id {
                next_amounts[*second_id] += amount;
            }
        }
        // extend the amounts
        amounts = next_amounts;
    }

    Some(amounts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}

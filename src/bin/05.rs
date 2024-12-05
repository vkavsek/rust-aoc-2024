use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(5);

type Rules = HashMap<u32, Vec<u32>>;

fn parse_rules(rules: &str) -> Rules {
    let mut r: Rules = HashMap::new();

    // Collect rules
    for rule in rules.trim().lines() {
        let (bef, aft) = rule
            .split_once('|')
            .and_then(|(b, a)| {
                // Try to convert before and after into u32
                let b = atoi::atoi::<u32>(b.as_bytes());
                b.and_then(|b| atoi::atoi::<u32>(a.as_bytes()).map(|a| (b, a)))
            })
            .expect("error parsing the rules");

        r.entry(bef)
            .and_modify(|after_vec| after_vec.push(aft))
            .or_insert(vec![aft]);
    }

    r
}

fn sort_updates(updates: &mut [u32], rules: &Rules) {
    let mut sorted = false;
    while !sorted {
        let mut swapped = false;
        for i in 1..updates.len() {
            let u = updates[i];

            let Some(u_rules) = rules.get(&u) else {
                continue;
            };

            let mut u_index = i;
            let mut slice_to_u = &updates[..u_index];
            // Swap for each rule
            for rule in u_rules {
                if let Some(pos) = slice_to_u.iter().position(|n| n == rule) {
                    swapped = true;
                    updates.swap(pos, u_index);
                    u_index = pos;
                    slice_to_u = &updates[..u_index];
                }
            }
        }

        sorted = !swapped;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rs, us) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rs);

    let mut result = 0;
    for update_line in us.trim().lines() {
        let updates = update_line
            .split(',')
            .map(|n| atoi::atoi::<u32>(n.as_bytes()).unwrap())
            .collect_vec();
        let midid = updates.len() / 2;
        assert!(updates.len() & 1 == 1);

        if updates.iter().enumerate().skip(1).all(|(i, n)| {
            let Some(after_rule) = rules.get(n) else {
                // No rule for this N - no need to check
                return true;
            };

            let slice_to_i = &updates[..i];

            // if slice contains any of the values return false
            !after_rule.iter().any(|n| slice_to_i.contains(n))
        }) {
            result += updates[midid];
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rs, us) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rs);

    let mut result = 0;
    for update_line in us.trim().lines() {
        let mut updates = update_line
            .split(',')
            .map(|n| atoi::atoi::<u32>(n.as_bytes()).unwrap())
            .collect_vec();
        assert!(updates.len() & 1 == 1);

        // Find invalid
        if updates.iter().enumerate().skip(1).any(|(i, n)| {
            let Some(after_rules) = rules.get(n) else {
                // No rule for this N - no need to check
                return false;
            };

            // if slice contains any of the values return true
            let slice_to_i = &updates[..i];
            after_rules.iter().any(|n| slice_to_i.contains(n))
        }) {
            sort_updates(&mut updates, &rules);
            let midid = updates.len() / 2;
            result += updates[midid];
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

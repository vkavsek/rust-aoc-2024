advent_of_code::solution!(8);

use std::collections::HashSet;

use advent_of_code::utils::*;

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, uniq_positions) = Grid::parse_with_unique_positions(input, b".");

    let mut antinodes = HashSet::new();
    for positions in uniq_positions.values() {
        for (i, &pos) in positions.iter().take(positions.len() - 1).enumerate() {
            let rest = &positions[(i + 1)..];
            for &other_pos in rest {
                let distance_other_pos = pos - other_pos;
                let antinode_location_a = pos + distance_other_pos;
                if grid.contains(antinode_location_a) {
                    antinodes.insert(antinode_location_a);
                }

                let distance_pos_other = other_pos - pos;
                let antinode_location_b = other_pos + distance_pos_other;
                if grid.contains(antinode_location_b) {
                    antinodes.insert(antinode_location_b);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, uniq_positions) = Grid::parse_with_unique_positions(input, b".");

    let mut antinodes = HashSet::new();
    for positions in uniq_positions.values() {
        for (i, &pos) in positions.iter().take(positions.len() - 1).enumerate() {
            let rest = &positions[(i + 1)..];
            for &other_pos in rest {
                let distance_other_pos = pos - other_pos;
                let mut antinode_location_a = pos;
                while grid.contains(antinode_location_a) {
                    antinodes.insert(antinode_location_a);
                    antinode_location_a += distance_other_pos;
                }

                let distance_pos_other = other_pos - pos;
                let mut antinode_location_b = other_pos;
                while grid.contains(antinode_location_b) {
                    antinodes.insert(antinode_location_b);
                    antinode_location_b += distance_pos_other;
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

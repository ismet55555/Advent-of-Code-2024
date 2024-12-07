//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use std::collections::vec_deque;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

type Updates = Vec<Vec<i32>>;
type Rules = Vec<(i32, i32)>;

fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}

fn load_page_rules_pairs(input: &str) -> Rules {
    let result: Rules = input
        .split_once("\n\n")
        .unwrap()
        .0 // Get first section
        .lines()
        .map(|x| {
            let parts: Vec<&str> = x.split("|").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    result
}

fn load_page_updates(input: &str) -> Updates {
    let result: Updates = input
        .split_once("\n\n")
        .unwrap()
        .1 // Get second section
        .lines()
        .map(|x| x.split(",").map(|y| y.parse().unwrap()).collect())
        .collect();

    result
}

// Return ordered and unordered update page sets
// Update sets are verified by rules provided
fn filter_updates(updates: Updates, rules: &Rules) -> (Updates, Updates) {
    let mut updates_ordered: Updates = vec![];
    let mut updates_unordered: Updates = vec![];

    for update in updates {
        let mut update_numbers_verified = vec![false; update.len()];

        for (index, updated_page) in update.iter().enumerate() {
            let mut rules_verified_for_page = true;

            for (rule_1, rule_2) in rules {
                // Ignore rules not involving the updated page
                if *updated_page != *rule_1 && *updated_page != *rule_2 {
                    continue;
                }

                // Ignore rules where the other number is not within the updated list
                if !update.contains(rule_1) || !update.contains(rule_2) {
                    continue;
                }

                // Look to the right and the second rule
                if *updated_page == *rule_1 {
                    let numbers_to_right = &update[index + 1..];
                    if !numbers_to_right.contains(rule_2) {
                        rules_verified_for_page = false;
                        break;
                    }
                }

                // Look to the left and the first rule
                if *updated_page == *rule_2 {
                    let numbers_to_left = &update[..index];
                    if !numbers_to_left.contains(rule_1) {
                        rules_verified_for_page = false;
                        break;
                    }
                }
            }

            if rules_verified_for_page {
                update_numbers_verified[index] = true;
            }
        }

        if update_numbers_verified.iter().all(|&x| x) {
            updates_ordered.push(update);
        } else {
            updates_unordered.push(update);
        }
    }
    (updates_ordered, updates_unordered)
}

fn order_updates(mut updates: Updates, rules: &Rules) -> Updates {
    for update in &mut updates {
        let mut changed = true;
        while changed {
            changed = false;
            for index in 0..update.len() - 1 {
                for &(rule_1, rule_2) in rules {
                    if update[index] == rule_2 && update[index + 1] == rule_1 {
                        update.swap(index, index + 1);
                        changed = true;
                    }
                }
            }
        }
    }
    updates
}

fn sum_update_middle_numbers(updates: Updates) -> i32 {
    let mut result: i32 = 0;
    for update in updates {
        let middle_number = update[update.len() / 2];
        result += update[update.len() / 2];
    }

    result
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let rules = load_page_rules_pairs(&input);
    let updates = load_page_updates(&input);
    let (updates_ordered, _) = filter_updates(updates, &rules);

    sum_update_middle_numbers(updates_ordered)
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    let rules = load_page_rules_pairs(&input);
    let updates = load_page_updates(&input);
    let (_, updates_unordered) = filter_updates(updates, &rules);
    let updates_ordered = order_updates(updates_unordered, &rules);

    sum_update_middle_numbers(updates_ordered)
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
    let part_1_input = load_text_file("inputs/input_part1.txt");
    let part_1_output = part_1(part_1_input);
    println!("Part 1: {}\n\n", part_1_output);

    let part_2_input = load_text_file("inputs/input_part2.txt");
    let part_2_output = part_2(part_2_input);
    println!("Part 2: {}", part_2_output);

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////
//                            TESTS
//////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = load_text_file("inputs/sample_part1.txt");
        let expected = 143;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 123;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}

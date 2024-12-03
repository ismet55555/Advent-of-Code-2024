//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

// Read the contents of a line into a String
// Will still include new lines
fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");
    contents
}

fn parse_to_number(input: &str) -> Vec<i32> {
    let out: Vec<i32> = input
        .split_whitespace()
        .map(|n| n.trim().parse().unwrap()) // Parse to a number
        .collect();
    out
}

fn are_levels_safe(input: Vec<i32>) -> bool {
    if input.is_empty() {
        return false;
    }

    // Get diffs between numbers
    let diffs: Vec<i32> = input.windows(2).map(|x| x[1] - x[0]).collect();

    // Check no sign change - all positive or all negative
    if !(diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0)) {
        return false;
    }

    // Check level changes are >1 and <4
    let within_level_change = diffs
        .iter()
        .map(|diff| diff.abs())
        .all(|diff_abs| diff_abs > 0 && diff_abs < 4);
    if !within_level_change {
        return false;
    }

    true
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let mut safe_count: i32 = 0;

    for (index, line) in input.lines().enumerate() {
        let level_numbers = parse_to_number(line);
        if !are_levels_safe(level_numbers) {
            continue;
        }

        safe_count += 1;
    }

    safe_count
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    let mut safe_count: i32 = 0;

    for (index, line) in input.lines().enumerate() {
        let level_numbers = parse_to_number(line);

        for index in 0..level_numbers.len() {
            let mut new_level_numbers = level_numbers.clone();
            new_level_numbers.remove(index);
            if !are_levels_safe(new_level_numbers) {
                continue;
            } else {
                safe_count += 1;
                break;
            }
        }
    }

    safe_count
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
    let part_1_input = load_text_file("inputs/input_part1.txt");
    let part_1_output = part_1(part_1_input);
    println!("Part 1: {}\n\n", part_1_output);

    let part_2_input = load_text_file("inputs/input_part2.txt");
    let part_2_output = part_2(part_2_input);
    println!("Part 1: {}", part_2_output);

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
        let expected = 2;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 4;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}

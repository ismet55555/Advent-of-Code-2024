//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");
    contents
}

fn execute_multipliplication(input: &str) -> i32 {
    let regex_pattern = r"mul\((\d+),\s*(\d+)\)";
    let re = Regex::new(regex_pattern).unwrap();

    if let Some(cap) = re.captures(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        x * y
    } else {
        0
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let regex_pattern = r"mul\((\d{1,3}),\s*(\d{1,3})\)";
    let re = Regex::new(regex_pattern).unwrap();

    let mut result: i32 = 0;
    for cap in re.captures_iter(&input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        result += x * y;
    }
    result
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    // Collect all instructions
    let regex_pattern = r"(?:mul\(\d+,\d+\)|don't\(\)|do\(\))";
    let re = Regex::new(regex_pattern).unwrap();
    let instructions: Vec<_> = re
        .find_iter(&input)
        .map(|m| m.as_str().to_string())
        .collect();

    let mut result: i32 = 0;
    let mut enable_instructions = true;
    for (index, instruction) in instructions.iter().enumerate() {
        if instruction.contains("do()") {
            enable_instructions = true;
        } else if instruction.contains("don't()") {
            enable_instructions = false;
        }

        if enable_instructions {
            result += execute_multipliplication(instruction);
            continue;
        }
    }

    result
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
        let expected = 161;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 48;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}

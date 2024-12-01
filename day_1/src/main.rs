//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use itertools::{sorted, Itertools};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

//////////////////////////////////////////////////////////////////////////////

fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");
    contents
}

fn parse_sides(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (left_col, right_col): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split("  ")
                .map(|n| n.trim().parse().unwrap()) // Parse to a number
                .collect();
            (nums[0], nums[1])
        })
        // .inspect(|x| println!("Value: {:?}", x))
        .unzip();

    (left_col, right_col)
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let (left_col, right_col) = parse_sides(&input);

    // Sort
    let left: Vec<i32> = left_col.into_iter().sorted().collect();
    let right: Vec<i32> = right_col.into_iter().sorted().collect();

    // Calculate the difference between each and sum
    let result: i32 = zip(left, right).map(|(l, r)| (l - r).abs()).sum();

    result
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    let (mut left_col, right_col) = parse_sides(&input);

    // Find multiplier / count
    let mut left_similarity_vec: Vec<i32> = vec![];
    for left_number in left_col.iter_mut() {
        let mulitplier: i32 = right_col
            .iter()
            .filter(|x| *x == left_number)
            // .inspect(|x| println!("Filter: {:?}", x))
            .count()
            .try_into()
            .unwrap();
        left_similarity_vec.push(*left_number * mulitplier);
    }
    let result: i32 = left_similarity_vec.iter().sum();

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
        let expected = 11;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 31;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}

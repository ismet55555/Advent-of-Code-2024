//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use itertools::{Combinations, Itertools};
use nom::number;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}

fn parse_input(input: String) -> (Vec<i64>, Vec<Vec<i64>>) {
    let (test_results, numbers_sets): (Vec<i64>, Vec<Vec<i64>>) = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(": ").collect();
            let item_1: i64 = parts[0].parse().unwrap();
            let item_2: Vec<i64> = parts[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (item_1, item_2)
        })
        .collect();

    (test_results, numbers_sets)
}

fn generate_operator_combinations(number_set: Vec<u8>, length: usize) -> Vec<Vec<u8>> {
    (0..length)
        .map(|_| number_set.clone())
        .multi_cartesian_product() // combinations_with_replacement() does not work
        .collect()
}

fn can_operators_work(test_result: &i64, number_set: &[i64]) -> bool {
    // println!("\n------------------------------------------------");
    // println!("{}  -  {:?}", test_result, number_set);

    // Generate combinations for plus (0) and multiply (1)
    let combinations = generate_operator_combinations(vec![0, 1], number_set.len() - 1);

    // Try out all combinations
    for combination in combinations {
        // println!("---- {:?} ----", combination);
        let mut total: i64 = number_set[0];

        for (index, number) in number_set.iter().skip(1).enumerate() {
            if combination[index] == 0 {
                // Addition operator
                total += number;
            } else {
                // Multiplication operator
                total *= number;
            }
            // println!("Total: {}", total);

            // If it is already over the total_result, combination does not work
            if total > *test_result {
                // println!(" -> BUST");
                continue; // Next operator combination
            }

            if total == *test_result {
                // println!(" -> ************ GOOD ******************\n");
                return true;
            }
        }
    }

    false
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i64 {
    let (test_results, number_sets) = parse_input(input);

    // Only get test results that work
    let valid_test_results: Vec<i64> = test_results
        .into_iter()
        .zip(number_sets)
        .filter(|(test_result, number_set)| can_operators_work(test_result, number_set))
        .map(|(test_result, _)| test_result)
        .collect();
    // println!("Valid Test Results: {:?}", valid_test_results);

    valid_test_results.iter().sum()
}

/// Part 2 Calculations
fn part_2(input: String) -> i64 {
    println!("{}", input);
    22
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
    let part_1_input = load_text_file("inputs/input_part1.txt");
    let part_1_output = part_1(part_1_input);
    println!("Part 1: {}\n\n", part_1_output);

    // let part_2_input = load_text_file("inputs/sample_part2.txt");
    // let part_2_output = part_2(part_2_input);
    // println!("Part 2: {}", part_2_output);

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
        let expected = 3749;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = load_text_file("inputs/sample_part2.txt");
    //     let expected = 22;
    //     let actual = part_2(input);
    //     assert_eq!(expected, actual);
    // }
}

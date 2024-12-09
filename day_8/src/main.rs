//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

type Grid = Vec<Vec<char>>;

fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}

fn get_grid_dimensions(grid: &Grid) -> (i32, i32) {
    let rows_total: i32 = grid[0].len().try_into().unwrap();
    let columns_total: i32 = grid.len().try_into().unwrap();
    (rows_total, columns_total)
}

fn is_antena(grid: &Grid, row_pos: usize, col_pos: usize) -> Option<char> {
    if grid[row_pos][col_pos] != '.' {
        return Some(grid[row_pos][col_pos]);
    }
    None
}

// (row position, column position)
type Position = (usize, usize);

// Directions: |=up-down, -=left-right, /=diagonal-up, \=diagonal-down
static SEARCH_DIRECTIONS: [(char, (i32, i32)); 8] = [
    ('|', (1, 0)),
    ('|', (-1, 0)),
    ('-', (0, 1)),
    ('-', (0, -1)),
    ('/', (1, 1)),
    ('/', (-1, -1)),
    ('\\', (-1, 1)),
    ('\\', (1, -1)),
];

// Look around distance of antena symbol to find another matching antena symbol
// Return position relative to antena and the direction character
fn look_around_antena(
    grid: &Grid,
    antena_symbol: char,
    pos: Position,
    rel_distance: i32,
) -> Vec<((i32, i32), char)> {
    let out: Vec<((i32, i32), char)> = vec![((-2, 1), '|')]; // example input, do i need position?

    // Go around "distance" from "pos" in all 8 directions
    // Check match on symbol
    // If symbol matches, store it in "out"

    for search_direction in SEARCH_DIRECTIONS.iter() {}

    // Back at caller, use the rel distance and direction to draw antinodes

    out
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    println!("{}", input);

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    // loop around

    11
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    println!("{}", input);
    22
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
    let part_1_input = load_text_file("inputs/sample_part1.txt");
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
        let expected = 14;
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

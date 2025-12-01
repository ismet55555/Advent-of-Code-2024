//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use std::collections::HashSet;
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
) -> Vec<(char, char, i32)> {
    // Return: antena symbol, direction, distance
    let mut out: Vec<(char, char, i32)> = vec![]; // example input, do i need position?

    let (grid_rows_total, grid_col_total) = get_grid_dimensions(grid);

    // Go around "distance" from "pos" in all 8 directions
    // Check match on symbol
    // If symbol matches, store it in "out"

    println!("Antena '{}' Position: ({},{})", antena_symbol, pos.0, pos.1);

    let row_pos = pos.0 as i32;
    let col_pos = pos.1 as i32;

    for (search_direction, (row_step, col_step)) in SEARCH_DIRECTIONS.iter() {
        let search_row_pos = row_pos + *row_step;
        let search_col_pos = col_pos + *col_step;
        print!(
            " - Step: ({},{})  '{}'  -> ({},{})",
            row_step, col_pos, search_direction, search_row_pos, search_col_pos
        );

        // If out of bounds, skip
        if search_row_pos + 1 > grid_rows_total
            || search_row_pos < 0
            || search_col_pos + 1 > grid_col_total
            || search_col_pos < 0
        {
            println!(" -> Out of Bounds");
            continue;
        }

        // Get symbol at search location
        let grid_symbol = grid[search_row_pos as usize][search_col_pos as usize];
        println!(" -> {}", grid_symbol);

        // Skip anything that is not antena
        if grid_symbol != antena_symbol {
            continue;
        }
        println!("Found antena {} - distance {}", antena_symbol, rel_distance);
        out.push((antena_symbol, *search_direction, rel_distance));
    }

    out
}

fn find_distinct_antena_symbols(grid: &Grid) -> HashSet<char> {
    let mut antena_symbols: HashSet<char> = HashSet::new();

    for row_pos in 0..grid.len() {
        for col_pos in 0..grid[0].len() {
            if grid[row_pos][col_pos] != '.' {
                antena_symbols.insert(grid[row_pos][col_pos]);
            }
        }
    }

    antena_symbols
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    // println!("{}", input);

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let grid_new = grid.clone();

    let unique_antena_symbols = find_distinct_antena_symbols(&grid);
    println!("{:?}", unique_antena_symbols);

    // look_around_antena(&grid, '0', (2_usize, 5_usize), 1);

    for antena_symbol in unique_antena_symbols {
        println!(
            "---------------  Antena: {}  ----------------",
            antena_symbol
        );
        for row_pos in 0..grid.len() {
            for col_pos in 0..grid[0].len() {
                if grid[row_pos][col_pos] != '.' {
                    let antinode_findings =
                        look_around_antena(&grid, antena_symbol, (row_pos, col_pos), 1);
                }
            }
        }
    }

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

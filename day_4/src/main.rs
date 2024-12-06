//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;
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

/// Looks at each direction of the given row/col position for the complete word
/// Return how many word matches are in the surrounding of this position
///
/// change in x -> change in rows
/// change in y -> change in column
fn count_words_in_directions(grid: &Grid, word: &str, i_row: &usize, i_col: &usize) -> i32 {
    let (rows_total, columns_total) = get_grid_dimensions(grid);

    // Row change:    + down
    // Column change: + right
    let grid_directions: Vec<(i32, i32, &str)> = vec![
        (1, 0, "down"), // Change rows only
        (-1, 0, "up"),
        (0, 1, "right"), // Change in columns only
        (0, -1, "left"),
        (1, 1, "down-right"),
        (-1, -1, "up-left"),
        (-1, 1, "up-right"),
        (1, -1, "down-left"),
    ];

    // Skip chars that are not word beginning (ie. QSDF does not start with X)
    if grid[*i_row][*i_col] != word.chars().next().unwrap() {
        return 0;
    }

    let mut match_count: i32 = 0;

    // Loop directions
    for (row_step, col_step, direction) in grid_directions {
        // Copy and reverse the string (ie. XMAS -> SAMX)
        let word_no_begin_letter: String = word[1..].to_string();
        let mut word_rev: String = word_no_begin_letter.chars().rev().collect();

        // Redefine the search start row/col
        let mut i_row_search = *i_row as i32;
        let mut i_col_search = *i_col as i32;

        // Loop through each letter in word
        for letter in word[1..].chars() {
            i_row_search += row_step;
            i_col_search += col_step;

            if i_row_search >= 0
                && i_row_search < rows_total
                && i_col_search >= 0
                && i_col_search < columns_total
            {
                let next_letter = grid[i_row_search as usize][i_col_search as usize];

                if word_rev.ends_with(next_letter) {
                    word_rev.pop();
                } else {
                    // Letter is not the right one
                    break;
                }

                if word_rev.is_empty() {
                    match_count += 1;
                }
            } else {
                // Search out of bounds!
                break;
            }
        }
    }
    match_count
}

/// Looks at each direction of the given row/col position for the word
/// in an X chape. Return how many instances matches found
///
/// Assume word is 3 characters long
///
/// Example: MAS (any direction)
///
/// M.S
/// .A.
/// M.S
///
/// change in x -> change in rows
/// change in y -> change in column
fn count_words_x_shaped(grid: &Grid, word: &str, i_row: &usize, i_col: &usize) -> i32 {
    let (rows_total, columns_total) = get_grid_dimensions(grid);

    // Row change:    + down
    // Column change: + right
    // (row start, col start, position, row direction, col direction)
    let start_positions: Vec<(i32, i32, &str, i32, i32)> = vec![
        (1, 1, "bottom-right -> top-left", -1, -1),
        (-1, -1, "top-left -> bottom-right", 1, 1),
        (-1, 1, "top-right -> bottom-left", 1, -1),
        (1, -1, "bottom-left -> top-right", -1, 1),
    ];

    // Skip chars that are not the middle out of 3 (ie. A in MAS)
    if grid[*i_row][*i_col] != word.chars().nth(1).unwrap() {
        return 0;
    }

    let mut match_count: i32 = 0;

    // Loop directions
    for (row_start, col_start, position, row_step, col_step) in start_positions {
        let word_string: String = word.to_string();

        // Redefine the search start row/col
        let mut i_row_search = *i_row as i32 + row_start;
        let mut i_col_search = *i_col as i32 + col_start;

        let mut found_letters = String::new();

        // Loop through each letter in word
        for letter in word.chars() {
            if i_row_search >= 0
                && i_row_search < rows_total
                && i_col_search >= 0
                && i_col_search < columns_total
            {
                let next_letter = grid[i_row_search as usize][i_col_search as usize];

                found_letters.push(next_letter);
                if found_letters == word {
                    match_count += 1;
                }

                // Can only have two matches (cross twice)
                if match_count == 2 {
                    return 1;
                }
            } else {
                // Search out of bounds!
                break;
            }

            i_row_search += row_step;
            i_col_search += col_step;
        }
    }
    0
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let word: &str = "XMAS";
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let mut total_items_found: i32 = 0;
    for i_row in 0..grid.len() {
        for i_col in 0..grid[0].len() {
            total_items_found += count_words_in_directions(&grid, word, &i_row, &i_col);
        }
    }

    total_items_found
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    let x_word: &str = "MAS";
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let mut total_items_found: i32 = 0;
    for i_row in 0..grid.len() {
        for i_col in 0..grid[0].len() {
            // Skip chars that are not the middle out of 3 (ie. A in MAS)
            if grid[i_row][i_col] != x_word.chars().nth(1).unwrap() {
                continue;
            }

            let found = count_words_x_shaped(&grid, x_word, &i_row, &i_col);
            total_items_found += found;
        }
    }

    total_items_found
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
        let expected = 18;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 9;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}

//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use std::collections::{HashMap, HashSet};
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

fn find_guard_position_and_direction(grid: &Grid) -> Option<((usize, usize), &str)> {
    let guard_face_directions = vec![('v', "down"), ('<', "left"), ('^', "up"), ('>', "right")];

    for (row_index, row) in grid.iter().enumerate() {
        for (symbol, direction_text) in &guard_face_directions {
            if let Some(col_index) = row.iter().position(|&c| c == *symbol) {
                return Some(((row_index, col_index), direction_text));
            }
        }
    }
    None
}

// Check if guard will collide if he/she moves into this position
fn is_obstacle(grid: &Grid, row_pos: usize, col_pos: usize) -> bool {
    if grid[row_pos][col_pos] == '#' {
        return true;
    }
    false
}

/// Moves guard and records number of uniquely visited position
fn move_guard_until_out_of_bounds(
    grid: &Grid,
    row_start: usize,
    col_start: usize,
    direction_start: &str,
) -> i32 {
    let (grid_rows_total, grid_col_total) = get_grid_dimensions(grid);

    // Row change:    + down
    // Column change: + right
    let directions = vec!["down", "left", "up", "right"];
    let move_steps: HashMap<&str, (i32, i32)> = HashMap::from([
        ("down", (1, 0)),
        ("left", (0, -1)),
        ("up", (-1, 0)),
        ("right", (0, 1)),
    ]);

    // Creating a diretion cycler and setting it to starting direction
    let direction_start_index = directions
        .iter()
        .position(|x| *x == direction_start)
        .unwrap();
    let mut direction_cycler = directions.into_iter().cycle().skip(direction_start_index);

    // Initial values
    let mut current_row_pos = row_start as i32;
    let mut curent_col_pos = col_start as i32;
    let mut current_direction = direction_cycler.next().unwrap();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    // Move loop
    loop {
        visited_positions.insert((current_row_pos, curent_col_pos));

        let (row_step, col_step) = move_steps[current_direction];
        current_row_pos += row_step;
        curent_col_pos += col_step;

        // Check position for out of bounds
        if current_row_pos + 1 > grid_rows_total || curent_col_pos + 1 > grid_col_total {
            break;
        }

        // Check position collision
        if is_obstacle(grid, current_row_pos as usize, curent_col_pos as usize) {
            current_row_pos -= row_step;
            curent_col_pos -= col_step;
            current_direction = direction_cycler.next().unwrap();
        }
    }

    visited_positions.len() as i32
}

/// Run a simulation to see if guard is stuck walking in a loop
fn is_guard_movement_a_loop(
    grid: &Grid,
    row_start: usize,
    col_start: usize,
    direction_start: &str,
) -> bool {
    let (grid_rows_total, grid_col_total) = get_grid_dimensions(grid);

    // Row change:    + down
    // Column change: + right
    let directions = vec!["down", "left", "up", "right"];
    let move_steps: HashMap<&str, (i32, i32)> = HashMap::from([
        ("down", (1, 0)),
        ("left", (0, -1)),
        ("up", (-1, 0)),
        ("right", (0, 1)),
    ]);

    // Creating a diretion cycler and setting it to starting direction
    let direction_start_index = directions
        .iter()
        .position(|x| *x == direction_start)
        .unwrap();
    let mut direction_cycler = directions.into_iter().cycle().skip(direction_start_index);

    // Initial values
    let mut current_row_pos = row_start as i32;
    let mut current_col_pos = col_start as i32;
    let mut current_direction = direction_cycler.next().unwrap();

    let mut visited_start_pos_count: i32 = 0;
    let mut loop_count_total: i32 = 0;

    // Move loop
    loop {
        // Checking position
        let (row_step, col_step) = move_steps[current_direction];
        current_row_pos += row_step;
        current_col_pos += col_step;

        // Check out of bounds
        if current_row_pos + 1 > grid_rows_total
            || current_row_pos < 0
            || current_col_pos + 1 > grid_col_total
            || current_col_pos < 0
        {
            return false;
        }

        // Check collision
        if is_obstacle(grid, current_row_pos as usize, current_col_pos as usize) {
            current_row_pos -= row_step;
            current_col_pos -= col_step;

            // Go to next direction
            current_direction = direction_cycler.next().unwrap();
        }

        // Check if initial position is visited
        if current_row_pos == row_start as i32 && current_col_pos == col_start as i32 {
            visited_start_pos_count += 1;
        }

        // Check if visited more than some threshold
        if visited_start_pos_count > 10 {
            return true;
        }

        loop_count_total += 1;
        if loop_count_total > 50_000 {
            return true;
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let ((guard_row_pos, guard_col_pos), guard_direction) =
        find_guard_position_and_direction(&grid).unwrap();

    move_guard_until_out_of_bounds(&grid, guard_row_pos, guard_col_pos, guard_direction)
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let ((guard_row_pos, guard_col_pos), guard_direction) =
        find_guard_position_and_direction(&grid).unwrap();

    let mut guard_loop_count: i32 = 0;

    // Loop each open map position and place an additional "#" on map
    // and run guard movement simulation to check for a loop
    let mut grid_2 = grid.clone();

    for row_pos in 0..grid.len() {
        for col_pos in 0..grid[0].len() {
            if !is_obstacle(&grid, row_pos, col_pos)
                && !(guard_row_pos == row_pos && guard_col_pos == col_pos)
            {
                // Place obstacle
                grid_2[row_pos][col_pos] = '#';

                // Run simulation
                if is_guard_movement_a_loop(&grid_2, guard_row_pos, guard_col_pos, guard_direction)
                {
                    guard_loop_count += 1;
                }

                // Reset grid for next iteration
                grid_2[row_pos][col_pos] = '.';
            }
        }
    }

    guard_loop_count
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
        let expected = 41;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 6;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}

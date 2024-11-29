use std::fs;

const INPUT_FILEPATH: &str = "inputs/sample_part1.txt";

fn main() {
    println!("Loading file: {} ...", INPUT_FILEPATH);
    let contents = fs::read_to_string(INPUT_FILEPATH).expect("ERROR: Failed to read input file");

    // Divide the lines in file into a vector of strings
    let input_lines: Vec<&str> = contents.split("\n").collect();

    // Print the first line of the input file
    for line in &input_lines {
        println!("{}", line);
    }
}

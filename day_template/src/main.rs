use std::fs;

//////////////////////////////////////////////////////////////////////////////

const INPUT_FILEPATH: &str = "inputs/sample_part1.txt";

fn load_input_file() -> Vec<&str> {
    println!("Loading file: {} ...", INPUT_FILEPATH);
    let contents = fs::read_to_string(INPUT_FILEPATH).expect("ERROR: Failed to read input file");

    // Divide the lines in file into a vector of strings
    let input_lines: Vec<&str> = contents.split("\n").collect()
    input_lines

    // // Print the first line of the input file
    // for line in &input_lines {
    //     println!("{}", line);
    // }
}

fn part_1() {
    todo!("part_1")
}

fn part_2() {
    todo!("part_2")
}

//////////////////////////////////////////////////////////////////////////////

fn main() {
    let part_1_input = load_input_file();
    let part_1 = part_1();

    let part_2_input = load_input_file();
    let part_2 = part_2();
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // let expected =
        // let acutal =
        // assert_eq!(expected, acutal);
    }
}

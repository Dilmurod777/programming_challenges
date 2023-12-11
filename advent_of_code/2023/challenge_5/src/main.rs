use std::fs;


fn main() {
    let input_filename = "input1.txt";
    let output_filename = "output.txt";

    let _ = fs::remove_file(output_filename);

    let contents = fs::read_to_string(input_filename)
        .expect("Cannot read file. Please, check the path!");

    let lines: Vec<&str> = contents
        .split("\n")
        .map(|x| x.trim())
        .collect();

    for i in 0..lines.len() {
        let line = lines[i];
    }
}
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let input_filename = "input.txt";
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

        let mut output_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_filename)
            .expect("Cannot open file. Please, check the path!");

        output_file
            .write_fmt(format_args!("{}", line))
            .expect("Write to output file failed. Try again!");
    }
}
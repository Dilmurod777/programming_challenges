use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let input_filename = "input2.txt";
    let output_filename = "output.txt";

    let _ = fs::remove_file(output_filename);

    let contents = fs::read_to_string(input_filename)
        .expect("Cannot read file. Please, check the path!");

    let lines: Vec<&str> = contents
        .split("\n")
        .map(|x| x.trim())
        .collect();
    let mut total: f32 = 0.0;
    let mut avg: f32;
    let mut exchange_amount: f32 = 0.0;

    for i in 0..lines.len() {
        let line = lines[i];

        if !line.contains(".") {
            let number_of_students = line.parse::<usize>().expect("Could not parse the integer. Try again!");

            if number_of_students == 0 {
                break;
            }

            for j in 0..number_of_students {
                total += lines[i + j + 1].parse::<f32>().unwrap();
            }

            avg = total / (number_of_students as f32);

            for j in 0..number_of_students {
                let expense = lines[i + j + 1].parse::<f32>().unwrap();
                let diff = avg - expense;
                let diff = f32::trunc(diff  * 100.0) / 100.0;

                if diff < 0.0 {
                    exchange_amount += -1.0 * diff;
                }
            }

            let mut output_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_filename)
                .expect("Cannot open file. Please, check the path!");

            output_file
                .write_fmt(format_args!("${exchange_amount:.2}\n"))
                .expect("Write to output file failed. Try again!");

            total = 0.0;
            exchange_amount = 0.0;
        }
    }
}
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn check_field_indexes(i: usize, j: usize, n: usize, m: usize) -> bool {
    return i < n && j < m;
}

fn main() {
    let input_filename = "input.txt";
    let output_filename = "output.txt";

    let _ = fs::remove_file(output_filename);

    let contents = fs::read_to_string(input_filename)
        .expect("Cannot read file. Please, check the path!");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut rows: usize;
    let mut cols: usize;

    let mut field: Vec<Vec<i32>>;
    let mut starting_index: usize;
    let mut ending_index: usize;

    let mut field_index = 0;

    for i in 0..lines.len() {
        let line = lines[i].trim().replace("\r", "");
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() == 2 {
            rows = parts[0].parse().expect("Could not parse. Try again!");
            cols = parts[1].parse().expect("Could not parse. Try again!");

            if rows == 0 && cols == 0 {
                break;
            }

            field = vec![vec![0; cols]; rows];
            starting_index = i + 1;
            ending_index = i + rows + 1;

            for j in 0..(ending_index - starting_index) {
                let row: Vec<&str> = lines[starting_index + j].trim().split("").filter(|x| x.len() != 0).collect();

                for k in 0..row.len() {
                    if row[k] == "*" {
                        if check_field_indexes(j.wrapping_sub(1), k, rows, cols) {
                            field[j.wrapping_sub(1)][k] += 1;
                        }

                        if check_field_indexes(j.wrapping_add(1), k, rows, cols) {
                            field[j.wrapping_add(1)][k] += 1;
                        }

                        if check_field_indexes(j, k.wrapping_sub(1), rows, cols) {
                            field[j][k.wrapping_sub(1)] += 1;
                        }

                        if check_field_indexes(j, k.wrapping_add(1), rows, cols) {
                            field[j][k.wrapping_add(1)] += 1;
                        }

                        if check_field_indexes(j.wrapping_sub(1), k.wrapping_sub(1), rows, cols) {
                            field[j.wrapping_sub(1)][k.wrapping_sub(1)] += 1;
                        }

                        if check_field_indexes(j.wrapping_sub(1), k.wrapping_add(1), rows, cols) {
                            field[j.wrapping_sub(1)][k.wrapping_add(1)] += 1;
                        }

                        if check_field_indexes(j.wrapping_add(1), k.wrapping_sub(1), rows, cols) {
                            field[j.wrapping_add(1)][k.wrapping_sub(1)] += 1;
                        }

                        if check_field_indexes(j.wrapping_add(1), k.wrapping_add(1), rows, cols) {
                            field[j.wrapping_add(1)][k.wrapping_add(1)] += 1;
                        }

                        field[j][k] = -1;
                    }
                }
            }

            field_index += 1;

            let mut output_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_filename)
                .expect("Cannot open file. Please, check the path!");

            output_file
                .write_fmt(format_args!("Field #{field_index}:\n"))
                .expect("Write to output file failed. Try again!");

            for j in 0..rows {
                let row = &field[j];
                let mut output_row = String::new();

                for k in 0..cols {
                    if row[k] == -1 {
                        output_row += "*";
                    } else {
                        output_row += row[k].to_string().as_str();
                    }
                }

                output_file
                    .write_fmt(format_args!("{output_row}:\n"))
                    .expect("Write to output file failed. Try again!");
            }
        }
    }
}
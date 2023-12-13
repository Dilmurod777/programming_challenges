use std::fs;

fn parse_line(line: &str) -> i64 {
    line.trim().replace(" ", "").parse::<i64>().unwrap()
}

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


    let mut time: i64 = 0;
    let mut distance: i64 = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split(":").collect();

        if line.starts_with("Time") {
            time = parse_line(parts[1]);
        }

        if line.starts_with("Distance") {
            distance = parse_line(parts[1]);
        }
    }

    let mut min_time :i64 = 0;

    for j in 0..time {
        let min_temp = j * (time - j);

        if min_temp > distance {
            min_time = j;
            break;
        }
    }

    println!("{}", time - min_time * 2 + 1);
}

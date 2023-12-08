use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn get_cycle_length(n: i32) -> i32 {
    let mut temp = n;
    let mut length = 1;

    while temp != 1 {
        if temp % 2 == 0 {
            temp /= 2;
        } else {
            temp = temp * 3 + 1;
        }

        length += 1;
    }

    return length;
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

    for line in lines {
        let bounds: Vec<&str> = line.trim().split(" ").collect();
        let i = bounds[0].parse::<i32>().expect("Error while parsing!");
        let j = bounds[1].parse::<i32>().expect("Error while parsing!");
        let mut max_cycle_length: i32 = -1;

        for k in i..j + 1 {
            let cycle_length = get_cycle_length(k);

            if cycle_length > max_cycle_length {
                max_cycle_length = cycle_length;
            }
        }

        let mut output_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_filename)
            .expect("Cannot open file. Please, check the path!");

        output_file
            .write_fmt(format_args!("{i} {j} {max_cycle_length}\n"))
            .expect("Write to output file failed. Try again!");
    }
}

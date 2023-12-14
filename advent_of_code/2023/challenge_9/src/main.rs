use std::fs;

fn is_all_zero(sequence: &Vec<i64>) -> bool {
    for v in sequence {
        if v != &0 {
            return false;
        }
    }

    return true;
}

fn get_differences(sequence: &Vec<i64>) -> Vec<i64> {
    let mut diff: Vec<i64> = Vec::new();

    for i in 0..sequence.len() - 1 {
        diff.push(sequence[i + 1] - sequence[i]);
    }

    return diff;
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

    let mut total = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        let mut sequence: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        let mut last_values: Vec<i64> = vec![sequence[sequence.len() - 1]];
        let mut first_values: Vec<i64> = vec![sequence[0]];

        loop {
            sequence = get_differences(&sequence);
            last_values.push(sequence[sequence.len() - 1]);
            first_values.push(sequence[0]);

            if is_all_zero(&sequence) {
                break;
            }
        }

        while first_values.len() != 1 {
            let a = first_values.pop().unwrap();
            let b = first_values.pop().unwrap();

            first_values.push(b - a);
        }

        let next_value = last_values.iter().sum::<i64>();
        let prev_value = first_values[0];

        println!("{} | {:?} | {}", prev_value, line, next_value);
        total += prev_value;
    }

    println!("{}", total);
}

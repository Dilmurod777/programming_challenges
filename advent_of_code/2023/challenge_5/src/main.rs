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

    let mut values: Vec<i64> = Vec::new();
    let mut ranges: Vec<Vec<i64>> = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i];

        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds") {
            let parts: Vec<&str> = line.split(":").collect();
            values = parts[1].trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            continue;
        }

        if !line.contains("map:") {
            let parts: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            ranges.push(parts);
        } else {
            for j in 0..values.len() {
                for range in &ranges {
                    let source = range[1];
                    let destination = range[0];
                    let count = range[2];

                    if values[j] >= source && values[j] < source + count {
                        values[j] = destination + values[j] - source;
                        break;
                    }
                }
            }

            ranges.clear();
        }
    }

    for j in 0..values.len() {
        for range in &ranges {
            let source = range[1];
            let destination = range[0];
            let count = range[2];

            if values[j] >= source && values[j] < source + count {
                values[j] = destination + values[j] - source;
                break;
            }
        }
    }

    ranges.clear();

    match values.iter().min() {
        None => {}
        Some(x) => {
            println!("min: {}", x);
        }
    }
}
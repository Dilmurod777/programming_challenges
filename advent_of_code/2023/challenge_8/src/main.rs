use std::collections::HashMap;
use std::fs;

fn main() {
    let input_filename = "input3.txt";
    let output_filename = "output.txt";

    let _ = fs::remove_file(output_filename);

    let contents = fs::read_to_string(input_filename)
        .expect("Cannot read file. Please, check the path!");

    let lines: Vec<&str> = contents
        .split("\n")
        .map(|x| x.trim())
        .collect();

    let mut routes: HashMap<String, Vec<String>> = HashMap::new();
    let directions: Vec<i32> = lines[0].chars().map(|x| {
        return match x {
            'L' => 0,
            'R' => 1,
            _ => 0
        };
    }).collect();

    for i in 2..lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split("=").collect();
        let location = parts[0].trim();
        let next_locations: Vec<String> = parts[1]
            .split(",")
            .map(|x| x
                .trim()
                .replace("(", "")
                .replace(")", ""))
            .collect();

        routes.entry(location.to_string()).or_insert(next_locations);
    }

    let mut current_location = "AAA";
    let final_location = "ZZZ";
    let mut steps = 0;
    let mut current_direction_index = 0;

    while current_location != final_location {
        let current_direction = directions[current_direction_index % directions.len()] as usize;
        let next_locations = routes.get(current_location).unwrap();
        current_location = next_locations[current_direction].as_str();
        steps += 1;
        current_direction_index += 1;
    }

    println!("{:?}", steps);
}

use std::collections::HashMap;
use std::fs;

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
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

    let mut routes: HashMap<String, Vec<String>> = HashMap::new();
    let directions: Vec<i32> = lines[0].chars().map(|x| {
        return match x {
            'L' => 0,
            'R' => 1,
            _ => 0
        };
    }).collect();

    let mut current_locations: Vec<&str> = Vec::new();

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

        if location.ends_with("A") {
            current_locations.push(location);
        }
    }

    let mut steps = vec![0; current_locations.len()];
    let mut current_direction_index: usize;

    for i in 0..current_locations.len() {
        current_direction_index = 0;
        while !current_locations[i].ends_with("Z") {
            let current_direction = directions[current_direction_index % directions.len()] as usize;

            let next_locations = routes.get(current_locations[i]).unwrap();
            current_locations[i] = next_locations[current_direction].as_str();
            steps[i] += 1;
            current_direction_index += 1;
        }
    }

    while steps.len() != 1 {
        let a = steps.pop().unwrap();
        let b = steps.pop().unwrap();

        steps.push(lcm(a, b));
    }

    println!("{:?}", steps[0]);
}

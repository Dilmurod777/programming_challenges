use std::collections::HashMap;
use std::fs;

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
    let mut current_number: String;
    let mut gear_row: i32;
    let mut gear_column: i32;
    let mut gear_items: HashMap<String, Vec<i32>> = HashMap::new();

    for i in 0..lines.len() {
        let line = lines[i];
        current_number = String::new();
        gear_row = -1;
        gear_column = -1;

        let chars: Vec<char> = line.chars().collect();

        for j in 0..chars.len() {
            if chars[j].is_numeric() {
                current_number.push(chars[j]);

                // i-1, j
                match i.checked_sub(1) {
                    None => {}
                    Some(_) => {
                        let another_line: Vec<char> = lines[i - 1].chars().collect();
                        let value = another_line[j];
                        if value == '*' {
                            gear_row = (i - 1) as i32;
                            gear_column = j as i32;
                        }
                    }
                }

                // i+1, j
                match i.checked_add(1) {
                    None => {}
                    Some(_) => {
                        if i + 1 < lines.len() {
                            let another_line: Vec<char> = lines[i + 1].chars().collect();
                            let value = another_line[j];
                            if value == '*' {
                                gear_row = (i + 1) as i32;
                                gear_column = j as i32;
                            }
                        }
                    }
                }

                // i, j-1
                match j.checked_sub(1) {
                    None => {}
                    Some(_) => {
                        let another_line: Vec<char> = lines[i].chars().collect();
                        let value = another_line[j - 1];
                        if value == '*' {
                            gear_row = i as i32;
                            gear_column = (j - 1) as i32;
                        }
                    }
                }

                // i, j+1
                match j.checked_add(1) {
                    None => {}
                    Some(_) => {
                        if j + 1 < chars.len() {
                            let another_line: Vec<char> = lines[i].chars().collect();
                            let value = another_line[j + 1];
                            if value == '*' {
                                gear_row = i as i32;
                                gear_column = (j + 1) as i32;
                            }
                        }
                    }
                }

                // i-1, j-1
                match i.checked_sub(1) {
                    None => {}
                    Some(_) => {
                        let another_line: Vec<char> = lines[i - 1].chars().collect();
                        match j.checked_sub(1) {
                            None => {}
                            Some(_) => {
                                let value = another_line[j - 1];
                                if value == '*' {
                                    gear_row = (i - 1) as i32;
                                    gear_column = (j - 1) as i32;
                                }
                            }
                        }
                    }
                }

                // i-1, j+1
                match i.checked_sub(1) {
                    None => {}
                    Some(_) => {
                        let another_line: Vec<char> = lines[i - 1].chars().collect();
                        match j.checked_add(1) {
                            None => {}
                            Some(_) => {
                                if j + 1 < chars.len() {
                                    let value = another_line[j + 1];
                                    if value == '*' {
                                        gear_row = (i - 1) as i32;
                                        gear_column = (j + 1) as i32;
                                    }
                                }
                            }
                        }
                    }
                }

                // i+1, j-1
                match i.checked_add(1) {
                    None => {}
                    Some(_) => {
                        if i + 1 < lines.len() {
                            let another_line: Vec<char> = lines[i + 1].chars().collect();
                            match j.checked_sub(1) {
                                None => {}
                                Some(_) => {
                                    let value = another_line[j - 1];
                                    if value == '*' {
                                        gear_row = (i + 1) as i32;
                                        gear_column = (j - 1) as i32;
                                    }
                                }
                            }
                        }
                    }
                }

                // i+1, j+1
                match i.checked_add(1) {
                    None => {}
                    Some(_) => {
                        if i + 1 < lines.len() {
                            let another_line: Vec<char> = lines[i + 1].chars().collect();
                            match j.checked_add(1) {
                                None => {}
                                Some(_) => {
                                    if j + 1 < chars.len() {
                                        let value = another_line[j + 1];
                                        if value == '*' {
                                            gear_row = (i + 1) as i32;
                                            gear_column = (j + 1) as i32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                add_part_to_gear(&current_number, gear_row, gear_column, &mut gear_items);

                current_number.clear();
                gear_column = -1;
            }
        }

        add_part_to_gear(&current_number, gear_row, gear_column, &mut gear_items);
    }

    for (_, values) in gear_items {
        if values.len() == 2 {
            total += values[0] * values[1];
        }
    }

    println!("{}", total);
}

fn add_part_to_gear(current_number: &String, gear_row: i32, gear_column: i32, gear_items: &mut HashMap<String, Vec<i32>>){
    if current_number != "" {
        if gear_column != -1 {
            let key = format!("{}-{}", gear_row, gear_column);
            let value: i32 = current_number.parse().unwrap();
            gear_items.entry(key.clone()).or_default().push(value);
        }
    }
}

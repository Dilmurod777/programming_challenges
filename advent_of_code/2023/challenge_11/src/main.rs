use std::cmp::{max, min};
use std::fs;

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn get_empty_columns_between(start: &Galaxy, end: &Galaxy, empty_columns: &Vec<bool>) -> usize {
    let mut count: usize = 0;
    let min_x = min(start.x, end.x);
    let max_x = max(start.x, end.x);


    for i in min_x..max_x + 1 {
        if empty_columns[i] {
            count += 1;
        }
    }

    return count;
}

fn get_empty_rows_between(start: &Galaxy, end: &Galaxy, empty_rows: &Vec<bool>) -> usize {
    let mut count: usize = 0;
    let min_y = min(start.y, end.y);
    let max_y = max(start.y, end.y);

    for i in min_y..max_y + 1 {
        if empty_rows[i] {
            count += 1;
        }
    }

    return count;
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

    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut empty_columns: Vec<bool> = vec![true; lines[0].len()];
    let mut empty_rows: Vec<bool> = vec![true; lines.len()];

    for i in 0..lines.len() {
        let line = lines[i];
        let chars: Vec<char> = line.chars().collect();
        let mut has_galaxy = false;

        for j in 0..chars.len() {
            if chars[j] == '#' {
                galaxies.push(Galaxy {
                    x: j,
                    y: i,
                });
                empty_columns[j] = false;
                has_galaxy = true;
            }
        }

        if has_galaxy {
            empty_rows[i] = false;
        }
    }

    let mut total = 0;
    let expansion_ratio: usize = 1000000;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let start = &galaxies[i];
            let end = &galaxies[j];

            let path = (end.x.abs_diff(start.x)) + (end.y.abs_diff(start.y));
            let empty_count_columns = get_empty_columns_between(start, end, &empty_columns) * (expansion_ratio - 1);
            let empty_count_rows = get_empty_rows_between(start, end, &empty_rows) * (expansion_ratio - 1);

            total += path + empty_count_columns + empty_count_rows;
        }
    }

    println!("{}", total);
}

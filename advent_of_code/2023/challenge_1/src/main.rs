use std::fs;
use std::ops::{Add, Sub};

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

    let mut sum: i32 = 0;
    let mut i: usize;
    let mut j: usize;
    let mut first_digit_value: i32;
    let mut last_digit_value: i32;
    let mut first_digit_idx: i32;
    let mut last_digit_idx: i32;

    let digits_as_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in lines {
        i = 0;
        j = line.len() - 1;
        first_digit_value = -1;
        last_digit_value = -1;
        first_digit_idx = -1;
        last_digit_idx = -1;
        let chars: Vec<char> = line.chars().collect();

        while first_digit_value == -1 || last_digit_value == -1 {
            if first_digit_value == -1 {
                if chars[i].is_numeric() {
                    first_digit_value = chars[i].to_digit(10).unwrap() as i32;
                    first_digit_idx = i as i32;
                }
            }

            if last_digit_value == -1 {
                if chars[j].is_numeric() {
                    last_digit_value = chars[j].to_digit(10).unwrap() as i32;
                    last_digit_idx = j as i32;
                }
            }

            match i.checked_add(1) {
                None => {
                    if j == 0 {
                        break;
                    }
                }
                Some(_) => {
                    i = i.add(1);
                }
            }

            match j.checked_sub(1) {
                None => {
                    if i == chars.len() {
                        break;
                    }
                }
                Some(_) => {
                    j = j.sub(1);
                }
            }
        }


        for idx in 0..digits_as_words.len() {
            let left_search_result = line.find(digits_as_words[idx]);
            let right_search_result = line.rfind(digits_as_words[idx]);
            let value = idx + 1;

            match left_search_result {
                None => continue,
                Some(search_idx) => {
                    if first_digit_idx == -1 {
                        first_digit_value = value as i32;
                        first_digit_idx = search_idx as i32;
                    } else if search_idx < (first_digit_idx as usize) {
                        first_digit_value = value as i32;
                        first_digit_idx = search_idx as i32;
                    }
                }
            }

            match right_search_result {
                None => continue,
                Some(search_idx) => {
                    if last_digit_idx == -1 {
                        last_digit_value = value as i32;
                        last_digit_idx = search_idx as i32;
                    } else if search_idx > (last_digit_idx as usize) {
                        last_digit_value = value as i32;
                        last_digit_idx = search_idx as i32;
                    }
                }
            }
        }

        sum += first_digit_value * 10 + last_digit_value;
    }

    println!("{sum}");
}

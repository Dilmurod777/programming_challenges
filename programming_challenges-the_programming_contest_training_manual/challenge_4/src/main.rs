use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

static DASH: &'static str = "-";
static BAR: &'static str = "|";
static SPACE: &'static str = " ";

fn get_max_rows(s: usize) -> usize {
    2 * s + 3
}

fn get_max_cols(s: usize) -> usize {
    s + 2
}

fn get_row_for_digit(digit: &u32, index: usize, s: usize) -> String {
    match digit {
        0 => get_row_for_zero(index, s),
        1 => get_row_for_one(index, s),
        2 => get_row_for_two(index, s),
        3 => get_row_for_three(index, s),
        4 => get_row_for_four(index, s),
        5 => get_row_for_five(index, s),
        6 => get_row_for_six(index, s),
        7 => get_row_for_seven(index, s),
        8 => get_row_for_eight(index, s),
        9 => get_row_for_nine(index, s),
        _ => String::new()
    }
}

fn get_row_for_zero(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ => format!("{}{}{}", BAR, SPACE.repeat(get_max_cols(s) - 2), BAR)
    }
}

fn get_row_for_one(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}", SPACE.repeat(get_max_cols(s))),
        _ if index == last_index => format!("{}", SPACE.repeat(get_max_cols(s))),
        _ if index == mid_index => format!("{}", SPACE.repeat(get_max_cols(s))),
        _ => format!("{}{}", SPACE.repeat(get_max_cols(s) - 1), BAR)
    }
}

fn get_row_for_two(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == mid_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index < mid_index => format!("{}{}", SPACE.repeat(get_max_cols(s) - 1), BAR),
        _ => format!("{}{}", BAR, SPACE.repeat(get_max_cols(s) - 1)),
    }
}

fn get_row_for_three(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == mid_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ => format!("{}{}", SPACE.repeat(get_max_cols(s) - 1), BAR)
    }
}

fn get_row_for_four(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}", SPACE.repeat(get_max_cols(s))),
        _ if index == last_index => format!("{}", SPACE.repeat(get_max_cols(s))),
        _ if index == mid_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index < mid_index => format!("{}{}{}", BAR, SPACE.repeat(get_max_cols(s) - 2), BAR),
        _ => format!("{}{}", SPACE.repeat(get_max_cols(s) - 1), BAR)
    }
}

fn get_row_for_five(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == mid_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index < mid_index => format!("{}{}", BAR, SPACE.repeat(get_max_cols(s) - 1)),
        _ => format!("{}{}", SPACE.repeat(get_max_cols(s) - 1), BAR)
    }
}

fn get_row_for_six(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == mid_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index < mid_index => format!("{}{}", BAR, SPACE.repeat(get_max_cols(s) - 1)),
        _ => format!("{}{}{}", BAR, SPACE.repeat(get_max_cols(s) - 2), BAR)
    }
}

fn get_row_for_seven(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}", SPACE.repeat(get_max_cols(s))),
        _ => format!("{}{}", SPACE.repeat(get_max_cols(s) - 1), BAR)
    }
}

fn get_row_for_eight(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == mid_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ => format!("{}{}{}", BAR, SPACE.repeat(get_max_cols(s) - 2), BAR)
    }
}

fn get_row_for_nine(index: usize, s: usize) -> String {
    let last_index = get_max_rows(s) - 1;
    let mid_index = get_max_rows(s) / 2;

    match index {
        0 => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == last_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index == mid_index => format!("{}{}{}", SPACE, DASH.repeat(get_max_cols(s) - 2), SPACE),
        _ if index < mid_index => format!("{}{}{}", BAR, SPACE.repeat(get_max_cols(s) - 2), BAR),
        _ => format!("{}{}", SPACE.repeat(get_max_cols(s) - 1), BAR)
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

    for i in 0..lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() == 1 && parts[0] == "0" {
            break;
        }

        let s = parts[0].parse::<usize>().expect("Could not parse number. Try again!");
        let n = parts[1];
        let digits: Vec<u32> = n.chars().map(|x| x.to_digit(10).expect("Could not convert to digit. Try again!")).collect();

        for row_index in 0..(2 * s + 3) {
            let mut row = String::new();
            for digit in &digits {
                row += get_row_for_digit(digit, row_index, s).as_str();
                row += SPACE;
            }

            let mut output_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_filename)
                .expect("Cannot open file. Please, check the path!");

            output_file
                .write_fmt(format_args!("{}\n", row))
                .expect("Write to output file failed. Try again!");
        }
    }
}
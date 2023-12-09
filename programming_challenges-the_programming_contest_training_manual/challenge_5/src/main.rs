use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;

pub static WHITE: char = 'O';

fn parse_parameter<T: FromStr>(param: &str) -> T {
    return param.parse::<T>().unwrap_or_else(|_| panic!("Could not parse {param}"));
}

fn recursive_fill_image_cell(x: usize, y: usize, c: char, image: &mut Vec<Vec<char>>) {
    if image[y][x] == c {
        return;
    }

    image[y][x] = c;

    let rows = image.len();
    let cols = image[0].len();

    match y.checked_sub(1) {
        None => {}
        Some(_) => {
            recursive_fill_image_cell(x, y - 1, c, image);
        }
    }

    match x.checked_sub(1) {
        None => {}
        Some(_) => {
            recursive_fill_image_cell(x - 1, y, c, image);
        }
    }

    if y + 1 < rows {
        recursive_fill_image_cell(x, y + 1, c, image);
    }

    if x + 1 < cols {
        recursive_fill_image_cell(x + 1, y, c, image);
    }
}

fn main() {
    let input_filename = "input.txt";
    let output_filename = "output.txt";

    let _ = fs::remove_file(output_filename);

    let contents = fs::read_to_string(input_filename)
        .expect("Cannot read file. Please, check the path!");

    let lines: Vec<&str> = contents
        .split("\n")
        .map(|x| x.trim())
        .collect();

    let mut image: Vec<Vec<char>> = vec![];
    let mut m: usize = 0;
    let mut n: usize = 0;

    let mut names: Vec<&str> = Vec::new();
    let mut images: Vec<Vec<Vec<char>>> = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split(" ").collect();
        let command = parts[0];

        match command {
            "I" => {
                m = parse_parameter::<usize>(parts[1]);
                n = parse_parameter::<usize>(parts[2]);

                image = vec!(vec![WHITE; m]; n);
            }
            "C" => {
                for i in 0..n {
                    for j in 0..m {
                        image[i][j] = WHITE;
                    }
                }
            }
            "L" => {
                let x = parse_parameter::<usize>(parts[1]) - 1;
                let y = parse_parameter::<usize>(parts[2]) - 1;
                let c = parse_parameter::<char>(parts[3]);

                image[y][x] = c;
            }
            "V" => {
                let x = parse_parameter::<usize>(parts[1]) - 1;
                let y1 = parse_parameter::<usize>(parts[2]) - 1;
                let y2 = parse_parameter::<usize>(parts[3]) - 1;
                let c = parse_parameter::<char>(parts[4]);

                for i in y1..y2 + 1 {
                    image[i][x] = c;
                }
            }
            "H" => {
                let x1 = parse_parameter::<usize>(parts[1]) - 1;
                let x2 = parse_parameter::<usize>(parts[2]) - 1;
                let y = parse_parameter::<usize>(parts[3]) - 1;
                let c = parse_parameter::<char>(parts[4]);

                for i in x1..x2 + 1 {
                    image[y][i] = c;
                }
            }
            "K" => {
                let x1 = parse_parameter::<usize>(parts[1]) - 1;
                let y1 = parse_parameter::<usize>(parts[2]) - 1;
                let x2 = parse_parameter::<usize>(parts[3]) - 1;
                let y2 = parse_parameter::<usize>(parts[4]) - 1;
                let c = parse_parameter::<char>(parts[5]);

                for i in y1..y2 + 1 {
                    for j in x1..x2 + 1 {
                        image[i][j] = c;
                    }
                }
            }
            "F" => {
                let x = parse_parameter::<usize>(parts[1]) - 1;
                let y = parse_parameter::<usize>(parts[2]) - 1;
                let c = parse_parameter::<char>(parts[3]);

                recursive_fill_image_cell(x, y, c, &mut image);
            }
            "S" => {
                let name = parts[1];

                names.push(name);
                images.push(image.clone());
            }
            "X" => {
                break;
            }
            &_ => {}
        };
    }

    let mut output_value = String::new();

    for i in 0..names.len(){
        output_value.push_str(format!("{}\n", names[i]).as_str());

        for j in 0..n {
            for k in 0..m {
                output_value.push(images[i][j][k]);
            }
            output_value.push('\n');
        }
    }

    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_filename)
        .expect("Cannot open file. Please, check the path!");

    output_file
        .write_fmt(format_args!("{}", output_value))
        .expect("Write to output file failed. Try again!");
}
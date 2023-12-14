use std::fs;

fn move_next_pipe(field: &mut Vec<Vec<char>>, current_position: &Vec<usize>, initial_position: &Vec<usize>, step_count: i32) -> Vec<usize> {
    let y = current_position[0];
    let x = current_position[1];
    let rows = field.len();
    let cols = field[0].len();
    let mut possible_moves: Vec<Vec<usize>> = Vec::new();

    // println!("{:?}", current_position);
    let current_pipe = field[y][x];

    // y-1, x
    if current_pipe == 'S' || current_pipe == '|' || current_pipe == 'L' || current_pipe == 'J' {
        match y.checked_sub(1) {
            None => {}
            Some(_) => {
                if y - 1 == initial_position[0] && x == initial_position[1] && step_count >= 3 {
                    return vec![];
                }

                if field[y - 1][x] == '|' || field[y - 1][x] == '7' || field[y - 1][x] == 'F' {
                    possible_moves.push(vec![y - 1, x]);
                }
            }
        }
    }

    // y, x+1
    if current_pipe == 'S' || current_pipe == '-' || current_pipe == 'L' || current_pipe == 'F' {
        match x.checked_add(1) {
            None => {}
            Some(_) => {
                if y == initial_position[0] && x + 1 == initial_position[1] && step_count >= 3 {
                    return vec![];
                }

                if x + 1 < cols {
                    if field[y][x + 1] == '-' || field[y][x + 1] == 'J' || field[y][x + 1] == '7' {
                        possible_moves.push(vec![y, x + 1]);
                    }
                }
            }
        }
    }

    // y+1, x
    if current_pipe == 'S' || current_pipe == '|' || current_pipe == '7' || current_pipe == 'F' {
        match y.checked_add(1) {
            None => {}
            Some(_) => {
                if y + 1 == initial_position[0] && x == initial_position[1] && step_count >= 3 {
                    return vec![];
                }

                if y + 1 < rows {
                    if field[y + 1][x] == '|' || field[y + 1][x] == 'L' || field[y + 1][x] == 'J' {
                        possible_moves.push(vec![y + 1, x]);
                    }
                }
            }
        }
    }

    // y, x-1
    if current_pipe == 'S' || current_pipe == '-' || current_pipe == 'J' || current_pipe == '7' {
        match x.checked_sub(1) {
            None => {}
            Some(_) => {
                if y == initial_position[0] && x - 1 == initial_position[1] && step_count >= 3 {
                    return vec![];
                }

                if field[y][x - 1] == '-' || field[y][x - 1] == 'L' || field[y][x - 1] == 'F' {
                    possible_moves.push(vec![y, x - 1]);
                }
            }
        }
    }

    field[y][x] = '.';
    return possible_moves[0].clone();
}

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

    let mut field: Vec<Vec<char>> = Vec::new();
    let mut initial_position: Vec<usize> = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i];
        let chars = line.chars().collect::<Vec<char>>();

        for j in 0..chars.len() {
            if chars[j] == 'S' {
                initial_position = vec![i, j];
                break;
            }
        }

        field.push(chars);
    }

    let mut current_position = initial_position.clone();
    let mut step_count = 0;
    loop {
        current_position = move_next_pipe(&mut field, &current_position, &initial_position, step_count);

        if current_position.len() == 0 {
            break;
        } else {
            step_count += 1;
        }
    }

    println!("{}", (step_count + 1) / 2);
}

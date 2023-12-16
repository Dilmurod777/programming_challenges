use std::cmp::min;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn get_enemy_king(cell: &char) -> char {
    if cell.is_lowercase() { 'K' } else { 'k' }
}

fn check_horizontal(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let cols = field[0].len();
    let enemy_king = get_enemy_king(&field[i][j]);

    for k in j + 1..cols {
        if field[i][k] == enemy_king {
            return true;
        } else if field[i][k] != '.' {
            break;
        }
    }

    for k in (0..j.checked_sub(1).unwrap_or(j)).rev() {
        if field[i][k] == enemy_king {
            return true;
        } else if field[i][k] != '.' {
            break;
        }
    }

    return false;
}

fn check_vertical(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let rows = field.len();
    let enemy_king = get_enemy_king(&field[i][j]);

    for k in i + 1..rows {
        if field[i][k] == enemy_king {
            return true;
        } else if field[i][k] != '.' {
            break;
        }
    }

    for k in (0..i.checked_sub(1).unwrap_or(i)).rev() {
        if field[i][k] == enemy_king {
            return true;
        } else if field[i][k] != '.' {
            break;
        }
    }

    return false;
}

fn check_diagonals(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let rows = field.len();
    let cols = field[0].len();
    let enemy_king = get_enemy_king(&field[i][j]);
    let upper_limit = j + min(cols - j, rows - i);
    let lower_limit = min(j, i);

    for k in j + 1..upper_limit {
        if field[k][k] == enemy_king || field[rows - k - 1][k] == enemy_king {
            return true;
        } else if field[k][k] != '.' {
            break;
        }
    }

    for k in (0..lower_limit).rev() {
        if field[k][k] == enemy_king || field[rows - k - 1][k] == enemy_king {
            return true;
        } else if field[k][k] != '.' {
            break;
        }
    }

    return false;
}

fn check_pawn(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let rows = field.len();
    let cols = field[0].len();
    let enemy_king = get_enemy_king(&field[i][j]);
    let mut positions = vec![vec![]; 0];

    if enemy_king == 'K' {
        positions.push(vec![i + 1, j.checked_sub(1).unwrap_or(j)]);
        positions.push(vec![i + 1, j + 1]);
    } else {
        positions.push(vec![i.checked_sub(1).unwrap_or(i), j.checked_sub(1).unwrap_or(j)]);
        positions.push(vec![i.checked_sub(1).unwrap_or(i), j + 1]);
    }

    for position in positions {
        if position[0] < rows && position[1] < cols {
            if field[position[0]][position[1]] == enemy_king {
                return true;
            }
        }
    }

    false
}

fn check_knight(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let rows = field.len();
    let cols = field[0].len();
    let enemy_king = get_enemy_king(&field[i][j]);
    let positions = vec![
        vec![i.checked_sub(2).unwrap_or(i), j.checked_sub(1).unwrap_or(j)],
        vec![i.checked_sub(2).unwrap_or(i), j + 1],
        vec![i.checked_sub(1).unwrap_or(i), j.checked_sub(2).unwrap_or(j)],
        vec![i.checked_sub(1).unwrap_or(i), j + 2],
        vec![i + 1, j.checked_sub(2).unwrap_or(j)],
        vec![i + 1, j + 2],
        vec![i + 2, j.checked_sub(1).unwrap_or(j)],
        vec![i + 2, j + 1],
    ];

    for position in positions {
        if position[0] < rows && position[1] < cols {
            if field[position[0]][position[1]] == enemy_king {
                return true;
            }
        }
    }

    return false;
}

fn check_bishop(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return check_diagonals(field, i, j);
}

fn check_rook(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return check_horizontal(field, i, j) || check_vertical(field, i, j);
}

fn check_queen(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return check_vertical(field, i, j) || check_horizontal(field, i, j) || check_diagonals(field, i, j);
}

fn check_cell_for_check(field: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let cell = &field[i][j];

    return match cell {
        'p' | 'P' => {
            return check_pawn(field, i, j);
        }
        'n' | 'N' => {
            return check_knight(field, i, j);
        }
        'b' | 'B' => {
            return check_bishop(field, i, j);
        }
        'r' | 'R' => {
            return check_rook(field, i, j);
        }
        'q' | 'Q' => {
            return check_queen(field, i, j);
        }
        _ => {
            false
        }
    };
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

    let mut field: Vec<Vec<char>> = Vec::new();
    let mut game_count = 0;
    let mut game_result: String;
    let mut is_game_checked: bool;

    for k in 0..lines.len() {
        let line = lines[k];
        if line.is_empty() {
            game_count += 1;
            is_game_checked = false;
            game_result = String::from("no king is in check");

            for i in 0..field.len() {
                for j in 0..field[0].len() {
                    let cell = field[i][j];

                    if cell == 'k' || cell == 'K' || cell == '.' {
                        continue;
                    }

                    if check_cell_for_check(&field, i, j) {
                        let color = if cell.is_lowercase() { "white" } else { "black" };
                        game_result = format!("{} is in check.", color);
                        is_game_checked = true;
                        break;
                    }
                }

                if is_game_checked {
                    break;
                }
            }

            field.clear();

            let mut output_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_filename)
                .expect("Cannot open file. Please, check the path!");

            output_file
                .write_fmt(format_args!("Game #{}: {}\n", game_count, game_result))
                .expect("Write to output file failed. Try again!");

            continue;
        }

        field.push(line.chars().collect::<Vec<char>>());
    }
}
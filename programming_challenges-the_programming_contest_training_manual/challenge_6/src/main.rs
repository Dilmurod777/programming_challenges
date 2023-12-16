use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
struct Instruction {
    category: usize,
    source: usize,
    destination: usize,
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

    let mut executed_instructions_count = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut registers: Vec<usize> = vec![0; 10];
    const MAX_REGISTER_VALUE: usize = 1000;

    let test_cases_number = lines[0].parse::<i32>().unwrap();
    let mut line_number = 2;

    let mut current_instruction_index = 0;

    for _ in 0..test_cases_number {
        for i in line_number..lines.len() {
            line_number += 1;

            let line = lines[i];
            if line.is_empty() {
                break;
            }

            let chars: Vec<char> = line.chars().collect();

            instructions.push(Instruction {
                category: chars[0].to_digit(10).unwrap() as usize,
                source: chars[1].to_digit(10).unwrap() as usize,
                destination: chars[2].to_digit(10).unwrap() as usize,
            })
        }

        loop {
            let instruction = &instructions[current_instruction_index];
            executed_instructions_count += 1;
            current_instruction_index += 1;

            match instruction.category {
                0 => {
                    if registers[instruction.destination] != 0 {
                        current_instruction_index = registers[instruction.source];
                    }
                }
                1 => {
                    if instruction.source == 0 && instruction.destination == 0 {
                        break;
                    }
                }
                2 => {
                    registers[instruction.source] = instruction.destination;
                }
                3 => {
                    registers[instruction.source] += instruction.destination;
                    registers[instruction.source] %= MAX_REGISTER_VALUE;
                }
                4 => {
                    registers[instruction.source] *= instruction.destination;
                    registers[instruction.source] %= MAX_REGISTER_VALUE;
                }
                5 => {
                    registers[instruction.source] = registers[instruction.destination];
                    registers[instruction.source] %= MAX_REGISTER_VALUE;
                }
                6 => {
                    registers[instruction.source] += registers[instruction.destination];
                    registers[instruction.source] %= MAX_REGISTER_VALUE;
                }
                7 => {
                    registers[instruction.source] *= registers[instruction.destination];
                    registers[instruction.source] %= MAX_REGISTER_VALUE;
                }
                _ => {}
            }
        }

        instructions.clear();
        registers.clear();
        current_instruction_index = 0;
    }

    println!("{:?}", executed_instructions_count);

    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_filename)
        .expect("Cannot open file. Please, check the path!");

    output_file
        .write_fmt(format_args!("{}\n", executed_instructions_count))
        .expect("Write to output file failed. Try again!");
}
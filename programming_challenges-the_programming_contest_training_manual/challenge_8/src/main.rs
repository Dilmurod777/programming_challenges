use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

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

    let trials_count = lines[0].parse::<i32>().unwrap();
    let mut line_number = 2;
    let mut candidates_number: usize = 0;
    let mut candidates: Vec<&str> = Vec::new();
    let mut votes: Vec<Vec<usize>> = Vec::new();
    let mut winners: Vec<usize> = Vec::new();
    let mut freqs: Vec<usize> = Vec::new();
    let mut eliminated_votes: Vec<usize>;

    for _ in 0..trials_count {
        for i in line_number..lines.len() {
            line_number += 1;
            let line = lines[i];

            if line.is_empty() {
                candidates_number = 0;
                candidates.clear();
                votes.clear();
                winners.clear();
                freqs.clear();
                break;
            }

            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() == 1 {
                candidates_number = parts[0].parse::<usize>().unwrap();
                freqs = vec![0; candidates_number]
            } else if candidates.len() < candidates_number {
                candidates.push(line);
            } else {
                votes.push(parts.iter().map(|x| x.parse::<usize>().unwrap()).collect());
            }
        }

        let mut current_stage: usize = 0;
        let mut max_votes: usize = 0;

        for i in 0..votes.len() {
            freqs[votes[i][current_stage] - 1] += 1;
            if freqs[votes[i][current_stage] - 1] > max_votes {
                max_votes = freqs[votes[i][current_stage] - 1];
            }
        }

        while winners.len() != 1 {
            winners = freqs
                .iter()
                .map(|x| *x == max_votes)
                .enumerate()
                .map(|(i, v)| if v { i } else { votes.len() + 1 })
                .filter(|x| *x != votes.len() + 1)
                .collect();
            eliminated_votes = votes
                .iter()
                .map(|v| { !winners.contains(&(v[current_stage] - 1)) })
                .enumerate()
                .map(|(i, v)| { if v { i } else { votes.len() + 1 } })
                .filter(|x| { *x != votes.len() + 1 })
                .collect();

            for i in &eliminated_votes {
                freqs[votes[*i][current_stage + 1] - 1] += 1;
                if freqs[votes[*i][current_stage + 1] - 1] > max_votes {
                    max_votes = freqs[votes[*i][current_stage + 1] - 1];
                }
            }

            current_stage += 1;
        }
    }


    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_filename)
        .expect("Cannot open file. Please, check the path!");

    output_file
        .write_fmt(format_args!("{}\n", candidates[winners[0]]))
        .expect("Write to output file failed. Try again!");
}
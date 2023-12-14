use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum HandType { None, FiveOfKind, FourOfKind, FullHouse, ThreeOfKind, TwoPair, OnePair, HighCard }

#[derive(Debug)]
struct Hand {
    order: String,
    rank: i32,
    hand_type: HandType,
    bid: i32,
}

fn calculate_hand_type(order: &str) -> HandType {
    let mut freqs: HashMap<String, i64> = HashMap::new();

    for v in order.chars() {
        freqs.entry(String::from(v)).or_default();
        *freqs.get_mut(String::from(v).as_str()).unwrap() += 1;
    }

    println!("{:?}", freqs);

    match freqs.keys().len() {
        5 => { return HandType::HighCard; }
        4 => {
            return HandType::OnePair;
        }
        3 => {
            for key in freqs.keys() {
                if freqs[key] == 3 {
                    return HandType::ThreeOfKind;
                }
            }

            return HandType::TwoPair;
        }
        2 => {
            for key in freqs.keys() {
                if freqs[key] == 4 {
                    return HandType::FourOfKind;
                }
            }

            return HandType::FullHouse;
        }
        1 => { return HandType::FiveOfKind; }
        _ => { return HandType::None; }
    }
}

fn main() {
    let input_filename = "input1.txt";
    let output_filename = "output.txt";

    let _ = fs::remove_file(output_filename);

    let contents = fs::read_to_string(input_filename)
        .expect("Cannot read file. Please, check the path!");

    let lines: Vec<&str> = contents
        .split("\n")
        .map(|x| x.trim())
        .collect();

    let mut hands: Vec<Hand> = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split(" ").collect();

        hands.push(Hand {
            order: String::from(parts[0]),
            rank: -1,
            bid: parts[1].parse::<i32>().unwrap(),
            hand_type: calculate_hand_type(&parts[0]),
        });
    }

    for hand in hands {
        println!("{:?}", hand);
    }
}

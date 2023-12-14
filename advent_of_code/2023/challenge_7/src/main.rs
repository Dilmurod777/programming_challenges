use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum HandType { None, FiveOfKind, FourOfKind, FullHouse, ThreeOfKind, TwoPair, OnePair, HighCard }

#[derive(Debug)]
struct Card {
    value: String,
    bid: i32,
}

fn calculate_hand_type(order: &str) -> &HandType {
    let mut freqs: HashMap<String, i64> = HashMap::new();

    for v in order.chars() {
        freqs.entry(String::from(v)).or_default();
        *freqs.get_mut(String::from(v).as_str()).unwrap() += 1;
    }

    return match freqs.keys().len() {
        5 => &HandType::HighCard,
        4 => &HandType::OnePair,
        3 => {
            for v in freqs.values() {
                if v == &3 {
                    return &HandType::ThreeOfKind;
                }
            }

            &HandType::TwoPair
        },
        2 => {
            for v in freqs.values() {
                if v == &4 {
                    return &HandType::FourOfKind;
                }
            }

            &HandType::FullHouse
        }
        1 => &HandType::FiveOfKind,
        _ => &HandType::None
    };
}

fn get_str_as_numbers(value: &str) -> Vec<u32> {
    value.chars().map(|x| {
        return match x {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => x.to_digit(10).unwrap()
        };
    }).collect()
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

    let mut hands: HashMap<&HandType, Vec<Card>> = HashMap::from([
        (&HandType::FiveOfKind, vec![]),
        (&HandType::FourOfKind, vec![]),
        (&HandType::FullHouse, vec![]),
        (&HandType::ThreeOfKind, vec![]),
        (&HandType::TwoPair, vec![]),
        (&HandType::OnePair, vec![]),
        (&HandType::HighCard, vec![]),
    ]);

    for i in 0..lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split(" ").collect();
        let hand_type = calculate_hand_type(&parts[0]);

        hands.get_mut(&hand_type).unwrap().push(Card {
            value: String::from(parts[0]),
            bid: parts[1].parse::<i32>().unwrap(),
        });
    }

    for cards in hands.values_mut() {
        cards.sort_by(|c1, c2| {
            let c1_chars: Vec<u32> = get_str_as_numbers(c1.value.as_str());
            let c2_chars: Vec<u32> = get_str_as_numbers(c2.value.as_str());

            for i in 0..5 {
                if c1_chars[i] > c2_chars[i] {
                    return Ordering::Greater;
                } else if c1_chars[i] < c2_chars[i] {
                    return Ordering::Less;
                }
            }

            return Ordering::Equal;
        });
    }


    let mut total = 0;
    let mut current_rank = 1;
    let order = vec![
        HandType::HighCard, HandType::OnePair, HandType::TwoPair,
        HandType::ThreeOfKind, HandType::FullHouse,
        HandType::FourOfKind, HandType::FiveOfKind];

    for key in order{
        if let Some(cards) = hands.get(&key) {
            for card in cards {
                total += current_rank * card.bid;
                current_rank += 1;
            }
        }
    }

    println!("{}", total);
}

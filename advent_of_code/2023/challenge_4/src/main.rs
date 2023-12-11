use std::fs;

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

    let mut total = 0;
    let mut cards_counts: Vec<i32> = vec![1; lines.len()];

    for i in 0..lines.len() {
        let line = lines[i];
        let mut card_total = 0;

        let parts: Vec<&str> = line.split(":").collect();
        let cards: Vec<&str> = parts[1].trim().split("|").collect();

        let winning_cards: Vec<&str> = cards[0].trim().split(" ").filter(|x| x.trim().len() > 0).collect();
        let winning_cards: Vec<i32> = winning_cards.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        let possessed_cards: Vec<&str> = cards[1].trim().split(" ").filter(|x| x.trim().len() > 0).collect();
        let possessed_cards: Vec<i32> = possessed_cards.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        for card in possessed_cards {
            if winning_cards.contains(&card) {
                card_total += 1;
            }
        }

        for j in i + 1..i + card_total + 1 {
            if j < cards_counts.len() {
                cards_counts[j] += cards_counts[i];
            }
        }
    }

    for count in &cards_counts {
        total += count;
    }

    println!("{}", total);
}
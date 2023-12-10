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

    let mut min_red: i32;
    let mut min_green: i32;
    let mut min_blue: i32;

    for i in 0..lines.len() {
        min_red = 0;
        min_green = 0;
        min_blue = 0;

        let parts: Vec<&str> = lines[i].trim().split(":").collect();
        let sets: Vec<&str> = parts[1].trim().split(";").collect();

        for set in sets {
            let items: Vec<&str> = set.trim().split(",").collect();

            for item in items {
                let item_parts: Vec<&str> = item.trim().split(" ").collect();
                let value: i32 = item_parts[0].parse().unwrap();
                let color = item_parts[1];

                if color == "red" && value > min_red {
                    min_red = value;
                }

                if color == "green" && value > min_green {
                    min_green = value;
                }

                if color == "blue" && value > min_blue {
                    min_blue = value;
                }
            }
        }

        total += min_red * min_green * min_blue;
    }

    println!("Total: {total}");
}

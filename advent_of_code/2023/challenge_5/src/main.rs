use std::cmp::min;
use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    count: i64,
    diff: i64,
}

#[derive(Debug)]
struct Seed {
    start: i64,
    count: i64,
}

fn update_ranges(ranges: &mut Vec<Range>, parts: &Vec<i64>) {
    let s = parts[1];
    let d = parts[0];
    let c = parts[2];

    if ranges.len() == 0 {
        ranges.push(Range {
            start: 0,
            count: s,
            diff: 0,
        });

        ranges.push(Range {
            start: s,
            count: c,
            diff: d - s,
        })
    } else {
        for i in 0..ranges.len() {
            let range = &ranges[i];
            let start = range.start;
            let count = range.count;
            let diff = range.diff;

            if s < start {
                if s + c <= start {
                    update_ranges(ranges, &vec![d, s, c]);
                    break;
                } else {
                    update_ranges(ranges, &vec![d, start, start - s]);
                    update_ranges(ranges, &vec![d, start, s + c - start]);
                    break;
                }
            } else {
                if s < start + count {
                    if s + c <= start + count {
                        ranges.insert(i + 1, Range {
                            start: start,
                            count: s - start,
                            diff: diff,
                        });

                        ranges.insert(i + 2, Range {
                            start: s,
                            count: c,
                            diff: diff + d - s,
                        });

                        ranges.insert(i + 3, Range {
                            start: s + c,
                            count: start + count - s - c,
                            diff: diff,
                        });

                        ranges.remove(i);

                        break;
                    } else {
                        ranges.insert(i + 1, Range {
                            start: start,
                            count: s - start,
                            diff: diff,
                        });

                        ranges.insert(i + 2, Range {
                            start: s,
                            count: start + count - s,
                            diff: diff + d - s,
                        });

                        ranges.remove(i);
                        update_ranges(ranges, &vec![d - s + start + count, start + count, c - start - count + s]);
                        break;
                    }
                } else {
                    continue;
                }
            }
        }
    }
}

fn clean_empty_ranges(ranges: &mut Vec<Range>) {
    let mut indexes = vec![0; 0];

    for i in 0..ranges.len() {
        if ranges[i].count == 0 {
            indexes.push(i);
        }
    }

    for i in 0..indexes.len() {
        ranges.remove(indexes[indexes.len() - i - 1]);
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

    let mut seeds: Vec<Seed> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i];

        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds") {
            let parts: Vec<&str> = line.split(":").collect();
            let values: Vec<i64> = parts[1].trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

            for i in 0..values.len() / 2 {
                seeds.push(Seed {
                    start: values[i * 2],
                    count: values[i * 2 + 1],
                });
            }


            // println!("Seeds: {:?}", seeds);
            continue;
        }

        if !line.contains("map:") && !line.starts_with("//") {
            let parts: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            update_ranges(&mut ranges, &parts);
            clean_empty_ranges(&mut ranges);
            // println!("{:?}", parts);
            // println!("{:?}", ranges);
            // println!("------------------------");
        }
    }

    // for range in &ranges {
    //     println!(" - {:?}", range);
    // }

    let mut min_location = -1;

    for seed in seeds {
        let mut start_value = seed.start;
        let end_value = seed.start + seed.count;
        let mut found = false;

        while start_value < end_value {
            for range in &ranges {
                println!("{} | {} | {} | {:?}", start_value, end_value, min_location, range);
                if range.start <= start_value && start_value < range.start + range.count {
                    if min_location == -1 {
                        min_location = start_value + range.diff;
                    } else {
                        min_location = min(min_location, start_value + range.diff)
                    }

                    start_value = range.start + range.count;
                    found = true;
                    if start_value >= end_value {
                        break;
                    }
                }
            }

            if !found {
                break;
            }
        }

        if min_location == -1{
            min_location = start_value;
        }
    }

    println!("{}", min_location);
}
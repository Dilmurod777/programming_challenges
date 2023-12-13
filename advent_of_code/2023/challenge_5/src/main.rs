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

    while !seeds.is_empty() {
        let seed = seeds.pop().unwrap();
        let s_start = seed.start;
        let s_end = seed.start + seed.count;
        let s_count = seed.count;

        if s_count == 0 {
            continue;
        }

        for range in &ranges {
            let r_start = range.start;
            let r_end = range.start + range.count;
            let r_diff = range.diff;

            println!("{} {} | {} {} {} | {}", s_start, s_end, r_start, r_end, r_diff, min_location);
            if s_start >= r_start && s_start < r_end {
                if s_end < r_end {
                    min_location = get_minimum(s_start + r_diff, min_location);
                } else {
                    min_location = get_minimum(s_start + r_diff, min_location);
                    seeds.push(Seed {
                        start: r_end,
                        count: s_count - (r_end - s_start),
                    })
                }
                break;
            }
        }
    }

    println!("{}", min_location);
}

fn get_minimum(value: i64, rel_min: i64) -> i64 {
    return if rel_min == -1 {
        value
    } else {
        min(rel_min, value)
    };
}
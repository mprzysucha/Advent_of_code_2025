use aoctools::{parse_u64, read_file};
use std::cmp::max;

fn main() {
    let mut lines_with_ranges = true;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut counter_part_1 = 0;
    for line in read_file("input.txt") {
        let line = line.unwrap();
        if line.trim().is_empty() {
            lines_with_ranges = false;
        } else if lines_with_ranges {
            let [left, right] = line.split('-').collect::<Vec<&str>>().try_into().unwrap();
            let from = parse_u64(left);
            let to = parse_u64(right);
            ranges.push((from, to));
        } else {
            if ranges.iter().find(|(l,r)| *l <= parse_u64(&line) && *r >= parse_u64(&line)).is_some() {
                counter_part_1 += 1;
            }
        }
    }
    println!("Part 1: {}", counter_part_1);
    println!("Part 2: {}", merge(ranges).iter().map(|s| s.1 - s.0 + 1).sum::<u64>());
}

fn merge(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut iter = ranges.iter();
    let mut previous = *iter.next().unwrap();
    let mut result: Vec<(u64, u64)> = Vec::new();
    for current in iter {
        if (*current).0 <= previous.1 {
            previous.1 = max(previous.1, current.1);
        } else {
            result.push(previous);
            previous.0 = current.0;
            previous.1 = current.1;
        }
    }
    result.push(previous);
    result
}
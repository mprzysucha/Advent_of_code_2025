use std::fs::ReadDir;
use std::io::BufRead;
use aoctools::{parse_usize, read_file};

fn main() {
    let (presents, regions) = read_input("input.txt");
    println!("Presents:");
    for p in presents.iter() {
        println!("{:#?}", p);
    }
    println!("Regions:");
    let mut enough_cnt = 0;
    for r in regions {
        let mut num_of_hashes_in_one_region = 0;
        for (idx, qp) in r.quantity_of_each_shape.iter().enumerate() {
            println!("{} {}", idx, *qp);
            if *qp > 0 {
                let mut num_of_hashes = 0;
                for line in presents[idx].lines.iter() {
                    num_of_hashes += line.chars().filter(|c| *c == '#').count();
                }
                num_of_hashes_in_one_region += (*qp) * num_of_hashes;
            }
        }
        println!("{:#?}\n{}\n*****\n", r, num_of_hashes_in_one_region);
        println!("Region size: {}", r.length * r.width);
        let enough = num_of_hashes_in_one_region <= r.length * r.width;
        println!("Enough: {}", enough);
        if enough {
            enough_cnt += 1;
        }
    }
    println!("Part 1: {}", enough_cnt);
}

#[derive(Debug)]
struct Present {
    lines: Vec<String>
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    quantity_of_each_shape: Vec<usize>,
}

fn read_input(file_name: &str) -> (Vec<Present>, Vec<Region>) {
    let mut presents = Vec::<Present>::new();
    let mut regions = Vec::<Region>::new();
    let mut in_present = false;
    let mut next_present_lines = Vec::new();
    for line in read_file(file_name) {
        let line = line.unwrap();
        if in_present {
            if line.trim().is_empty() {
                in_present = false;
            } else {
                next_present_lines.push(line.clone());
            }
        }
        if line.ends_with(":") {
            in_present = true;
            if !next_present_lines.is_empty() {
                presents.push(Present { lines: next_present_lines });
                next_present_lines = Vec::new();
            }
        }
        if !line.trim().ends_with(":") && line.contains(":") && line.contains("x") {
            if !next_present_lines.is_empty() {
                presents.push(Present { lines: next_present_lines });
                next_present_lines = Vec::new();
            }
            let splits = line.split(": ").collect::<Vec<&str>>();
            let dimensions = splits[0].split("x").map(|x| parse_usize(x)).collect::<Vec<usize>>();
            let required_presents = splits[1].split(" ").map(|x| parse_usize(x)).collect::<Vec<usize>>();
            regions.push(Region {
                width: dimensions[0],
                length: dimensions[1],
                quantity_of_each_shape: required_presents,
            });
        }
    }
    (presents, regions)
}
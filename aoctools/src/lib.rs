use std::fs;
use std::fs::File;
use std::io::{ BufReader, Lines, prelude::* };
use std::str::FromStr;

pub fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file_name = String::from(file_name);
    let file = File::open(file_name).expect("Can't read a file");
    BufReader::new(file).lines()
}

pub fn read_file_and_process<P>(file_name: &str, processor: P)
where P: Fn(&str) -> () {
    for line_res in read_file(file_name) {
        match line_res {
            Ok(line) => processor(line.as_str()),
            Err(e) => panic!("Problem when reading a file: {:?}", e),
        }
    }
}

pub fn read_whole_file(file_name: &str) -> String {
    if let Ok(content) = fs::read_to_string(file_name) {
        return content;
    } else {
        panic!("Error reading {}", file_name);
    }
}

pub fn parse(s: &str) -> u32 {
    match s.parse::<u32>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}

pub fn char_to_int(ch: char) -> usize {
    ch as usize - 48
}

pub fn parse2<T: FromStr>(s: &str) -> T {
    match s.parse::<T>() {
        Ok(num) => num,
        Err(_) => panic!("Error parsing number: {}", s),
    }
}

pub fn parse_usize(s: &str) -> usize {
    match s.parse::<usize>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}

pub fn parse_i32(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}

pub fn parse_u64(s: &str) -> u64 {
    match s.parse::<u64>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}

pub fn parse_i64(s: &str) -> i64 {
    match s.parse::<i64>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}

pub fn parse_i128(s: &str) -> i128 {
    match s.parse::<i128>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}

pub fn parse_f64(s: &str) -> f64 {
    match s.parse::<f64>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}
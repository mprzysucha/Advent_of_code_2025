use aoctools::read_file;
use std::collections::HashMap;

fn main() {
    // let paths = read_paths("test_input.txt");
    let paths = read_paths("input.txt");
    let num_of_paths = find_paths_1(&paths, "you", "out", 0);
    println!("Part 1: {}", num_of_paths);

    // let paths_2 = read_paths("test_input_2.txt");
    let paths_2 = read_paths("input.txt");

    let mut path = Path::new(paths_2.clone(), "svr".to_string(), "out".to_string());
    let part_2_solution = path.find();
    println!("Part 2: {:?}",path.find());
}

fn find_paths_1(paths: &HashMap<String, Vec<String>>, start: &str, end: &str, mut num_of_paths: usize) -> usize {
    // println!("start: {}", start);
    for next in paths.get(start).unwrap() {
        if next == end {
            return num_of_paths + 1;
        }
        num_of_paths = find_paths_1(paths, next, end, num_of_paths)
    }
    num_of_paths
}

struct Path {
    paths: HashMap<String, Vec<String>>,
    start: String,
    end: String,
    cache: HashMap<(String, bool, bool), usize>,
}

impl Path {
    pub fn new(paths: HashMap<String, Vec<String>>, start: String, end: String) -> Self {
        Self { paths, start, end, cache: HashMap::<(String, bool, bool), usize>::new() }
    }

    fn find(&mut self) -> usize {
        self.find_internal(self.start.clone(), false, false)
    }

    fn find_internal(&mut self, start: String, mut fft: bool, mut dac: bool) -> usize {
        if self.cache.contains_key(&(start.clone(), fft, dac)) {
            return *self.cache.get(&(start.clone(), fft, dac)).unwrap();
        }
        if start == self.end {
            if fft && dac {
                return 1;
            }
            return 0;
        }
        if start == "fft" {
            fft = true;
        }
        if start == "dac" {
            dac = true;
        }
        let mut sum = 0;

        let nexts = match self.paths.get(&start) {
            Some(n) => { n }
            None => { &Vec::<String>::new() }
        };

        for n in nexts.clone() {
            sum += self.find_internal(n.clone(), fft, dac);
        }
        self.cache.insert((start.clone(), fft, dac), sum);
        sum
    }

}

fn read_paths(file_name: &str) -> HashMap<String, Vec<String>> {
    let mut paths = HashMap::<String, Vec<String>>::new();
    for line in read_file(file_name) {
        let line = line.unwrap();
        let split = line.split(": ").collect::<Vec<&str>>();
        if split.len() >= 2 {
            let key = split[0].to_string();
            let values: Vec<String> = split[1].split(" ").map(|s| s.to_string()).collect();
            paths.insert(key, values);
        }
    }
    paths
}
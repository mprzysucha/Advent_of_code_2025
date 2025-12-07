use std::collections::{HashMap, HashSet};
use aoctools::read_file;

fn main() {
    let file_name = "test_input.txt";
    let (map, _width, height) = create_a_map(file_name);
    let mut tachyon_beams_indices: HashSet<usize> = find_positions(&map, 'S', 0);
    let mut split_counter = 0;
    for h in 1..height {
        let mut next_tachyon_beams_indices = HashSet::<usize>::new();
        for tb in &tachyon_beams_indices {
            if map[h][*tb] == '^' {
                if tb - 1 > 0 {
                    next_tachyon_beams_indices.insert(tb - 1);
                }
                if tb + 1 < height {
                    next_tachyon_beams_indices.insert(tb + 1);
                }
                split_counter += 1;
            } else if map[h][*tb] == '.' {
                next_tachyon_beams_indices.insert(tb + 0);
            }
        }
        if next_tachyon_beams_indices.len() > 0 {
            tachyon_beams_indices = next_tachyon_beams_indices;
        }
    }
    println!("Part 1: {}", split_counter);

    let mut timelines_counter = 1_u64;
    let mut quantum_tachyon_beams_indices: HashMap<usize, u64> = find_quantum_positions(&map, 'S', 0);
    for h in 1..height {
        let mut next_quantum_tachyon_beams_indices = HashMap::<usize, u64>::new();
        for (&tb, &cnt) in &quantum_tachyon_beams_indices {
            let mut keys = Vec::<usize>::new();
            if map[h][tb] == '^' {
                keys.push(tb - 1);
                keys.push(tb + 1);
                timelines_counter += cnt;
            } else if map[h][tb] == '.' {
                keys.push(tb);
            }
            for k in keys {
                let new_val = match next_quantum_tachyon_beams_indices.remove(&k) {
                    Some(old) => old + cnt,
                    None => cnt,
                };
                next_quantum_tachyon_beams_indices.insert(k, new_val);
            }
        }
        if next_quantum_tachyon_beams_indices.len() > 0 {
            quantum_tachyon_beams_indices = next_quantum_tachyon_beams_indices;
        }
    }
    println!("Part 2: {}", timelines_counter);
}

fn create_a_map(file_name: &str) -> (Vec<Vec<char>>, usize, usize) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in read_file(file_name) {
        let line = line.unwrap();
        width = line.len();
        height += 1;
        map.push(line.chars().collect());
    }
    (map, width, height)
}

fn find_positions(map: &Vec<Vec<char>>, ch: char, row_num: usize) -> HashSet<usize> {
    let mut ch_indices = HashSet::<usize>::new();
    for (i, c) in map[row_num].iter().enumerate() {
        if *c == ch {
            ch_indices.insert(i);
        }
    }
    ch_indices
}

fn find_quantum_positions(map: &Vec<Vec<char>>, ch: char, row_num: usize) -> HashMap<usize, u64> {
    let mut ch_indices = HashMap::<usize, u64>::new();
    for (i, c) in map[row_num].iter().enumerate() {
        if *c == ch {
            ch_indices.insert(i, 1_u64);
        }
    }
    ch_indices
}
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use aoctools::read_file;

fn main() {
    let file_name = "input.txt";
    let take_max = 1000;
    let circuits = read_circuits(file_name);
    let number_of_boxes = circuits.len();
    let mut distances = HashMap::<(usize, usize), f64>::new();
    for i in 0..circuits.len() {
        for j in i+1..circuits.len() {
            distances.insert((min(i, j), max(i, j)), distance(circuits[i], circuits[j]));
        }
    }
    let mut sorted_distances = distances.into_iter().collect::<Vec<((usize, usize), f64)>>();
    sorted_distances.sort_by(|((_, _), d1), ((_, _), d2)| d1.partial_cmp(d2).unwrap());
    let top_of_sorted_distances = sorted_distances.clone().into_iter().take(take_max).collect::<Vec<((usize, usize), f64)>>();
    let mut sets = Vec::<HashSet<usize>>::new();
    for (idx, d) in top_of_sorted_distances.iter().enumerate() {
        match sets.iter().position(|set| set.contains(&d.0.0)) {
            Some(p1) => {
                match sets.iter().position(|set| set.contains(&d.0.1)) {
                    Some(p2) => {
                        if p1 != p2 {
                            for e in sets[p2].clone() {
                                sets[p1].insert(e);
                            }
                            sets[p2].clear();
                        }
                    }
                    None => {
                        sets[p1].insert(d.0.0);
                        sets[p1].insert(d.0.1);
                    }
                }

            }
            None => {
                match sets.iter().position(|set| set.contains(&d.0.1)) {
                    Some(p2) => {
                        sets[p2].insert(d.0.0);
                        sets[p2].insert(d.0.1);
                    }
                    None => {
                        let mut hs = HashSet::<usize>::new();
                        hs.insert(d.0.0);
                        hs.insert(d.0.1);
                        sets.push(hs);
                    }
                }
            }
        }
    }
    let mut lens = sets.iter().map(|hs| hs.len()).collect::<Vec<usize>>();
    lens.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("Part 1: {:?}", lens[0] * lens[1] * lens[2]);

    let mut last1 = (0, 0, 0);
    let mut last2 = (0, 0, 0);
    for (idx, d) in sorted_distances.iter().enumerate() {
        match sets.iter().position(|set| set.contains(&d.0.0)) {
            Some(p1) => {
                match sets.iter().position(|set| set.contains(&d.0.1)) {
                    Some(p2) => {
                        if p1 != p2 {
                            for e in sets[p2].clone() {
                                sets[p1].insert(e);
                            }
                            sets[p2].clear();
                        }
                    }
                    None => {
                        sets[p1].insert(d.0.0);
                        sets[p1].insert(d.0.1);
                    }
                }

            }
            None => {
                match sets.iter().position(|set| set.contains(&d.0.1)) {
                    Some(p2) => {
                        sets[p2].insert(d.0.0);
                        sets[p2].insert(d.0.1);
                    }
                    None => {
                        let mut hs = HashSet::<usize>::new();
                        hs.insert(d.0.0);
                        hs.insert(d.0.1);
                        sets.push(hs);
                    }
                }
            }
        }
        if sets.iter().filter(|set| set.len() > 0).collect::<Vec<&HashSet<usize>>>().len() == 1 {
            if sets.iter().filter(|set| set.len() > 0).collect::<Vec<&HashSet<usize>>>()[0].iter().collect::<Vec<&usize>>().len() == number_of_boxes {
                last1 = circuits[d.0.0];
                last2 = circuits[d.0.1];
                break;
            }
        }
    }
    println!("Part 2: {}", last1.0 * last2.0);
}

fn read_circuits(file_name: &str) -> Vec<(usize, usize, usize)> {
    let mut circuits: Vec<(usize, usize, usize)> = Vec::new();
    for line in read_file(file_name) {
        let line = line.unwrap();
        let coords = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        circuits.push((coords[0], coords[1], coords[2]));
    }
    circuits
}

fn distance(c1: (usize, usize, usize), c2: (usize, usize, usize)) -> f64 {
    let mut first_dim_dist = max(c1.0, c2.0) - min(c1.0, c2.0);
    let mut second_dim_dist = max(c1.1, c2.1) - min(c1.1, c2.1);
    let mut third_dim_dist = max(c1.2, c2.2) - min(c1.2, c2.2);
    first_dim_dist = first_dim_dist * first_dim_dist;
    second_dim_dist = second_dim_dist * second_dim_dist;
    third_dim_dist = third_dim_dist * third_dim_dist;
    ((first_dim_dist + second_dim_dist + third_dim_dist) as f64).sqrt()
}

fn find_idx_of_the_closest(circuit: (usize, usize, usize), circuits: &Vec<(usize, usize, usize)>) -> usize {
    let mut the_shortest_distance = f64::MAX;
    let mut the_closest_circuit_idx = 0;
    for (idx, c) in circuits.into_iter().enumerate() {
        let d = distance(circuit, *c);
        if d < the_shortest_distance && d > 0_f64 {
            the_closest_circuit_idx = idx;
            the_shortest_distance = d;
        }
    }
    the_closest_circuit_idx
}
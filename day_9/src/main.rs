use aoctools::{parse2, read_file};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Instant;

fn point_inside_area(point: (usize, usize), coord1: (usize, usize), coord2: (usize, usize)) -> bool {
    point.0 < max(coord1.0, coord2.0) &&
        point.1 < max(coord1.1, coord2.1) &&
        point.0 > min(coord1.0, coord2.0) &&
        point.1 > min(coord1.1, coord2.1)
}

fn main() {
    let now = Instant::now();
    let coords = read_map("input.txt", 100000);
    let mut areas: HashMap<(usize, usize), u64> = HashMap::<(usize, usize), u64>::new();
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let curr_area = area(coords[i], coords[j]);
            if curr_area > max_area {
                max_area = curr_area;
            }
            areas.insert((min(i, j), max(i, j)), area(coords[i], coords[j]));
        }
    }
    println!("Part 1: {}", max_area);

    let mut sorted_areas = areas.into_iter().collect::<Vec<((usize, usize), u64)>>();
    sorted_areas.sort_by(|((_, _), a1), ((_, _), a2)| a2.cmp(a1));
    max_area = 0;
    'outer: for ((i, j), area) in sorted_areas {
        for x in 0..coords.len() {
            let take_x = x;
            let mut take_y = x + 1;
            if x == coords.len() - 1 {
                take_y = 0;
            }
            let p1_inside = point_inside_area(coords[take_x], coords[i], coords[j]);
            let p2_inside = point_inside_area(coords[take_y], coords[i], coords[j]);
            let point_between = ((max(coords[take_x].0, coords[take_y].0) + min(coords[take_x].0, coords[take_y].0)) / 2, (max(coords[take_x].1, coords[take_y].1) + min(coords[take_x].1, coords[take_y].1)) / 2);
            let point_between_inside = point_inside_area(point_between, coords[i], coords[j]);
            if p1_inside || p2_inside || point_between_inside {
                continue 'outer;
            }
        }
        println!("Part 2: {}", area);
        break;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn read_map(file_name: &str, size: usize) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for line in read_file(file_name) {
        let line = line.unwrap();
        let split = line.split(',').collect::<Vec<&str>>();
        let x = parse2::<usize>(split[0]);
        let y = parse2::<usize>(split[1]);
        coords.push((x, y));
    }
    coords
}

fn area(coord1: (usize, usize), coord2: (usize, usize)) -> u64 {
    ((max(coord1.0, coord2.0) - min(coord1.0, coord2.0) + 1) as u64) * ((max(coord1.1, coord2.1) - min(coord1.1, coord2.1) + 1) as u64)
}

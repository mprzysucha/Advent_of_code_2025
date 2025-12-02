use aoctools::read_whole_file;
use aoctools::parse_u64;

fn main() {
    let mut sum_of_invalid: u64 = 0;
    let mut sum_of_invalid_par_2: u64 = 0;
    for range in read_whole_file("test_input.txt").split(",") {
        let [x, y] = range.split("-").map(|x| parse_u64(&x)).collect::<Vec<u64>>().try_into().unwrap();
        for i in x..y+1 {
            if is_invalid(&i.to_string()) {
                sum_of_invalid += i;
            }
            if is_invalid_part_2(&i.to_string()) {
                sum_of_invalid_par_2 += i;
            }
        }
    }
    println!("Part 1: {}", sum_of_invalid);
    println!("Part 2: {}", sum_of_invalid_par_2);
}

fn is_invalid(num: &str) -> bool {
    if num.len() % 2 != 0 {
        return false;
    }
    &num[0..num.len()/2] == &num[num.len()/2..num.len()]
}

fn is_invalid_part_2(num: &str) -> bool {
    if num.len() < 2 {
        return false;
    }
    let mut max_size = num.len() / 2;
    if num.len() % 2 != 0 {
        max_size = num.len() / 3;
    }
    for slice_len in 1..max_size+1 {
        if num.len() % slice_len != 0 {
            continue;
        }
        if num[0..slice_len].repeat(num.len() / slice_len) == num {
            return true;
        }
    }
    false
}

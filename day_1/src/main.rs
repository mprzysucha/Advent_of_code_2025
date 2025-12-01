use aoctools::read_whole_file;

fn main() {
    let mut zero_counter = 0;
    let mut zero_counter_part_2 = 0;
    let mut position = 50;
    for line in read_whole_file("input1.txt").split("\n") {
        let mut sign = 1;
        if line.chars().nth(0).unwrap() == 'L' {
            sign = -1;
        }
        let prev_position = position;
        position += sign * line[1..].parse::<i32>().unwrap();
        if position < 0 {
            zero_counter_part_2 += 1 - (position + 1) / 100;
            position = 99 + (position + 1) % 100;
            if prev_position == 0 {
                zero_counter_part_2 -= 1;
            }
        }
        if position >= 100 {
            zero_counter_part_2 += (position - 1) / 100;
            position %= 100;
        }
        if position == 0 {
            zero_counter += 1;
            zero_counter_part_2 += 1;
        }
    }
    println!("Part 1: {}", zero_counter);
    println!("Part 2: {}", zero_counter_part_2);
}

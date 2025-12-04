use aoctools::read_file;

fn main() {
    println!("Part 1: {}", solve(2));
    println!("Part 2: {}", solve(12));
}
fn solve(key_len: usize) -> u64 {
    let mut sum: u64 = 0;
    for line in read_file("input.txt") {
        let line = line.unwrap();
        sum += solve_line(key_len, &line);
    }
    sum
}

fn solve_line(key_len: usize, line: &str) -> u64 {
    // println!("\nLine: {}", line);
    let mut nums = Vec::new();
    for _ in 0..key_len {
        nums.push(0);
    }
    for (idx, ch) in line.chars().enumerate() {
        let as_num = ch as usize - 48;
        for i in 0..key_len {
            if line.len() - idx >= key_len - i && as_num > nums[i] {
                nums[i] = as_num;
                for j in i+1..key_len {
                    nums[j] = 0;
                }
                break;
            }
        }
    }
    let mut max: u64 = 0;
    for i in (0..key_len).rev() {
        max += 10_u64.pow(i as u32) * (nums[nums.len() - i - 1] as u64)
    }
    max
}

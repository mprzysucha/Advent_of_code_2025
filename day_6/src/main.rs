use aoctools::{char_to_int, read_file};

fn main() {
    // Part 2 only (Part 1 done in Excel)
    let mut sum_part_2 = 0;
    let file_name = "test_input.txt";
    let (mut numbers, break_idxs) = fetch_numbers_and_break_indices(file_name);
    for line in read_file(file_name) {
        let line = line.unwrap();
        let mut part_cnt = 0;
        for (ch_idx, ch) in line.chars().enumerate() {
            if break_idxs.contains(&ch_idx) {
                part_cnt += 1;
                continue;
            }
            if line.contains("+") || line.contains("*") {
                let chars_with_signs = line.chars().collect::<Vec<char>>();
                let mut current_sign = chars_with_signs[0];
                let mut block_sum = 0;
                let mut block_product = 1;
                part_cnt = 0;
                for (ch_idx, ch) in line.chars().enumerate() {
                    if ch == '+' || ch == '*' {
                        current_sign = ch;
                    }
                    if break_idxs.contains(&ch_idx) {
                        if block_product > 1 {
                            sum_part_2 += block_product;
                        } else {
                            sum_part_2 += block_sum;
                        }
                        block_sum = 0;
                        block_product = 1;
                        part_cnt += 1;
                        continue;
                    }
                    if current_sign == '+' {
                        block_sum += numbers[ch_idx - part_cnt];
                    } else if current_sign == '*' {
                        block_product *= numbers[ch_idx - part_cnt];
                    }
                    if ch_idx == line.len() - 1 {
                        if block_product > 1 {
                            sum_part_2 += block_product;
                        } else {
                            sum_part_2 += block_sum;
                        }
                    }
                }
                break;
            }
            if ch != ' ' {
                numbers[ch_idx - part_cnt] = 10 * numbers[ch_idx - part_cnt] + char_to_int(ch);
            }
        }
    }
    println!("Part 2: {}", sum_part_2);
}

fn fetch_numbers_and_break_indices(file_name: &str) -> (Vec<usize>, Vec<usize>) {
    let mut sign_idxs: Vec<usize> = Vec::new();
    let mut vertical_numbers_counter = 0;
    for line in read_file(file_name) {
        let line = line.unwrap();
        vertical_numbers_counter = line.len();
        if line.contains("+") || line.contains("*") {
            for (idx, ch) in line.chars().enumerate() {
                if ch == '+' || ch == '*' {
                    sign_idxs.push(idx);
                }
            }
            break;
        }
    }
    let mut break_idxs: Vec<usize> = Vec::new();
    for i in 1..sign_idxs.len() {
        break_idxs.push(sign_idxs[i] - 1);
    }
    vertical_numbers_counter -= break_idxs.len();
    let mut numbers: Vec<usize> = Vec::new();
    for _ in 0..vertical_numbers_counter {
        numbers.push(0);
    }
    (numbers, break_idxs)
}
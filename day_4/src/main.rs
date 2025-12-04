use aoctools::read_file;

fn main() {
    let file_name = "input.txt";
    let first_line = read_file(file_name).into_iter().next().unwrap().unwrap();
    let len = first_line.len();
    let mut prev_final_cnt = -1;
    let mut final_cnt = 0;
    let mut iter_num = 0;
    let mut map_of_lists = createMap(len, len);
    for (x_idx, line) in read_file(file_name).map(|l| l.unwrap()).enumerate() {
        for (y_idx, ch) in line.chars().enumerate() {
            map_of_lists[x_idx][y_idx] = ch;
        }
    }
    while final_cnt > prev_final_cnt {
        iter_num += 1;
        prev_final_cnt = final_cnt;
        let mut cnt = createCounter(len, len);
        for (x_idx, line) in map_of_lists.clone().into_iter().enumerate() {
            for (y_idx, ch) in line.into_iter().enumerate() {
                if ch == '@' {
                    for (adj_x, adj_y) in adjacents(x_idx, y_idx, len) {
                        cnt[adj_x][adj_y] += 1;
                    }
                } else if ch == '.' {
                    cnt[x_idx][y_idx] += 4;
                }
            }
        }
        for i in 0..len {
            for j in 0..len {
                if cnt[i][j] < 4 {
                    final_cnt += 1;
                    map_of_lists[i][j] = '.';
                }
            }
        }
        if iter_num == 1 {
            println!("Part 1: {}", final_cnt);
        }
    }
    println!("Part 2: {}", final_cnt);
}

fn adjacents(x_idx: usize, y_idx: usize, len: usize) -> Vec<(usize, usize)> {
    let mut adjacents: Vec<(usize, usize)> = Vec::new();
    for (x, y) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
        if x_idx as i32 + x >= 0 && x_idx as i32 + x < len as i32 && y_idx as i32 + y >= 0 && y_idx as i32 + y < len as i32 {
            adjacents.push(((x_idx as i32 + x) as usize, (y_idx as i32 + y) as usize));
        }
    }
    adjacents
}

fn createMap(x: usize, y: usize) -> Vec<Vec<char>> {
    let mut counter: Vec<Vec<char>> = Vec::new();
    for i in 0..x {
        counter.push(Vec::new());
        for j in 0..y {
            counter[i].push('.');
        }
    }
    counter
}

fn createCounter(x: usize, y: usize) -> Vec<Vec<usize>> {
    let mut counter: Vec<Vec<usize>> = Vec::new();
    for i in 0..x {
        counter.push(Vec::new());
        for j in 0..y {
            counter[i].push(0);
        }
    }
    counter
}
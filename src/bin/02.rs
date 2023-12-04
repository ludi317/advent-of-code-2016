advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (1, 1);
    let mut code = 0;
    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => {
                    if pos.0 > 0 {
                        pos.0 -= 1
                    }
                }
                'D' => {
                    if pos.0 < 2 {
                        pos.0 += 1
                    }
                }
                'L' => {
                    if pos.1 > 0 {
                        pos.1 -= 1
                    }
                }
                'R' => {
                    if pos.1 < 2 {
                        pos.1 += 1
                    }
                }
                _ => panic!("unrecognized character"),
            }
        }
        code = code * 10 + pos_to_code_part1(pos);
    }
    Some(code as u32)
}

fn pos_to_code_part1(pos: (isize, isize)) -> isize {
    let pad: [[isize; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    pad[pos.0 as usize][pos.1 as usize]
}



pub fn part_two(input: &str) -> Option<String> {
    let mut pos = (1, -1);
    let mut code: String = "".to_string();
    let mut prev_code = '5';
    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => {
                    if !(prev_code == '5'
                        || prev_code == '2'
                        || prev_code == '1'
                        || prev_code == '4'
                        || prev_code == '9')
                    {
                        pos.0 -= 1
                    }
                }
                'D' => {
                    if !(prev_code == '5'
                        || prev_code == 'A'
                        || prev_code == 'D'
                        || prev_code == 'C'
                        || prev_code == '9')
                    {
                        pos.0 += 1
                    }
                }
                'L' => {
                    if !(prev_code == '1'
                        || prev_code == '2'
                        || prev_code == '5'
                        || prev_code == 'A'
                        || prev_code == 'D')
                    {
                        pos.1 -= 1
                    }
                }
                'R' => {
                    if !(prev_code == '1'
                        || prev_code == '4'
                        || prev_code == '9'
                        || prev_code == 'C'
                        || prev_code == 'D')
                    {
                        pos.1 += 1
                    }
                }
                _ => panic!("unrecognized character"),
            }
            prev_code = pos_to_code_part_2(pos);
        }
        let cur_code = pos_to_code_part_2(pos);
        code.push(cur_code);
        prev_code = cur_code;
    }
    Some(code)
}

fn pos_to_code_part_2(pos: (isize, isize)) -> char {
    match pos {
        (-1, 1) => return '1',
        (1, -1) => return '5',
        (3, 1) => return 'D',
        (1, 3) => return '9',
        _ => (),
    }
    let pad: [[char; 3]; 3] = [['2', '3', '4'], ['6', '7', '8'], ['A', 'B', 'C']];
    pad[pos.0 as usize][pos.1 as usize]
}


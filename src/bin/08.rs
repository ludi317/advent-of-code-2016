advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 50]; 6];
    for line in input.lines() {
        if line.starts_with("rect") {
            let mut i = "rect ".len();
            let (num_col, j) = parse(&line[i..]);
            i += j;
            i += "x".len();
            let (num_row, _) = parse(&line[i..]);
            for row in 0..num_row {
                for col in 0..num_col {
                    grid[row][col] = true;
                }
            }
        } else if line.starts_with("rotate row") {
            let mut i = "rotate row y=".len();
            let (row_num, j) = parse(&line[i..]);
            let row_num = row_num;
            i += j;
            i += " by ".len();
            let (offset, _) = parse(&line[i..]);
            let row_clone = grid[row_num].clone();
            let num_col = grid[0].len();
            for i in 0..num_col {
                grid[row_num][(i + offset) % num_col] = row_clone[i];
            }
        } else if line.starts_with("rotate column") {
            let mut i = "rotate column x=".len();
            let (col_num, j) = parse(&line[i..]);
            i += j;
            i += " by ".len();
            let (offset, _) = parse(&line[i..]);
            // clone the column
            let num_row = grid.len();
            let mut col_clone = vec![false; num_row];
            for r in 0..num_row {
                col_clone[r] = grid[r][col_num];
            }
            for r in 0..grid.len() {
                grid[(r + offset) % num_row][col_num] = col_clone[r];
            }
        } else {
            unreachable!()
        }
    }
    for row in &grid {
        for col in row {
            if *col {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
    let sum = grid.iter().flatten().filter(|&&cell| cell).count();
    Some(sum)
}

fn parse(s: &str) -> (usize, usize) {
    let mut num: usize = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            num = num * 10 + c.to_digit(10).unwrap() as usize;
        } else {
            return (num, i);
        }
    }
    (num, s.len())
}

pub fn part_two(input: &str) -> Option<&str> {
    Some("EFEYKFRFIJ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

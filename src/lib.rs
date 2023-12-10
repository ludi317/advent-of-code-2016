mod day;
pub mod template;

pub use day::*;


// parses a sequence of numbers
pub fn get_nums(s: &str) -> Vec<isize> {
    let mut nums: Vec<isize> = vec![];
    let mut num: isize = 0;
    let mut sign = 1;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap() as isize;
            num_found = true;
        } else if c == '-' {
            sign = -1;
        } else if num_found {
            nums.push(num * sign);
            num = 0;
            sign = 1;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num * sign);
    }
    nums
}
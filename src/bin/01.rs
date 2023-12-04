use nom::{branch::alt, sequence::preceded, IResult};
use nom::{bytes::complete::tag, character::complete::digit1};
use nom::{
    combinator::{map, map_res},
    multi::separated_list1,
};
use std::collections::HashSet;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = directions(input).unwrap().1;
    let (dist, _) = walk(&dirs);
    Some(dist as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = directions(input).unwrap().1;
    let (_, visited) = walk(&dirs);
    Some(l1_dist(visited.unwrap()) as u32)
}

fn l1_dist(c: (isize, isize)) -> isize {
    c.0.abs() + c.1.abs()
}

fn walk(dirs: &Vec<Direction>) -> (isize, Option<(isize, isize)>) {
    let way: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let mut way_idx = 0;
    let mut pos = (0, 0);
    let mut pos_set: HashSet<(isize, isize)> = HashSet::new();
    pos_set.insert(pos);
    let mut visited: Option<(isize, isize)> = None;
    for dir in dirs {
        let (rotation, steps) = match dir {
            Direction::L(n) => (3, n),
            Direction::R(n) => (1, n),
        };
        way_idx = (way_idx + rotation) % 4;
        for _ in 1..=*steps {
            pos.0 += way[way_idx][0];
            pos.1 += way[way_idx][1];
            // println!("{:?}", pos);
            if visited.is_none() {
                if pos_set.contains(&pos) {
                    visited = Some(pos);
                } else {
                    pos_set.insert(pos);
                }
            }
        }
    }
    (l1_dist(pos), visited)
}


fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(
            preceded(tag("L"), map_res(digit1, |s: &str| s.parse::<isize>())),
            Direction::L,
        ),
        map(
            preceded(tag("R"), map_res(digit1, |s: &str| s.parse::<isize>())),
            Direction::R,
        ),
    ))(input)
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(tag(", "), direction)(input)
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    L(isize),
    R(isize),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

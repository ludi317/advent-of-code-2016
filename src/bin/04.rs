use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::delimited;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let rooms = rooms(input).unwrap().1;
    let sum = rooms
        .iter()
        .filter(|r| checksum(r.name) == r.checksum)
        .map(|r| r.sector_id)
        .sum::<usize>();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rooms = rooms(input).unwrap().1;

    let r: Vec<_> = rooms
        .iter()
        .filter(|r| checksum(r.name) == r.checksum)
        .filter(|r| cipher(r.name, r.sector_id).contains("north"))
        .collect();
    Some(r[0].sector_id)
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


#[derive(Debug, PartialEq)]
struct Room<'a> {
    name: &'a str,
    sector_id: usize,
    checksum: &'a str,
}

#[derive(Copy, Clone)]
struct Letter {
    c: char,
    n: usize,
}

fn cipher(name: &str, mut rot: usize) -> String {
    rot %= 26;
    let rot_u8 = rot as u8;
    let a = 'a' as u8;
    name.chars()
        .map(|c| {
            if c.is_alphabetic() {
                ((c as u8 - a + rot_u8) % 26 + a) as char
            } else {
                ' '
            }
        })
        .collect()
}

fn checksum(name: &str) -> String {
    // get the frequency of each letter in the name
    let mut freq = name.chars().filter(|c| c.is_alphabetic()).fold(
        [Letter { c: ' ', n: 0 }; 26],
        |mut freq, c| {
            let idx = c as usize - 'a' as usize;
            freq[idx].n += 1;
            freq[idx].c = c;
            freq
        },
    );
    // sort the letters by frequency, then alphabetically
    freq.sort_by(|a, b| b.n.cmp(&a.n).then(a.c.cmp(&b.c)));

    freq.iter().take(5).map(|l| l.c).collect()
}

fn room(input: &str) -> IResult<&str, Room> {
    let (input, name) = take_while(|c: char| c.is_alphabetic() || c == '-')(input)?;
    let (input, sector_id) = digit1(input)?;
    let (input, checksum) = delimited(tag("["), alpha1, tag("]"))(input)?;

    let sector_id = sector_id.parse::<usize>().unwrap();

    Ok((
        input,
        Room {
            name,
            sector_id,
            checksum,
        },
    ))
}

fn rooms(input: &str) -> IResult<&str, Vec<Room>> {
    separated_list1(line_ending, room)(input)
}

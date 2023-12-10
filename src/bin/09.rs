use advent_of_code::get_nums;
advent_of_code::solution!(9);
pub fn part_one(input: &str) -> Option<usize> {
    let bs: &[u8] = input.as_bytes();
    let mut i = 0;
    let mut s_len = 0;
    while i < bs.len() {
        if bs[i] == b'(' {
            let x_pos = input[(i + 1)..].find('x').unwrap() + i + 1;
            let close_paren_pos = input[(i + 1)..].find(')').unwrap() + i + 1;
            let rep_start = close_paren_pos + 1;
            let num_chars = get_nums(&input[i + 1..x_pos])[0] as usize;
            let repeats = get_nums(&input[x_pos + 1..close_paren_pos])[0] as usize;
            // println!("{} {}", num_chars, repeats );
            s_len += repeats * num_chars;
            // for _ in 0..repeats {
            //     s.push_str(&input[rep_start..rep_start+num_chars]);
            // }
            i = rep_start + num_chars;
        } else {
            // s.push(bs[i] as char);
            s_len += 1;
            i += 1;
        }
    }
    Some(s_len)
}

pub fn part_two(input: &str) -> Option<usize> {
    let bs: &[u8] = input.as_bytes();
    let mut i = 0;
    let mut s_len = 0;
    while i < bs.len() {
        if bs[i] == b'(' {
            let x_pos = input[(i + 1)..].find('x').unwrap() + i + 1;
            let close_paren_pos = input[(i + 1)..].find(')').unwrap() + i + 1;
            let rep_start = close_paren_pos + 1;
            let num_chars = get_nums(&input[i + 1..x_pos])[0] as usize;
            let repeats = get_nums(&input[x_pos + 1..close_paren_pos])[0] as usize;
            let snippet = &input[close_paren_pos + 1..close_paren_pos + 1 + num_chars];
            let inner_len = part_two(snippet);
            s_len += repeats * inner_len.unwrap();
            i = rep_start + num_chars;
        } else {
            s_len += 1;
            i += 1;
        }
    }
    Some(s_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("ADVENT"), Some(6));
        assert_eq!(part_one("A(1x5)BC"), Some(7));
        assert_eq!(part_one("(3x3)XYZ"), Some(9));
        assert_eq!(part_one("A(2x2)BCD(2x2)EFG"), Some(11));
        assert_eq!(part_one("(6x1)(1x3)A"), Some(6));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("(3x3)XYZ"), Some(9));
        assert_eq!(part_two("X(8x2)(3x3)ABCY"), Some(20));
        assert_eq!(part_two("(27x12)(20x12)(13x14)(7x10)(1x12)A"), Some(241920));
        assert_eq!(
            part_two("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            Some(445)
        );
    }
}

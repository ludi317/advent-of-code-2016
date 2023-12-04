advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<String> {
    Some(code_most(input))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(code_least(input))
}


fn code_least(s: &str) -> String {
    let line_length = s.lines().next().unwrap().chars().count();
    let mut freqs: Vec<Vec<usize>> = vec![vec![0; 26]; line_length];

    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            freqs[i][(c as u8 - b'a') as usize] += 1;
        }
    }

    let code: String = freqs
        .iter()
        .map(|frequencies| {
            frequencies
                .iter()
                .enumerate()
                .filter(|&(_, f)| f != &0usize)
                .min_by_key(|&(_, freq)| freq)
                .map(|(index, _)| (index as u8 + b'a') as char)
                .unwrap()
        })
        .collect();

    code
}

fn code_most(s: &str) -> String {
    let line_length = s.lines().next().unwrap().chars().count();
    let mut freqs: Vec<Vec<usize>> = vec![vec![0; 26]; line_length];

    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            freqs[i][(c as u8 - b'a') as usize] += 1;
        }
    }

    let code: String = freqs
        .iter()
        .map(|frequencies| {
            frequencies
                .iter()
                .enumerate()
                .max_by_key(|&(_, freq)| freq)
                .map(|(index, _)| (index as u8 + b'a') as char)
                .unwrap()
        })
        .collect();

    code
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

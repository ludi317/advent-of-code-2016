advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    Some(valid_triangle_count(input))
}


fn is_valid_triangle(v: &Vec<usize>) -> bool {
    return v[0] + v[1] > v[2] && v[0] + v[2] > v[1] && v[1] + v[2] > v[0];
}

fn valid_triangle_count(s: &str) -> usize {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect()
        })
        .filter(|v| is_valid_triangle(v))
        .count()
}


pub fn part_two(input: &str) -> Option<usize> {
    Some(valid_triangle_count_part2(input))
}

fn valid_triangle_count_part2(s: &str) -> usize {
    s.lines()
        .collect::<Vec<_>>() // Collect lines into a Vec to allow chunking
        .chunks(3) // Process three lines at a time
        .flat_map(|chunk| {
            // Create an iterator of tuples, each representing a column from the input
            (0..3).map(|i| {
                chunk
                    .iter()
                    .map(|line| {
                        line.split_whitespace()
                            .nth(i)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap()
                    })
                    .collect()
            })
        })
        .filter(|v| is_valid_triangle(v)) // Use is_valid_triangle to filter valid triangles
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;


}

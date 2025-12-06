use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;


pub fn run() -> (usize, usize) {
    let path = Path::new(BASE_DIR).join("real").join("day3.txt");
    let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .lines()
                            .map(|s| s.to_owned())
                            .collect();
    (pt1(&input), pt2(&input))
}

fn pt1(input: &[String]) -> usize {
    input.iter().map(|line| {
        line_max_joltage(&line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>(),1)
    }).sum()
}


fn line_max_joltage(line: &[usize], n: usize) -> usize {
    let (max_i,&max_val) = line[..line.len()-n].iter().enumerate().rev().max_by(|a,b| a.1.cmp(b.1)).unwrap();
    if n == 0 {
        return max_val;
    }

    // Happy path - find next max in a reduced set
    let next = line_max_joltage(&line[max_i+1..], n-1);
    max_val * (10 as usize).pow(n as u32) + next
}


fn pt2(input: &[String]) -> usize {
    input.iter().map(|line| {
        line_max_joltage(&line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>(),11)
    }).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
    let path = Path::new(BASE_DIR).join("test").join("day3.txt");
    let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .lines()
                            .map(|s| s.to_owned())
                            .collect();

    assert_eq!(pt1(&input), 357);
    }

    #[test]
    fn test_part2() {
    let path = Path::new(BASE_DIR).join("test").join("day3.txt");
    let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .lines()
                            .map(|s| s.to_owned())
                            .collect();

    assert_eq!(pt2(&input), 3121910778619);
    }
}
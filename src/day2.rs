use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use fancy_regex::Regex;


pub fn run() -> (usize, usize) {
    let path = Path::new(BASE_DIR).join("real").join("day2.txt");
    let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .split(',')
                            .map(|s| s.to_owned())
                            .collect();
    (pt1(&input), pt2(&input))
}

fn pt1(input: &[String]) -> usize {
    let re = Regex::new(r"\b(\d{1,})\1{1}\b").unwrap();
    input.iter().fold(0,|acc,s|{
        let range = build_range(s);
        acc + match_range(&range, &re)
    })
}

fn pt2(input: &[String]) -> usize {
    let re = Regex::new(r"\b(\d{1,})\1{1,}\b").unwrap();
    input.iter().fold(0,|acc,s|{
        let range = build_range(s);
        acc + match_range(&range, &re)
    })
}

fn build_range(s: &str) -> Vec<usize> {
    let mut parts = s.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    (start..=end).collect()
}

fn match_range(range: &[usize], re: &Regex) -> usize {
    range.iter().fold(0, |acc, &num| if match_num(num,re) { acc + num } else { acc })
}

fn match_num(num: usize, re: &Regex) -> bool {
    let s = num.to_string();
    re.is_match(&s).expect("Failed to run regex")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let path = Path::new(BASE_DIR).join("test").join("day2.txt");
        let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .split(',')
                            .map(|s| s.to_owned())
                            .collect();
        assert_eq!(pt1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let path = Path::new(BASE_DIR).join("test").join("day2.txt");
        let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .split(',')
                            .map(|s| s.to_owned())
                            .collect();
        assert_eq!(pt2(&input), 4174379265);
    }
}
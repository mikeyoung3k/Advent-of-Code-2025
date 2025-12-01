use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;

pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day1.txt");
    let input: Vec<String> = read_to_string(path)
                                    .expect("Error reading file")
                                    .lines()
                                    .map(|s| s.to_owned())
                                    .collect::<Vec<_>>();
    (pt1(&input,50),pt2(&input,50))
}

fn pt1(input: &[String],mut pos: isize) -> isize {
    let mut count = 0;
    for s in input {
        let val = parse_string(s);
        pos += val;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn parse_string(s: &str) -> isize {
    let val = s[1..].parse::<isize>().unwrap();
    match s.chars().nth(0).unwrap() {
        'L' => -val,
        'R' => val,
        _ => panic!("Invalid direction: {}", s),
    }
}

fn pt2(input: &[String], mut pos: isize) -> isize {
        let mut count = 0;
    for s in input {
        let val = parse_string(s);
        if val < 0 {
            count += -val/100;

            if pos != 0 && val.abs()%100 >= pos {
                count += 1;
            }
        } else {
            count += (val)/100;

            if pos != 0 && val.abs()%100 >= 100-pos {
                count += 1;
            }
        }
        pos += val;
        pos = pos.rem_euclid(100);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt1() {
        let path = Path::new(BASE_DIR).join("test").join("day1.txt");
        let input: Vec<String> = read_to_string(path)
                                    .expect("Error reading file")
                                    .lines()
                                    .map(|s| s.to_owned())
                                    .collect::<Vec<_>>();
        assert_eq!(pt1(&input,50), 3);
    }
    #[test]
    fn test_pt2() {
        let path = Path::new(BASE_DIR).join("test").join("day1.txt");
        let input: Vec<String> = read_to_string(path)
                                    .expect("Error reading file")
                                    .lines()
                                    .map(|s| s.to_owned())
                                    .collect::<Vec<_>>();
        assert_eq!(pt2(&input,50), 6);
    }

    #[test]
    fn test_parse_string() {
        let a = "L4".to_string();
        let b = "R2".to_string();

        assert_eq!(parse_string(&a), -4);
        assert_eq!(parse_string(&b), 2);
    }
}
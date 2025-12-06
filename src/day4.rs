use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use std::collections::BTreeSet;


pub fn run() -> (usize, usize) {
    let path = Path::new(BASE_DIR).join("real").join("day4.txt");
    let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .lines()
                            .map(|s| s.to_owned())
                            .collect();
    (pt1(&input), pt2(input))
}

fn pt1(input: &[String]) -> usize {
    let set = build_set(input);
    set.iter().map(|&pos| if find_neighbors(&set, pos) < 4 {1} else {0}).sum()
}

fn find_neighbors(set: &BTreeSet<(isize,isize)>, pos: (isize,isize)) -> usize {
    let mut count = 0;
    for row in pos.0-1 ..= pos.0+1 {
        for col in pos.1-1 ..= pos.1+1 {
            if row ==  pos.0 && col == pos.1 {
                continue;
            }
            if set.contains(&(row, col)) {
                count +=1;
            }
        }
    }
    count
}

fn build_set(data: &[String]) -> BTreeSet<(isize,isize)> {
    let mut set = BTreeSet::new();
    for (row,line) in data.iter().enumerate() {
        for (col,ch) in line.chars().enumerate() {
            if ch == '@' {
                set.insert((row as isize, col as isize));
            }
        }
    }
    set
}




fn pt2(mut input: Vec<String>) -> usize {
    let mut count = 0;
    let mut set = build_set(&input);
    loop {
        let mut loop_count = 0;
        for &pos in set.iter() {
            if find_neighbors(&set, pos) < 4 {
                loop_count += 1;
                input.get_mut(pos.0 as usize).expect("Bad row").replace_range(pos.1 as usize..pos.1 as usize+1, ".");
            }
        };
        count += loop_count;
        if loop_count == 0 {
            break;
        }
        set = build_set(&input);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
    let path = Path::new(BASE_DIR).join("test").join("day4.txt");
    let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .lines()
                            .map(|s| s.to_owned())
                            .collect();

    assert_eq!(pt1(&input), 13);
    }

    #[test]
    fn test_part2() {
    let path = Path::new(BASE_DIR).join("test").join("day4.txt");
    let input: Vec<String> = read_to_string(path)
                            .expect("Error reading file")
                            .lines()
                            .map(|s| s.to_owned())
                            .collect();

    assert_eq!(pt2(input), 43);
    }
}
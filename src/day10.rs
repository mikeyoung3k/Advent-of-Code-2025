use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use itertools::Itertools;


pub fn run() -> (usize,usize) {
    let path = Path::new(BASE_DIR).join("real").join("day10.txt");
    let data = read_data(&path);
    (pt1(&data),0)
}

#[derive(Default,Debug)]
struct Line {
    target: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<usize>
}

type Button = Vec<usize>;

fn read_data(path: &Path) -> Vec<Line>{
    read_to_string(path).unwrap().lines().map(|line| {
        parse_line(line)
    }).collect::<Vec<Line>>()
}

fn parse_line(line: &str) -> Line {
    let mut items = line.split_whitespace();
    let mut line = Line::default();
    while let Some(mut item) = items.next() {
        if item.starts_with('[') {
            item = item.strip_prefix('[').unwrap();
            item = item.strip_suffix(']').unwrap();
            line.target = item.chars().map(|c|{
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Bad target symbol")
                }
            }).collect::<Vec<bool>>();
        } else if item.starts_with('(') {
            item = item.strip_prefix('(').unwrap();
            item = item.strip_suffix(')').unwrap();
            line.buttons.push(item.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>());
        } else if item.starts_with('{') {
            item = item.strip_prefix('{').unwrap();
            item = item.strip_suffix('}').unwrap();
            line.joltages = item.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        } else {
            panic!("Invalid item")
        }
    }
    line
}

fn pt1(data: &Vec<Line>) -> usize {
    data.iter().map(|line| {
        permute_line(line)
    }).sum()
}

fn permute_line(line: &Line) -> usize {
    for k in 1..line.buttons.len() {
        for combo in line.buttons.iter().permutations(k) {
            let num_pushes = combo.len();
            let mut state = vec![false; line.target.len()];
            for button in combo {
                for pos in button {
                    state[*pos] = !state[*pos];
                }   
                if state == line.target {
                    return num_pushes;
                }
            }
        }
    }
    0
}

fn pt2(data: &Vec<Line>) -> usize {
    todo!();
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_pt1() {
        let path = Path::new(BASE_DIR).join("test").join("day10.txt");
        let data = read_data(&path);

        assert_eq!(pt1(&data),7);
    }
}
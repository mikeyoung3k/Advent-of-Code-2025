use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use std::collections::{HashMap,BTreeSet};


pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day7.txt");
    let (splitters,start, last_row) = read_input(&path);
    (pt1(&splitters,&start,&last_row), pt2(&splitters,&start,&last_row))
}

type Row = isize;
type Col = isize;

type Coord = (Row, Col);
type BeamPath = Vec<Coord>;

fn read_input(path: &Path) -> (HashMap<Row,BTreeSet<isize>>, (isize,isize),isize) {
    let mut splitters: HashMap<Row, BTreeSet<Col>> = HashMap::new();
    let mut start = (0, 0);
    let contents = read_to_string(path).expect("Error reading file");
    let mut last_row = 0;
    for (row,line) in contents.lines().enumerate() {
        let mut row_split = BTreeSet::new();
        for (col,char) in line.chars().enumerate() {
            if char == 'S' {
                start = (row as isize, col as isize);
            }
            if char == '^' {
                row_split.insert(col as isize);
            }
        }
        splitters.insert(row as isize, row_split);
        last_row = row;
    }
    (splitters,start, last_row as isize)
}

fn pt1(splitters: &HashMap<Row,BTreeSet<isize>>, start: &(isize,isize),last_row:&isize) -> isize {
    let mut active_beams = BTreeSet::new();
    let mut count = 0;
    active_beams.insert(start.1);
    for row in 0..=*last_row {
        if let Some(row_splitters) = splitters.get(&row) {
            for beam in active_beams.clone() {
                if row_splitters.contains(&beam) {
                    count += 1;
                    active_beams.insert(beam-1);
                    active_beams.insert(beam+1);
                    active_beams.remove(&beam);
                    if row_splitters.contains(&(beam-1)) {
                        active_beams.remove(&(beam-1));
                    }
                    if row_splitters.contains(&(beam+1)) {
                        active_beams.remove(&(beam+1));
                    }
                }
            }
        }
    }
    count
}

fn pt2(splitters: &HashMap<Row,BTreeSet<isize>>, start: &(isize,isize),last_row: &isize) -> isize {
    let mut active_beams: HashMap<isize, usize> = HashMap::new();
    active_beams.insert(start.1,1);
    for row in 0..=*last_row {
        if let Some(row_splitters) = splitters.get(&row) && row_splitters.len() > 0 {
            let mut next_beams = HashMap::new();
            for beam in active_beams.into_iter() {
                if row_splitters.contains(&beam.0) {
                    *next_beams.entry(beam.0-1).or_insert(0) += beam.1;
                    *next_beams.entry(beam.0+1).or_insert(0) += beam.1;
                } else {
                    *next_beams.entry(beam.0).or_insert(0) += beam.1;
                }
            }
            active_beams = next_beams;
        }
    }
    active_beams.values().sum::<usize>() as isize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt1() {
        let path = Path::new(BASE_DIR).join("test").join("day7.txt");
        let (splitters,start,last_row) = read_input(&path);
        assert_eq!(pt1(&splitters,&start,&last_row), 21);
    }

    #[test]
    fn test_pt2() {
        let path = Path::new(BASE_DIR).join("test").join("day7.txt");
        let (splitters,start,last_row) = read_input(&path);
        assert_eq!(pt2(&splitters,&start,&last_row), 40);
    }
}
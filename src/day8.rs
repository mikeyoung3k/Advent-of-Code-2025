use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use std::collections::{HashSet,BTreeMap};

pub fn run() -> (usize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day8.txt");
    let data = read_data(&path);
    (pt1(&data,1000), pt2(&data))
}

fn  read_data(path: &Path) -> Vec<Point> {
    read_to_string(path).expect("Error reading file")
    .lines()
    .map(|line| {
            let arr = line.split(',')
                .map(|n| n.parse::<isize>().unwrap() )
                .collect::<Vec<_>>();
            Point::from_slice(&arr)
    })
    .collect()
}

#[derive(Debug, PartialEq,Eq, Hash)]
struct Point(isize,isize,isize);

impl Point {
    fn from_slice(arr: &[isize]) -> Point {
        Point(arr[0], arr[1], arr[2])
    }
    fn distance(&self, other: &Point) -> isize {
        (self.0 - other.0).pow(2) +
        (self.1 - other.1).pow(2) +
        (self.2 - other.2).pow(2)
    }
}

fn pt1(points: &[Point],max_conns: usize) -> usize {
    let mut all_sets: Vec<HashSet<&Point>> = Vec::new();
    let mut distances = Vec::new();
    for (i,point) in points.iter().enumerate() {
        for other in points.iter().skip(i+1) {
            let dist = point.distance(other);
            if dist > 0 {
                distances.push((dist,point,other));
            }
        }
    }
    distances.sort_by(|a, b| a.0.cmp(&b.0));
    
    distances.truncate(max_conns);
    for (_,point,other) in distances {
        if let Some(_) = all_sets.iter().find(|s| s.contains(point) && s.contains(other)) {
            continue
        }
        let found_point = all_sets.iter().find(|s| s.contains(point)).is_some();
        let found_other = all_sets.iter().find(|s| s.contains(other)).is_some();
        if !found_point && !found_other {
            // Add a new set
            let mut new_set = HashSet::new();
            new_set.insert(point);
            new_set.insert(other);
            all_sets.push(new_set);
        } else if found_point && found_other {
            // Merge sets
            let other = all_sets.iter_mut().find(|s| s.contains(other)).unwrap().drain().collect::<Vec<_>>();
            let point = all_sets.iter_mut().find(|s| s.contains(point)).unwrap();
            point.extend(other);
        } else if found_other {
            // Put point in other set
            let other = all_sets.iter_mut().find(|s| s.contains(other)).unwrap();
            other.insert(point);
        } else {
            // Put other in point set
            let point = all_sets.iter_mut().find(|s| s.contains(point)).unwrap();
            point.insert(other);
        }

    }

    let mut sets = all_sets.iter().map(|s| s.len()).collect::<Vec<_>>();
    sets.sort_by(|a, b| b.cmp(&a));
    sets.truncate(3);
    sets.iter().product()
}

fn pt2(points: &[Point]) -> isize {
    let mut all_sets: Vec<HashSet<&Point>> = Vec::new();
    let mut distances = Vec::new();
    for (i,point) in points.iter().enumerate() {
        for other in points.iter().skip(i+1) {
            let dist = point.distance(other);
            if dist > 0 {
                distances.push((dist,point,other));
            }
        }
    }
    distances.sort_by(|a, b| a.0.cmp(&b.0));
    
    for (_,point,other) in distances {
        if let Some(_) = all_sets.iter().find(|s| s.contains(point) && s.contains(other)) {
            continue
        }
        let found_point = all_sets.iter().find(|s| s.contains(point)).is_some();
        let found_other = all_sets.iter().find(|s| s.contains(other)).is_some();
        if !found_point && !found_other {
            // Add a new set
            let mut new_set = HashSet::new();
            new_set.insert(point);
            new_set.insert(other);
            all_sets.push(new_set);
        } else if found_point && found_other {
            // Merge sets
            let other = all_sets.iter_mut().find(|s| s.contains(other)).unwrap().drain().collect::<Vec<_>>();
            let point = all_sets.iter_mut().find(|s| s.contains(point)).unwrap();
            point.extend(other);
        } else if found_other {
            // Put point in other set
            let other = all_sets.iter_mut().find(|s| s.contains(other)).unwrap();
            other.insert(point);
        } else {
            // Put other in point set
            let point = all_sets.iter_mut().find(|s| s.contains(point)).unwrap();
            point.insert(other);
        }
        if all_sets.iter().any(|s| s.len() == points.len()) {
            return point.0*other.0
        }

    }

    // let mut sets = all_sets.iter().map(|s| s.len()).collect::<Vec<_>>();
    // sets.sort_by(|a, b| b.cmp(&a));
    // sets.truncate(3);
    // sets.iter().product()
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let path = Path::new(BASE_DIR).join("test").join("day8.txt");
        let data= read_data(&path);

        assert_eq!(pt1(&data,10), 40);
    }

    #[test]
    fn test_part2() {
        let path = Path::new(BASE_DIR).join("test").join("day8.txt");
        let data= read_data(&path);

        assert_eq!(pt2(&data), 25272);
    }
}
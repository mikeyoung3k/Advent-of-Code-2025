use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use std::collections::BTreeSet;


pub fn run() -> (usize, usize) {
    let path = Path::new(BASE_DIR).join("real").join("day5.txt");
    let (fresh, avail) = read_input(&path);
    (pt1(&fresh,&avail), pt2(&fresh))
}

fn read_input(path: &Path) -> (Vec<String>,Vec<String>) {
    let mut fresh_items = Vec::new();
    let mut avail_items = Vec::new();
    let input = read_to_string(path).expect("Error reading file");
    let mut fresh = true;
    for line in input.lines() {
        if line.is_empty() {
            fresh = false;
            continue
        }
        if fresh {
            fresh_items.push(line.to_owned());
        } else {
            avail_items.push(line.to_owned());
        }
    }
    (fresh_items, avail_items)
}

fn pt1(fresh: &[String],avail: &[String]) -> usize {
    let ranges = parse_fresh_ranges(fresh);
    avail.iter().filter(|&item| {
        ranges.iter().any(|range| range.contains(&item.parse::<usize>().unwrap()))
    }).count()
}

fn pt2(fresh: &[String]) -> usize {
    let mut comb_ranges = Vec::new();
    let mut all_ranges: Vec<(usize,usize)> = fresh.iter().map(|line| {
        let mut parts = line.split('-');
        let first = parts.next().unwrap().parse::<usize>().unwrap();
        let second = parts.next().unwrap().parse::<usize>().unwrap();
        (first,second)
    })
    .collect();
    all_ranges.sort_by(|a,b| a.0.cmp(&b.0));
    let mut range_iter = all_ranges.into_iter();
    comb_ranges.push(range_iter.next().unwrap());
    for r in range_iter {
        let mut last_range = comb_ranges.pop().unwrap();
        if r.0 < last_range.0 {
            panic!();
        }
        if r.0 <= last_range.1 {
            if r.1 <= last_range.1 {
                comb_ranges.push(last_range);
                continue
            }
            if r.1 > last_range.1 {
                last_range.1 = r.1;
                comb_ranges.push(last_range);
            }
        } else {            
            comb_ranges.push(last_range);
            comb_ranges.push(r);
        }

    }
    comb_ranges.iter().fold(0,|acc,r| {
        acc + r.1 + 1 - r.0
    })
}

fn parse_fresh_ranges(fresh_items : &[String]) -> Vec<std::ops::RangeInclusive<usize>> {
    fresh_items.iter().map(|line| {
        let mut parts = line.split('-');
        let first = parts.next().unwrap().parse::<usize>().unwrap();
        let second = parts.next().unwrap().parse::<usize>().unwrap();
        first..=second
    })
    .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let (fresh, avail) = read_input(&Path::new(BASE_DIR).join("test").join("day5.txt"));
        assert_eq!(pt1(&fresh, &avail), 3);
    }

    #[test]
    fn test_part2() {
        let (fresh, avail) = read_input(&Path::new(BASE_DIR).join("test").join("day5.txt"));
        assert_eq!(pt2(&fresh), 14);
    }
}
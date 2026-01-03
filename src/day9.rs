use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;

pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day9.txt");
    let data = read_data(&path);
    (pt1(&data),pt2(&data))
}

#[derive(Debug,Clone,PartialEq)]
struct Point(isize,isize);
impl Point {
    fn area(&self, other: &Self) -> isize {
        ((self.0-other.0).abs()+1) *
        ((self.1-other.1 ).abs()+1)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Line{
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn crosses(&self, other: &Self) -> bool {
        if self.is_vertical() == other.is_vertical() {
            return false;
        }

        if self.is_vertical() {
            if other.start.0.min(other.end.0) < self.start.0 &&
               other.start.0.max(other.end.0) > self.end.0 &&
               other.start.1 > self.start.1.min(self.end.1) &&
               other.start.1 < self.start.1.max(self.end.1) {
                    return true;
               }
        } else {
            if other.start.1.min(other.end.1) < self.start.1 &&
               other.start.1.max(other.end.1) > self.end.1 &&
               other.start.0 > self.start.0.min(self.end.0) &&
               other.start.0 < self.start.0.max(self.end.0) {
                    return true;
               }
        }
        false
    }
}

fn  read_data(path: &Path) -> Vec<Point> {
    read_to_string(path).expect("Error reading file")
    .lines()
    .map(|line|{
        let mut arr = line.split(',')
                    .map(|n| n.parse::<isize>().unwrap() );
                Point(arr.next().unwrap(),arr.next().unwrap())
    })
    .collect()
}

fn pt1(data: &Vec<Point>) -> isize {
    let mut max_area = 0;
    for point in data.iter() {
        for other in data.iter() {
            let area = point.area(other);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn pt2(data: &Vec<Point>) -> isize {
    let mut puzzle_lines = data.windows(2).map(|window|{
        Line{
            start: window.first().unwrap().clone(),
            end: window.last().unwrap().clone(),
        }
    })
    .collect::<Vec<_>>();

    puzzle_lines.push(Line { start: data.last().unwrap().clone(), end: data.first().unwrap().clone() });

    let mut max_area = 0;
    for point in data {
        for other_point in data {
            let lines = draw_lines(point,other_point);
            if lines.iter().any(|line| {
                puzzle_lines.iter().any(|pl| {
                    pl.crosses(&line)
                })
            }) {
                continue;
            }
            let lowest_point = Point(point.0.min(other_point.0), point.1.min(other_point.1));
            let highest_point = Point(point.0.max(other_point.0), point.1.max(other_point.1));
            if data.iter().any(|p| {
                lowest_point.0 < p.0 && p.0 < highest_point.0 &&
                lowest_point.1 < p.1 && p.1 < highest_point.1
            }) {
                continue
            }
            let area = point.area(other_point);
            // Ray cast each vertex of the rectangle to check it's inside the polygon
            if area > max_area {
                // println!("Allowed point pair: {:?}, {:?}",point,other_point);
                max_area = area;
            }

        }
    }
    max_area
    // Lines may not cross
    // No points may exist strictly in the rectangle

}

fn draw_lines(start: &Point, end: &Point) -> Vec<Line> {
    let mut lines = Vec::new();
    lines.push(Line{start: start.clone(), end: Point(end.0,start.1)});
    lines.push(Line{start: start.clone(), end: Point(start.0,end.1)});
    lines.push(Line{start: end.clone(), end: Point(end.0,start.1)});
    lines.push(Line{start: end.clone(), end: Point(start.0,end.1)});
    lines
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_pt1() {
        let path = Path::new(BASE_DIR).join("test").join("day9.txt");
        let data = read_data(&path);

        assert_eq!(pt1(&data),50);
    }

    #[test]
    fn test_draw_lines() {
        let start = Point(1,0);
        let end = Point(5,10);
        let mut expect = Vec::new();
        expect.push(Line{start: Point(1,0),end: Point(5,0)});
        expect.push(Line{start: Point(1,0),end: Point(1,10)});
        expect.push(Line{start: Point(5,10),end: Point(5,0)});
        expect.push(Line{start: Point(5,10),end: Point(1,10)});

        let res = draw_lines(&start,&end);

        assert_eq!(res, expect);
    }

    #[test]
    fn test_crosses() {
        let v_line1 = Line{start: Point(5,0), end: Point(5,10)};
        let h_line1 = Line{start: Point(0,5), end: Point(10,5)};
        assert!(v_line1.crosses(&h_line1));

        let v_line2 = Line{start: Point(5,10), end: Point(5,11)};
        assert!(!v_line1.crosses(&v_line2));

        let h_line2 = Line{start:Point(10,5), end: Point(11,5)};
        assert!(!h_line1.crosses(&h_line2));

        let h_line3 = Line{start: Point(100,500), end: Point(110,500)};
        assert!(!v_line1.crosses(&h_line3));

        let v_line3 = Line{start: Point(100,500), end: Point(100,510)};
        assert!(!h_line1.crosses(&v_line3));

        let h_line4 = Line{start: Point(0,5), end: Point(5,5)};
        assert!(!v_line1.crosses(&h_line4));

        let v_line4 = Line{start: Point(5,0), end: Point(5,5)};
        assert!(!h_line1.crosses(&v_line4));
    }

    #[test]
    fn test_pt2() {
        let path = Path::new(BASE_DIR).join("test").join("day9.txt");
        let data = read_data(&path);

        assert_eq!(pt2(&data),24);
        
        let path = Path::new(BASE_DIR).join("test").join("day9-2.txt");
        let data = read_data(&path);

        assert_eq!(pt2(&data),40);

        let path = Path::new(BASE_DIR).join("test").join("day9-3.txt");
        let data = read_data(&path);

        assert_eq!(pt2(&data),35);

        let path = Path::new(BASE_DIR).join("test").join("day9-4.txt");
        let data = read_data(&path);

        assert_eq!(pt2(&data),66);

        let path = Path::new(BASE_DIR).join("test").join("day9-5.txt");
        let data = read_data(&path);

        assert_eq!(pt2(&data),8);

        let path = Path::new(BASE_DIR).join("test").join("day9-6.txt");
        let data = read_data(&path);

        assert_eq!(pt2(&data),30);

        let path = Path::new(BASE_DIR).join("real").join("day9.txt");
        let data = read_data(&path);

        assert_eq!(pt2(&data),1461987144);
    }
}
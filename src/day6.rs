use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;


pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day6.txt");
    let (nums, ops) = read_input(&path);
    let (num2, op2) = pt2_input(&path);
    (pt1(&nums,&ops), pt2(num2,op2))
}

fn read_input(path: &Path) -> (Vec<Vec<String>>,Vec<String>) {
    let mut nums: Vec<Vec<String>> = read_to_string(path).expect("Error reading file")
       .lines()
       .map(|s| {
        s.split_whitespace()
        .map(|s| s.to_owned())
        .collect()
       })
       .collect();
    let ops = nums.pop().unwrap();
    (nums,ops)
}

fn pt2_input(path: &Path) -> (Vec<String>, String) {
    let mut nums: Vec<String> = read_to_string(path).expect("Error reading file")
       .lines()
       .map(|s| {
        s.to_owned()})
        .collect();
    let ops = nums.pop().unwrap().chars().rev().collect::<String>();
    (nums,ops)
}

fn pt1(nums: &Vec<Vec<String>>, ops: &[String]) -> isize {
    let nums: Vec<Vec<isize>> = nums.iter().map(|v| v.iter().map(|n| n.parse::<isize>().unwrap()).collect()).collect();
    let mut res = 0;
    for (i,_) in nums.first().unwrap().iter().enumerate() {
        let mut prob_nums = Vec::new();
        for v in &nums {
            prob_nums.push(v[i]);
        }
        res += apply_ops(&prob_nums, &ops[i])
    }
    res
}

type Row = Vec<Vec<Option<isize>>>;

fn make_vec_digits(v: String, column_widths: &Vec<usize>) -> Row {
    let mut ch = v.chars().rev().peekable();
    let mut res = Vec::new();
    let mut cell = Vec::new();
    let mut col_widths_iter = column_widths.iter();
    let mut col_width = col_widths_iter.next().unwrap();
    while let Some(c) = ch.next() {
        if &cell.len() < col_width {
            cell.push(c.to_digit(10).map(|d| d as isize))
        } else {
            res.push(cell);
            cell = Vec::new();
            if ch.peek().is_some() {
                col_width =  col_widths_iter.next().unwrap();
            }
        }
    }
    if !cell.is_empty() {
        res.push(cell);
    }
    res
}

fn apply_ops(num: &[isize], op: &str) -> isize {
    match op {
        "+" => num.iter().sum(),
        "*" => num.iter().product(),
        _ => panic!("Invalid operation: {}", op),
    }
}

fn parse_ops(ops: String) -> (Vec<String>, Vec<usize>) {
    let mut ch = ops.chars();
    let mut col_widths = Vec::new();
    let mut ops = Vec::new();
    let mut curr_width = 0;
    while let Some(c) = ch.next() {
        if c.is_whitespace() {
            curr_width += 1;
        } else {
            ops.push(c.to_string());
            col_widths.push(curr_width);
            curr_width = 0;
        }
    }
    *col_widths.first_mut().unwrap() += 1;
    (ops, col_widths)
}

fn pt2(nums: Vec<String>, ops: String) -> isize {
    let mut res = 0;
    let (operations,col_widths) = parse_ops(ops);
    let digits: Vec<Row> = nums.into_iter().map(|s| make_vec_digits(s,&col_widths)).collect();
    for col in  0..digits.first().unwrap().len() { // i is column number
        let mut nums = Vec::new();
        let column: Vec<&Vec<Option<isize>>> = digits.iter().map(|v| v.get(col).unwrap()).collect();
        let op = &operations[col];
        for digit_place in 0..column.first().unwrap().len() {
            let mut num = 0;
            for cell in column.iter() {
                if let Some(Some(val)) = cell.get(digit_place) {
                    num *= 10;
                    num += val;
                }
            }
            nums.push(num);
        }
        res += apply_ops(&nums, op);
        
    }
    res
}

#[cfg(test)]
mod test{ 
    use super::*;

    #[test]
    fn test_pt1() {
        let path = Path::new(BASE_DIR).join("test").join("day6.txt");
        let (nums, ops) = read_input(&path);
        assert_eq!(pt1(&nums, &ops), 4277556);
    }

    #[test]
    fn test_make_vec_digits() {
        let s = "123 328  51 64 ".to_owned();
        let a = vec![None,Some(4), Some(6)];
        let b= vec![Some(1), Some(5), None];
        let c = vec![Some(8), Some(2), Some(3)];
        let d = vec![Some(3), Some(2), Some(1)];
        let expect = vec![a, b, c, d];

        let res = make_vec_digits(s,&vec![3,3,3,3]);

        assert_eq!(res, expect);
    }

    #[test]
    fn test_pt2() {
        let path = Path::new(BASE_DIR).join("test").join("day6.txt");
        let (nums, ops) = pt2_input(&path);
        assert_eq!(pt2(nums, ops), 3263827);
    }
}
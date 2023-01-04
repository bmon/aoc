use core::num;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("../input.html").unwrap();
    let r = io::BufReader::new(file);
    match lv2(r) {
        Ok(answer) => println!("{}", answer),
        Err(error) => panic!("calculation failed with error: {:?}", error),
    };
}

#[derive(Debug)]
enum CalcError {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    SomeError(String),
}

impl From<io::Error> for CalcError {
    fn from(err: io::Error) -> CalcError {
        CalcError::IOError(err)
    }
}

impl From<num::ParseIntError> for CalcError {
    fn from(err: num::ParseIntError) -> CalcError {
        CalcError::ParseIntError(err)
    }
}

fn lv1(reader: BufReader<File>) -> Result<i32, CalcError> {
    let mut total: i32 = 0;
    for line in reader.lines() {
        let ln = line?;
        let score = evaluate_score_lv1(&ln)?;
        total += score;
    }
    return Ok(total);
}

fn lv2(reader: BufReader<File>) -> Result<i32, CalcError> {
    let mut total: i32 = 0;
    for line in reader.lines() {
        let ln = line?;
        let score = evaluate_score_lv2(&ln)?;
        total += score;
    }
    return Ok(total);
}

fn evaluate_score_lv1(ln: &String) -> Result<i32, CalcError> {
    if let Some((lhs, rhs)) = ln.split_once(',') {
        let left = &Section::from(&String::from(lhs))?;
        let right = &Section::from(&String::from(rhs))?;

        if left.contains(right) || right.contains(left) {
            Ok(1)
        } else {
            Ok(0)
        }
    } else {
        Err(CalcError::SomeError(
            "couldn't parse section strings".to_owned(),
        ))
    }
}

fn evaluate_score_lv2(ln: &String) -> Result<i32, CalcError> {
    if let Some((lhs, rhs)) = ln.split_once(',') {
        let left = &Section::from(&String::from(lhs))?;
        let right = &Section::from(&String::from(rhs))?;

        if left.overlaps(right) {
            Ok(1)
        } else {
            Ok(0)
        }
    } else {
        Err(CalcError::SomeError(
            "couldn't parse section strings".to_owned(),
        ))
    }
}

struct Section {
    start: i32,
    end: i32,
}

impl Section {
    fn from(sec: &String) -> Result<Self, CalcError> {
        if let Some((lhs, rhs)) = sec.split_once('-') {
            Ok(Section {
                start: lhs.parse()?,
                end: rhs.parse()?,
            })
        } else {
            Err(CalcError::SomeError("malformed section: ".to_owned() + sec))
        }
    }

    fn contains(&self, rhs: &Section) -> bool {
        self.start <= rhs.start && self.end >= rhs.end
    }

    fn overlaps(&self, rhs: &Section) -> bool {
        (self.start <= rhs.end && self.end >= rhs.start)
            || (self.end >= rhs.start && self.start <= rhs.end)
    }
}

use core::num;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    match lv2() {
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

fn lv1() -> Result<i32, CalcError> {
    let file = File::open("../input.html")?;
    let lines = io::BufReader::new(file).lines();

    let mut current: i32 = 0;
    let mut sizes: Vec<i32> = Vec::new();
    for line in lines {
        let cal = line?;
        if cal == "" {
            sizes.push(current);
            current = 0;
        } else {
            current += cal.parse::<i32>()?;
        }
    }
    sizes.sort();

    if let Some(result) = sizes.last() {
        return Ok(*result);
    };
    return Err(CalcError::SomeError("no results found".into()));
}

fn lv2() -> Result<i32, CalcError> {
    let file = File::open("../input.html")?;
    let lines = io::BufReader::new(file).lines();

    let mut current: i32 = 0;
    let mut sizes: Vec<i32> = Vec::new();
    for line in lines {
        let cal = line?;
        if cal == "" {
            sizes.push(current);
            current = 0;
        } else {
            current += cal.parse::<i32>()?;
        }
    }
    sizes.sort();

    let len = sizes.len();
    if len < 3 {
        return Err(CalcError::SomeError("not enough results found".into()));
    }

    let total = sizes[len - 3..].iter().sum();

    return Ok(total);
}

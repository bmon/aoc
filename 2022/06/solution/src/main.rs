use core::num;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() {
    let file = File::open("../input.html").unwrap();
    let mut r = io::BufReader::new(file);
    match lv2(&mut r) {
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

fn lv1(reader: &mut BufReader<File>) -> Result<i32, CalcError> {
    let mut ln = String::new();
    reader.read_line(&mut ln)?;

    for i in 4..ln.len() {
        match ln.get(i - 4..i) {
            None => continue,
            Some(tok) => {
                let mut v: Vec<_> = tok.chars().collect();
                v.sort();
                v.dedup();
                if v.len() == 4 {
                    return Ok(i.try_into().unwrap());
                }
            }
        }
    }
    Err(CalcError::SomeError("didn't find match".to_owned()))
}

fn lv2(reader: &mut BufReader<File>) -> Result<i32, CalcError> {
    let mut ln = String::new();
    reader.read_line(&mut ln)?;

    for i in 14..ln.len() {
        match ln.get(i - 14..i) {
            None => continue,
            Some(tok) => {
                let mut v: Vec<_> = tok.chars().collect();
                v.sort();
                v.dedup();
                if v.len() == 14 {
                    return Ok(i.try_into().unwrap());
                }
            }
        }
    }
    Err(CalcError::SomeError("didn't find match".to_owned()))
}

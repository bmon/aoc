#![feature(iter_array_chunks)]
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
        //println!("{} gives score {}", ln, score);
    }
    return Ok(total);
}

fn lv2(reader: BufReader<File>) -> Result<i32, CalcError> {
    let mut total: i32 = 0;
    for [i1, i2, i3] in reader.lines().array_chunks() {
        let mut g1: Vec<char> = i1?.chars().collect();
        let mut g2: Vec<char> = i2?.chars().collect();
        let mut g3: Vec<char> = i3?.chars().collect();

        g1.sort();
        g1.dedup();
        g2.sort();
        g2.dedup();
        g3.sort();
        g3.dedup();

        for token in g1 {
            if g2.contains(&token) && g3.contains(&token) {
                match evaluate_token_score(token) {
                    Some(score) => total += score,
                    None => {
                        return Err(CalcError::SomeError(format!(
                            "unable to evaluate token score: {}",
                            token
                        )))
                    }
                }
            }
        }
    }
    return Ok(total);
}

fn evaluate_score_lv1(ln: &String) -> Result<i32, CalcError> {
    let (lhs, rhs) = ln.split_at(ln.len() / 2);

    for token in lhs.chars() {
        if rhs.contains(token) {
            match evaluate_token_score(token) {
                Some(score) => return Ok(score),
                None => {
                    return Err(CalcError::SomeError(format!(
                        "unable to evaluate token score: {}",
                        token
                    )))
                }
            };
        };
    }
    Err(CalcError::SomeError(format!(
        "did not find token in both bags: {} {}",
        lhs, rhs
    )))
}

fn evaluate_token_score(ch: char) -> Option<i32> {
    // wonder if there's a better way to do this eg python's ord().
    // explicitly forcing software developers to handle unicode is fine just would be more
    // convenient to not have to manually allocate the bytes to unmarshal my value into... what
    // would have been wrong with ch.encode_utf8 -> i32?
    let mut buf = [0, 1];
    ch.encode_utf8(&mut buf);

    if ch.is_lowercase() {
        return Some(i32::from(buf[0] - 97 + 1)); // ascii value, minus 97 (ascii 'a') + 1
    }
    if ch.is_uppercase() {
        return Some(i32::from(buf[0] - 65 + 27)); // ascii value, minus 65 (ascii 'A') + 27
    }
    return None;
}

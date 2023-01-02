use core::num;
use regex::Regex;
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

fn evaluate_score_lv1(ln: &String) -> Result<i32, CalcError> {
    let re: Regex = Regex::new(r"(?P<opponent>.) (?P<selected>.)").unwrap();
    let mut score: i32 = 0;
    match re.captures(&*ln).and_then(|cap| -> Option<i32> {
        let sel = cap.name("selected")?.as_str();
        match cap.name("opponent")?.as_str() {
            "A" => match sel {
                "X" => score += 3 + 1,
                "Y" => score += 6 + 2,
                "Z" => score += 0 + 3,
                _ => return None,
            },
            "B" => match sel {
                "X" => score += 0 + 1,
                "Y" => score += 3 + 2,
                "Z" => score += 6 + 3,
                _ => return None,
            },
            "C" => match sel {
                "X" => score += 6 + 1,
                "Y" => score += 0 + 2,
                "Z" => score += 3 + 3,
                _ => return None,
            },
            _ => return None,
        }
        Some(score)
    }) {
        Some(_) => return Ok(score),
        None => {
            return Err(CalcError::SomeError(
                ln.to_owned() + " could not be evaluated",
            ))
        }
    }
}

fn lv2(reader: BufReader<File>) -> Result<i32, CalcError> {
    let mut total: i32 = 0;
    for line in reader.lines() {
        let ln = line?;
        let score = evaluate_score_lv2(&ln)?;
        total += score;
        //println!("{} gives score {}", ln, score);
    }
    return Ok(total);
}

fn evaluate_score_lv2(ln: &String) -> Result<i32, CalcError> {
    let re: Regex = Regex::new(r"(?P<opponent>.) (?P<selected>.)").unwrap();
    let mut score: i32 = 0;
    match re.captures(&*ln).and_then(|cap| -> Option<i32> {
        let sel = cap.name("selected")?.as_str();
        match cap.name("opponent")?.as_str() {
            "A" => match sel {
                "X" => score += 0 + 3,
                "Y" => score += 3 + 1,
                "Z" => score += 6 + 2,
                _ => return None,
            },
            "B" => match sel {
                "X" => score += 0 + 1,
                "Y" => score += 3 + 2,
                "Z" => score += 6 + 3,
                _ => return None,
            },
            "C" => match sel {
                "X" => score += 0 + 2,
                "Y" => score += 3 + 3,
                "Z" => score += 6 + 1,
                _ => return None,
            },
            _ => return None,
        }
        Some(score)
    }) {
        Some(_) => return Ok(score),
        None => {
            return Err(CalcError::SomeError(
                ln.to_owned() + " could not be evaluated",
            ))
        }
    }
}

use core::num;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

fn lv1(reader: &mut BufReader<File>) -> Result<String, CalcError> {
    let mut stack = CrateStack::from(reader)?;
    stack.render();
    stack.execute_operations(reader)?;
    stack.render();
    Ok(stack.top_crates())
}

fn lv2(reader: &mut BufReader<File>) -> Result<String, CalcError> {
    let mut stack = CrateStack::from(reader)?;
    stack.render();
    stack.multicrate = true;
    stack.execute_operations(reader)?;
    stack.render();
    Ok(stack.top_crates())
}

struct CrateStack {
    stacks: Vec<Vec<char>>,
    multicrate: bool,
}

impl CrateStack {
    fn from(reader: &mut BufReader<File>) -> Result<Self, CalcError> {
        let mut crates = CrateStack {
            stacks: Vec::new(),
            multicrate: false,
        };
        for line in reader.lines() {
            let ln = line?;
            if ln == "" {
                break;
            }
            if ln.starts_with(" 1 ") {
                continue;
            }
            if crates.stacks.len() == 0 {
                let num_stacks: usize = (ln.len() + 1) / 4;
                (0..num_stacks).for_each(|_| crates.stacks.push(Vec::new()))
            }

            let mut idx = 0;
            loop {
                let tok = &ln.get(idx * 4..idx * 4 + 3);
                if tok.is_none() {
                    break;
                }

                let tok = tok.unwrap();
                if tok != "   " {
                    let ch = tok.chars().nth(1).unwrap();
                    crates.stacks.get_mut(idx).unwrap().insert(0, ch)
                }
                idx += 1;
            }
        }
        Ok(crates)
    }

    fn render(&self) {
        let longest = self.stacks.iter().map(|c| c.len()).max().unwrap();

        for i in (0..longest).rev() {
            for stack in self.stacks.iter() {
                if let Some(ch) = stack.get(i) {
                    print!("[{}] ", ch)
                } else {
                    print!("    ")
                }
            }
            println!()
        }
        (1..self.stacks.len() + 1).for_each(|n| print!(" {}  ", n));
        println!()
    }

    fn execute_operations(&mut self, reader: &mut BufReader<File>) -> Result<(), CalcError> {
        let re: Regex =
            Regex::new(r"move (?P<amount>\d+) from (?P<src>\d+) to (?P<dst>\d+)").unwrap();
        for line in reader.lines() {
            let ln = line?;
            re.captures(&*ln)
                .and_then(|cap| -> Option<()> {
                    let amount: usize = cap.name("amount")?.as_str().parse().unwrap();
                    let src: usize = cap.name("src")?.as_str().parse().unwrap();
                    let dst: usize = cap.name("dst")?.as_str().parse().unwrap();
                    self.execute(amount, src, dst).unwrap();
                    Some(()) // looks like I am committing evil here with all this unwrap, would be better to return
                             // Result but the compiler will not let me, instead I have to return Option...
                })
                .unwrap();
        }
        Ok(())
    }

    fn execute(&mut self, amount: usize, src: usize, dst: usize) -> Result<(), CalcError> {
        if self.multicrate {
            let src = self.stacks.get_mut(src - 1).unwrap();
            for tok in src.drain(src.len() - amount..).collect::<Vec<char>>() {
                self.stacks.get_mut(dst - 1).unwrap().push(tok);
            }
        } else {
            for _ in 0..amount {
                let tok = self.stacks.get_mut(src - 1).unwrap().pop().unwrap();
                self.stacks.get_mut(dst - 1).unwrap().push(tok);
            }
        }

        Ok(())
    }

    fn top_crates(&self) -> String {
        let mut res = String::new();
        for stack in self.stacks.iter() {
            match stack.last() {
                Some(tok) => res.push(*tok),
                None => res.push('_'),
            }
        }
        res
    }
}

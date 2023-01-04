use core::num;
use std::collections::HashMap;
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

fn lv1(reader: &mut BufReader<File>) -> Result<i32, CalcError> {
    let mut dirs: HashMap<String, ChallengeDir> = HashMap::new();
    dirs.insert("/".to_owned(), ChallengeDir::new("/"));

    let mut cwd = dirs.get_mut("/").unwrap();

    for line in reader.lines() {
        let ln = line?;
        match ln {
            _ if ln.starts_with("$ ls") => continue,
            _ if ln.starts_with("$ cd") => {
                cwd = dirs.get_mut(ln.strip_prefix("$ cd ").unwrap()).unwrap();
                continue;
            }
            _ if ln.starts_with("dir") => {
                let dirname = ln.strip_prefix("dir ").unwrap();
                let res = dirs.insert(dirname.into(), ChallengeDir::new(dirname));
                match res {
                    Some(_) => {
                        return Err(CalcError::SomeError(format!(
                            "inserted duplicate dir: {}",
                            dirname
                        )))
                    }
                    None => {}
                }
                cwd.add_dir(dirs.get(dirname).unwrap())
            }
            _ => {
                let (size_s, name) = ln.split_once(' ').unwrap();
                let size = size_s.parse()?;
                cwd.add_file(name.into(), size);
            }
        }
    }

    Ok(0)
}

fn lv2(reader: &mut BufReader<File>) -> Result<i32, CalcError> {
    Ok(0)
}

struct ChallengeFile {
    name: String,
    size: usize,
}

struct ChallengeDir<'a> {
    name: &'a str,
    dirs: Vec<&'a ChallengeDir<'a>>,
    files: Vec<ChallengeFile>,
}

impl<'a> ChallengeDir<'a> {
    fn new(name: &str) -> Self {
        let dirs = Vec::new();
        let files = Vec::new();
        ChallengeDir { name, dirs, files }
    }

    fn add_dir(&mut self, dir: &'a ChallengeDir) {
        self.dirs.push(dir)
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.files.push(ChallengeFile { name, size })
    }
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

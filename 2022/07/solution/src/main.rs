use core::num;
use std::collections::HashMap;
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

struct Dirname {
    vec: Vec<String>,
}

impl Dirname {
    fn dirname(&self) -> String {
        "/".to_owned() + &self.vec.join("/")
    }
    fn newdirname(&self, newdir: &String) -> String {
        if self.dirname() == "/" {
            return format!("/{}", newdir);
        }
        return format!("{}/{}", self.dirname(), newdir);
    }
}

fn lv1(reader: &mut BufReader<File>) -> Result<usize, CalcError> {
    let mut dirs: HashMap<String, ChallengeDir> = HashMap::new();
    let root = ChallengeDir::new("/".to_owned());
    dirs.insert("/".to_owned(), root);

    let mut cwd = Dirname { vec: Vec::new() };

    for line in reader.lines() {
        let ln = line?;
        println!("{}", ln);
        match ln {
            _ if ln.starts_with("$ ls") => {
                continue;
            }
            _ if ln.starts_with("$ cd /") => {
                continue;
            }
            _ if ln.starts_with("$ cd ..") => {
                cwd.vec.pop().unwrap();
            }
            _ if ln.starts_with("$ cd") => {
                let dir = ln.strip_prefix("$ cd ").unwrap().to_owned();
                cwd.vec.push(dir);
            }
            _ if ln.starts_with("dir") => {
                let dirname = ln.strip_prefix("dir ").unwrap().to_owned();
                let newdir = ChallengeDir::new(cwd.newdirname(&dirname));
                match dirs.insert(cwd.newdirname(&dirname), newdir) {
                    Some(_) => {
                        return Err(CalcError::SomeError(format!(
                            "duplicate directory {}",
                            dirname
                        )))
                    }
                    None => {}
                };

                println!("{} {:?}", cwd.dirname(), dirs.keys());
                dirs.get_mut(&cwd.dirname())
                    .unwrap()
                    .add_dir(cwd.newdirname(&dirname));
            }
            _ => {
                let (size_s, name) = ln.split_once(' ').unwrap();
                let size = size_s.parse()?;
                dirs.get_mut(&cwd.dirname())
                    .unwrap()
                    .add_file(name.to_owned(), size);
            }
        }
    }

    let mut size_sum = 0;
    for (_, dir) in dirs.iter() {
        let size = dir.sizeof(&dirs)?;
        println!("{} - {}", dir.name, size);
        if size <= 100000 {
            size_sum += size;
        }
    }

    Ok(size_sum)
}

fn lv2(reader: &mut BufReader<File>) -> Result<usize, CalcError> {
    let mut dirs: HashMap<String, ChallengeDir> = HashMap::new();
    let root = ChallengeDir::new("/".to_owned());
    dirs.insert("/".to_owned(), root);

    let mut cwd = Dirname { vec: Vec::new() };

    for line in reader.lines() {
        let ln = line?;
        println!("{}", ln);
        match ln {
            _ if ln.starts_with("$ ls") => {
                continue;
            }
            _ if ln.starts_with("$ cd /") => {
                continue;
            }
            _ if ln.starts_with("$ cd ..") => {
                cwd.vec.pop().unwrap();
            }
            _ if ln.starts_with("$ cd") => {
                let dir = ln.strip_prefix("$ cd ").unwrap().to_owned();
                cwd.vec.push(dir);
            }
            _ if ln.starts_with("dir") => {
                let dirname = ln.strip_prefix("dir ").unwrap().to_owned();
                let newdir = ChallengeDir::new(cwd.newdirname(&dirname));
                match dirs.insert(cwd.newdirname(&dirname), newdir) {
                    Some(_) => {
                        return Err(CalcError::SomeError(format!(
                            "duplicate directory {}",
                            dirname
                        )))
                    }
                    None => {}
                };

                println!("{} {:?}", cwd.dirname(), dirs.keys());
                dirs.get_mut(&cwd.dirname())
                    .unwrap()
                    .add_dir(cwd.newdirname(&dirname));
            }
            _ => {
                let (size_s, name) = ln.split_once(' ').unwrap();
                let size = size_s.parse()?;
                dirs.get_mut(&cwd.dirname())
                    .unwrap()
                    .add_file(name.to_owned(), size);
            }
        }
    }

    let unused = 70000000 - dirs.get("/").unwrap().sizeof(&dirs)?;

    let mut best = 10000000000000;
    for (_, dir) in dirs.iter() {
        let size = dir.sizeof(&dirs)?;
        println!("{} - {}", dir.name, size);
        if size + unused >= 30000000 && best > size {
            best = size
        }
    }
    println!("required: {}", unused);

    Ok(best)
}

#[derive(Debug)]
struct ChallengeFile {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct ChallengeDir {
    name: String,
    dirs: Vec<String>,
    files: Vec<ChallengeFile>,
}

impl ChallengeDir {
    fn new(name: String) -> Self {
        let dirs = Vec::new();
        let files = Vec::new();
        ChallengeDir { name, dirs, files }
    }

    fn add_dir(&mut self, name: String) {
        self.dirs.push(name)
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.files.push(ChallengeFile { name, size })
    }

    fn sizeof(&self, dirs: &HashMap<String, ChallengeDir>) -> Result<usize, CalcError> {
        let mut size = self.files.iter().map(|f| f.size).sum();
        for dirname in self.dirs.iter() {
            match dirs.get(dirname) {
                Some(subdir) => size += subdir.sizeof(dirs)?,
                None => {
                    return Err(CalcError::SomeError(format!(
                        "couldn't find subdir {} in dirs list",
                        dirname
                    )))
                }
            }
        }
        Ok(size)
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

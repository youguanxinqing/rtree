use std::fs;
use std::path;
use std::fmt;
use std::iter::Iterator;
use std::collections::HashMap;

use fs::DirEntry;

fn main() {
    let cmd = Cmd::new(std::env::args()).unwrap_or_else(|err|{
        panic!(err);
    });
    cmd.run();
}

struct Cmd {
    dir: String,
    opts: Opts
}

impl Cmd {
    fn new(args: std::env::Args) -> Result<Cmd, &'static str> {
        let mut args = args.skip(1);
        
        let dir = match args.next() {
            Some(arg) => arg,
            None => String::from(".")
        };
        
        let opts = Opts::new(args)?;
        Ok(Cmd{dir, opts})
    }

    fn run(&self) {
        if self.opts.is_empty {
            println!("_");
        }
        let mut walker = Walker::new(self.dir.as_str()).unwrap_or_else(|err|{
            panic!(err);
        });
        walker.walk(&self.opts);
    }
}

struct Opts {
    is_empty: bool,
    need_all: bool
}

impl Opts {
    fn new(mut _args: std::iter::Skip<std::env::Args>) -> Result<Opts, &'static str> {
        Ok(Opts{
            is_empty: true,
            need_all: false
        })
    }
}

// walk dir
struct Walker<'a> {
    dir: &'a str
}

impl <'a> Walker<'a> {
    fn new(dir: &'a str) -> Result<Walker, &'static str> {
        if dir.is_empty() {
            return Err("Walker::new cannot support empty string");
        }
        Ok(Walker{ dir })
    }

    fn walk(&mut self, opts: &Opts) {
        let paths = fs::read_dir(path::Path::new(self.dir)).expect("except a correct dir");
        let mut map: HashMap<i32, Vec<Option<DirEntry>>> = HashMap::new();
        let paths: Vec<Option<DirEntry>> = paths.map(|x| -> Option<DirEntry> {
            Some(x.unwrap())
        }).collect();
        
        let mut max_level: i32 = 0;
        if !paths.is_empty() {
            map.insert(max_level, paths);
        }
        loop {
            if map.is_empty() || max_level < 0 { break }

            if let Some(paths) = map.get_mut(&max_level) {
                if !paths.is_empty() {
                    let path = paths.pop().unwrap().unwrap();
                    let file_name = String::from(path.file_name().to_str().unwrap());
                    // filter hiden file or dir
                    if !opts.need_all && file_name.starts_with(".") {
                        continue;
                    }

                    let file_type = path.file_type().unwrap();
                    let file = File::new(file_name, max_level, file_type.is_dir());
                    println!("{}", file);

                    if file_type.is_dir() {
                        let paths = fs::read_dir(path.path()).unwrap();
                        let paths: Vec<Option<DirEntry>> = paths.map(|x| -> Option<DirEntry> {
                            Some(x.unwrap())
                        }).collect();
                        max_level += 1;
                        map.insert(max_level, paths);
                    }
                } else {
                    max_level -= 1;
                }
            } else {
                max_level -= 1;
            }
        }
    }
}

struct File {
    level: i32,
    filename: String,
    is_dir: bool
}

impl File {
    fn new(filename: String, level: i32, is_dir: bool) -> File {
        File{filename, level, is_dir}
    }
}

// output file fmt
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for _ in 0..self.level {
            s.push_str("| ");
        }
        s.push_str("|-");
        s.push_str(self.filename.as_str());
        if self.is_dir {
            s.push_str("(d)")
        } else {
            s.push_str("(f)")
        }
        write!(f, "{}", s)
    }
}
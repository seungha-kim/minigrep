extern crate minigrep;

use std::env;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::process::exit;
use minigrep::grep_line;

const ERR_NO_PAT: &str = "usage: minigrep <PATTERN> [FILENAME]";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let pattern = match args.get(1) {
        Some(v) => v,
        None => {
            eprintln!("{}", ERR_NO_PAT);
            exit(1);
        }
    };
    let file_name_opt = args.get(2);
    match file_name_opt {
        Some(v) => {
            let f = File::open(v)?;
            let lines = BufReader::new(f).lines();
            print_line(lines, pattern);
        },
        None => {
            let stdin = io::stdin();
            let lines = stdin.lock().lines();
            print_line(lines, pattern);
        },
    };
    
    Ok(())
}

fn print_line<T: BufRead>(lines: io::Lines<T>, pattern: &str) {
    for line in lines {
        let line = line.unwrap();
        if grep_line(&line, pattern) {
            println!("{}", line);
        }
    }
}
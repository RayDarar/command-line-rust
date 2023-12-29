use std::{
    error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Config {
    /// Files to concatenate
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number the output lines, starting at 1
    #[arg(short = 'n', long = "number", group = "numbering")]
    number_lines: bool,

    /// Number only nonblank output lines
    #[arg(short = 'b', long = "number-nonblank", group = "numbering")]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn error::Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> MyResult<()> {
    let config = Config::parse();

    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buffer) => {
                let mut i = 0;
                let has_numbering = config.number_lines || config.number_nonblank_lines;

                for line in buffer.lines() {
                    let line = line?;

                    if line.is_empty() && config.number_nonblank_lines {
                        println!();
                        continue;
                    }

                    i += 1;
                    if has_numbering {
                        println!("{:>6}\t{}", i, line);
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

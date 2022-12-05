use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Result type 
type MyResult<T> = Result<T, Box<dyn Error>>;

/*
 * This is the configuration structure. It gives a clearer meaning to this 
 * data.
 */
#[derive(Debug)]
pub struct Config {
    _files: Vec<String>,            // file name list
    _number_lines: bool,            // if the flag -n is passed
    _number_nonblank_lines: bool,   // if the flag -b is passed
}

/*
 * This function get the arguments using the clap crate (one of simplest for 
 * working with command-line arguments). A vector of file names is built and
 * other arguments are returned too.
 */
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Hamilton G. Jr. <hamiltonjr2010@gmail.com>")
        .about("Rust cat")

        // argument dash for redirected stdin
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )

        // argument n for nembered lines 
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )

        // argument n for non-blank nembered lines 
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    // returns configuration given for the arguments
    Ok(Config {
        _files: matches.values_of_lossy("files").unwrap(),
        _number_lines: matches.is_present("number"),
        _number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

/*
 * This function opens a file and make error handling for files. A file can be 
 * unreadable, corrupted or unexistent: such problems would crash the program
 * without an error handling.
 */
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/*
 * This function implements all the program logic. Given the files and other 
 * arguments, it reads each file and shows it properly.
 */
pub fn run(config: Config) -> MyResult<()> {
    // run for the file name list
    for filename in config._files {
        match open(&filename) {
            // error handling for files
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                // line numbering given for arguments
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if config._number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config._number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

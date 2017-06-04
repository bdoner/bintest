#[macro_use]
extern crate clap;
extern crate ansi_term;

mod args;
mod test;
mod filesystem;
mod results;

use std::process::exit;
use std::fs::read_dir;
use std::io::Write;
use ansi_term::Colour::{Red,Yellow,Green};
use test::Test;

fn main() {
    let args = args::get_args();
    let quiet = args.occurrences_of("quiet") > 0;
    let tests = make_tests(&args);

    println!("Running {} tests...", tests.len());

    let mut results = results::Results::new();

    for test in tests {
        if !quiet {
            print!("Test {} ... ", test.name());
            std::io::stdout().flush().unwrap();
        }

        let ans = test.run();
        results.register(&ans, &test);

        if quiet {
            match ans {
                Ok(true) => print!("."),
                Ok(false) => print!("{}", Red.paint("F")),
                Err(_) => print!("{}", Yellow.paint("E")),
            }
            std::io::stdout().flush().unwrap();
        } else {
            match ans {
                Ok(true) => println!("{}", Green.paint("ok")),
                Ok(false) => println!("{}", Red.paint("FAIL")),
                Err(e) => println!("{} {}", Yellow.paint("ERROR:"), e),
            }
        }
    }

    if quiet {
        println!("");
    }

    println!("Results: {}", results);
}

fn make_tests(args: &clap::ArgMatches) -> Vec<Test> {
    let test_dir = args.value_of("directory").unwrap();

    let source = args.value_of("source").unwrap();
    let temp = args.value_of("temp").unwrap();
    let expected = args.value_of("expected").unwrap();
    let command = args.value_of("command").unwrap();
    let verbose = args.occurrences_of("verbose") > 0;

    let tests_dir = match read_dir(test_dir) {
        Ok(ans) => ans,
        Err(e) => {
            println!("Failed to list contents of dir. {}", e);
            exit(1);
        }
    };

    let mut tests = Vec::<Test>::new();

    for directory in tests_dir {
        let directory = directory.unwrap();
        if directory.file_type().unwrap().is_dir() {
            tests.push(Test::new(source, temp, expected, command, &directory, !verbose));
        }
    }

    tests
}

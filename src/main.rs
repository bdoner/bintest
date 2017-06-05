#[macro_use]
extern crate clap;
extern crate ansi_term;

mod args;
mod filesystem;
mod results;
mod test;

use ansi_term::Colour::{Red, Yellow, Green, Blue};
use std::fs::read_dir;
use std::io::Write;
use std::process::exit;
use test::{Test, TestResult};

fn main() {
    let args = args::get_args();
    let tests = make_tests(&args);

    println!("Running {} tests...", tests.len());

    let mut results = results::Results::new();

    for test in tests {
        if !args.quiet {
            print!("Test {} ... ", test.name());
            std::io::stdout().flush().unwrap();
        }

        let ans = test.run();
        results.register(&ans, &test);

        if args.quiet {
            match ans {
                Ok(TestResult::Success) => print!("."),
                Ok(TestResult::Fail) => print!("{}", Red.paint("F")),
                Ok(TestResult::Ignored) => print!("{}", Blue.paint("I")),
                Err(_) => print!("{}", Yellow.paint("E")),
            }
            std::io::stdout().flush().unwrap();
        } else {
            match ans {
                Ok(TestResult::Success) => println!("{}", Green.paint("ok")),
                Ok(TestResult::Fail) => println!("{}", Red.paint("FAIL")),
                Ok(TestResult::Ignored) => println!("{}", Blue.paint("ignored")),
                Err(e) => println!("{} {}", Yellow.paint("ERROR:"), e),
            }
        }
    }

    if args.quiet {
        println!("");
    }

    println!("Results: {}", results);
}

fn make_tests(args: &args::Arguments) -> Vec<Test> {
    let tests_dir = match read_dir(&args.test_dir) {
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
            tests.push(Test::new(&directory, args));
        }
    }

    tests
}

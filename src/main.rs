#[macro_use]
extern crate clap;

mod args;
mod test;

use std::process::exit;
use std::fs::read_dir;
use std::io::Write;

fn main() {
    let args = args::get_args();
    let test_dir = args.value_of("directory").unwrap();
    let options = test::Options {
        source: args.value_of("source").unwrap(),
        temp: args.value_of("temp").unwrap(),
        expected: args.value_of("expected").unwrap(),
        command: args.value_of("command").unwrap(),
    };

    let tests = match read_dir(test_dir) {
        Ok(ans) => ans,
        Err(e) => {
            println!("Failed to list contents of dir. {}", e);
            exit(1);
        }
    };

    for directory in tests {
        let directory = directory.unwrap();
        if !directory.file_type().unwrap().is_dir() {
            continue;
        }
        print!("Running test {:?}: ", directory.file_name());
        std::io::stdout().flush().unwrap();
        match test::run_test(&options, &directory.path()) {
            Ok(true) => println!("Success."),
            Ok(false) => println!("Fail."),
            Err(e) => println!("Error: {}", e),
        }
    }
}

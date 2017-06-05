use std::process::Command;
use std::path::PathBuf;
use std::io::Read;
use std::{io, fs};

use filesystem;
use args::Arguments;

pub struct Test {
    name: String,
    source: PathBuf,
    temp: PathBuf,
    expected: PathBuf,
    cmd: String,
    quiet: bool,
    ignore: bool,
    clean_failed: bool,
}

impl Test {
    pub fn new(directory: &::std::fs::DirEntry, args: &Arguments) -> Test {
        let name = directory.file_name();
        let directory = directory.path().to_path_buf();
        let command = directory.join(&args.command);
        let cmd = fs::File::open(command)
            .and_then(|mut f| {
                          let mut cmd_str = String::new();
                          f.read_to_string(&mut cmd_str).map(move |_| cmd_str)
                      })
            .unwrap_or_else(|_| "cargo run -q".to_owned());

        Test {
            name: name.to_string_lossy().into_owned(),
            source: directory.join(&args.source),
            temp: directory.join(&args.temp),
            expected: directory.join(&args.expected),
            cmd: cmd,
            quiet: !args.verbose,
            ignore: args.ignore,
            clean_failed: args.clean_failed,
        }
    }

    pub fn run(&self) -> TestResult {
        let ans = self.real_run();
        if self.clean_failed {
            match ans {
                Ok(TestResult::Fail) |
                Err(_) => {
                    let err = fs::remove_dir_all(&self.temp);
                    if err.is_err() {
                        return TestResult::Error(None);
                    }
                }
                _ => {}
            }
        }
        match ans {
            Ok(res) => res,
            Err(e) => TestResult::Error(Some(e)),
        }
    }

    fn real_run(&self) -> Result<TestResult, io::Error> {
        if self.ignore && self.name.starts_with("ignore") {
            return Ok(TestResult::Ignored);
        }

        // Prepare temp
        let _ = fs::remove_dir_all(&self.temp);
        filesystem::recursive_copy(&self.source, &self.temp)?;

        // Run in temp
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&self.cmd).current_dir(&self.temp);
        // Suppress output if quiet
        if self.quiet {
            cmd.output()?;
        } else {
            cmd.status()?;
        }

        // Compare answer
        let ans = filesystem::compare_dirs(&self.temp, &self.expected).unwrap_or(false);
        Ok(if ans {
               fs::remove_dir_all(&self.temp)?;
               TestResult::Success
           } else {
               TestResult::Fail
           })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub enum TestResult {
    Success,
    Ignored,
    Fail,
    Error(Option<io::Error>),
}

impl From<Result<TestResult, io::Error>> for TestResult {
    fn from(other: Result<TestResult, io::Error>) -> TestResult {
        match other {
            Ok(result) => result,
            Err(err) => TestResult::Error(Some(err))
        }
    }
}

impl ::std::cmp::Eq for TestResult {
}

impl ::std::cmp::PartialEq for TestResult {
    fn eq(&self, other: &TestResult) -> bool {
        match (self, other) {
            (&TestResult::Success, &TestResult::Success) |
            (&TestResult::Fail, &TestResult::Fail) |
            (&TestResult::Ignored, &TestResult::Ignored) |
            (&TestResult::Error(_), &TestResult::Error(_)) => true,
            _ => false,
        }
    }
}

impl ::std::hash::Hash for TestResult {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        match *self {
            TestResult::Success => 1,
            TestResult::Fail => 2,
            TestResult::Ignored => 3,
            TestResult::Error(_) => 4,
        }.hash(state);
    }
}

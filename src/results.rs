use test::{Test, TestResult};
use std::io;
use std::fmt::Display;
use std::fmt;
use ansi_term::Colour::{Red, Yellow};
use ansi_term;

pub struct Results {
    success: Vec<String>,
    fail: Vec<String>,
    ignore: Vec<String>,
    error: Vec<String>,
}

impl Results {
    pub fn new() -> Results {
        Results {
            success: Vec::new(),
            fail: Vec::new(),
            ignore: Vec::new(),
            error: Vec::new(),
        }
    }

    pub fn register(&mut self, ans: &Result<TestResult, io::Error>, test: &Test) {
        let name = test.name().to_owned();
        match *ans {
            Ok(TestResult::Success) => self.success.push(name),
            Ok(TestResult::Fail) => self.fail.push(name),
            Ok(TestResult::Ignored) => self.ignore.push(name),
            Err(_) => self.error.push(name),
        }
    }

    fn print_elements(fmter: &mut fmt::Formatter,
                      elements: &[String],
                      name: &str,
                      style: ansi_term::Style)
                      -> Result<(), fmt::Error> {
        if elements.is_empty() {
            return Ok(());
        }
        fmter
            .write_str(&format!("{}{} \n", style.paint(name), style.paint(":")))?;
        for element in elements {
            fmter.write_str(element)?;
            fmter.write_str("\n")?;
        }
        Ok(())
    }
}

impl Display for Results {
    fn fmt(&self, fmter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmter
            .write_str(&format!("Success: {}, Fail: {}, Error: {}, Ignored: {}\n",
                               self.success.len(),
                               self.fail.len(),
                               self.error.len(),
                               self.ignore.len()))?;
        Results::print_elements(fmter, &self.fail, "Failed", Red.normal())?;
        Results::print_elements(fmter, &self.error, "Error", Yellow.normal())?;
        Ok(())
    }
}

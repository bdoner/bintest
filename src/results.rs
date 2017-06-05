use test::{Test, TestResult};
use std::fmt::Display;
use std::fmt;
use ansi_term::Colour::{Red, Yellow};
use ansi_term;
use std::collections::HashMap;

pub struct Results {
    results: HashMap<TestResult, Vec<String>>,
}

impl Results {
    pub fn new() -> Results {
        let mut results = HashMap::new();
        results.insert(TestResult::Success, Vec::new());
        results.insert(TestResult::Fail, Vec::new());
        results.insert(TestResult::Ignored, Vec::new());
        results.insert(TestResult::Error(None), Vec::new());
        Results { results: results }
    }

    pub fn register(&mut self, ans: &TestResult, test: &Test) {
        let name = test.name().to_owned();
        let vec = self.results.get_mut(ans).unwrap();
        vec.push(name);
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
        let success = &self.results[&TestResult::Success];
        let fail = &self.results[&TestResult::Fail];
        let error = &self.results[&TestResult::Error(None)];
        let ignored = &self.results[&TestResult::Ignored];

        fmter
            .write_str(&format!("Success: {}, Fail: {}, Error: {}, Ignored: {}\n",
                               success.len(),
                               fail.len(),
                               error.len(),
                               ignored.len()))?;
        Results::print_elements(fmter, fail, "Failed", Red.normal())?;
        Results::print_elements(fmter, error, "Error", Yellow.normal())?;
        Ok(())
    }
}

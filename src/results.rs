use test::Test;
use std::io;
use std::fmt::Display;
use std::fmt;

pub struct Results {
    success: Vec<String>,
    fail: Vec<String>,
    error: Vec<String>,
}

impl Results {
    pub fn new() -> Results {
        Results {
            success: Vec::new(),
            fail: Vec::new(),
            error: Vec::new(),
        }
    }

    pub fn register(&mut self, ans: &Result<bool, io::Error>, test: &Test) {
        let name = test.name().to_owned();
        match *ans {
            Ok(true) => self.success.push(name),
            Ok(false) => self.fail.push(name),
            Err(_) => self.error.push(name),
        }
    }
}

impl Display for Results {
    fn fmt(&self, fmter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmter.write_str(&format!("Success: {}, Fail: {}, Error: {}",
                                self.success.len(),
                                self.fail.len(),
                                self.error.len()))
    }
}

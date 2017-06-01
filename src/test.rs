use std::process::Command;
use std::path::PathBuf;
use std::io::Read;
use std::{io, fs};

use filesystem;

pub struct Test {
    name: String,
    source: PathBuf,
    temp: PathBuf,
    expected: PathBuf,
    cmd: String,
    quiet: bool,
}

impl Test {
    pub fn new(source: &str,
               temp: &str,
               expected: &str,
               command: &str,
               directory: &::std::fs::DirEntry,
               quiet: bool)
               -> Test {
        let name = directory.file_name();
        let directory = directory.path().to_path_buf();
        let command = directory.join(command);
        let cmd = fs::File::open(command)
            .and_then(|mut f| {
                          let mut cmd_str = String::new();
                          f.read_to_string(&mut cmd_str).map(move |_| cmd_str)
                      })
            .unwrap_or_else(|_| "cargo run -q".to_owned());


        Test {
            name: name.to_string_lossy().into_owned(),
            source: directory.join(source),
            temp: directory.join(temp),
            expected: directory.join(expected),
            cmd: cmd,
            quiet: quiet,
        }
    }

    pub fn run(&self) -> Result<bool, io::Error> {
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
        if ans {
            fs::remove_dir_all(&self.temp)?;
        }
        Ok(ans)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

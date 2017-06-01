use std::{fs, io};
use std::process::Command;
use std::path::Path;
use std::io::Read;

pub struct Options<'a> {
    pub source: &'a str,
    pub temp: &'a str,
    pub expected: &'a str,
    pub command: &'a str,
}

pub fn run_test(opts: &Options, directory: &Path) -> Result<bool, io::Error> {
    let source = &directory.join(opts.source);
    let temp = &directory.join(opts.temp);
    let expected = &directory.join(opts.expected);
    let command = &directory.join(opts.command);

    // Prepare temp
    let _ = fs::remove_dir_all(temp);
    recursive_copy(source, temp)?;

    // Run in temp
    let cmd = fs::File::open(command)
        .and_then(|mut f| {
                      let mut cmd_str = String::new();
                      f.read_to_string(&mut cmd_str).map(move |_| cmd_str)
                  })
        .unwrap_or_else(|_| "cargo run -q".to_owned());
    Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .current_dir(temp)
        .status()?;

    // Compare answer
    let ans = compare_dirs(temp, expected).unwrap_or(false);
    if ans {
        fs::remove_dir_all(temp)?;
    }
    Ok(ans)
}

fn recursive_copy(src: &Path, dst: &Path) -> Result<(), io::Error> {
    fs::create_dir_all(dst)?;
    for file in fs::read_dir(src)? {
        let file = file?;
        let dir = file.file_type()?.is_dir();
        let path = file.path();
        let path = path.file_name().unwrap();

        let src = &src.join(path);
        let dst = &dst.join(path);

        if dir {
            recursive_copy(src, dst)?;
        } else {
            fs::copy(src, dst)?;
        }
    }
    Ok(())
}

fn compare_dirs(src: &Path, dst: &Path) -> Result<bool, io::Error> {
    for file in fs::read_dir(src)? {
        let file = file?;
        let dir = file.file_type()?.is_dir();
        let path = file.path();
        let path = path.file_name().unwrap();

        let src = &src.join(path);
        let dst = &dst.join(path);

        if dir {
            if !compare_dirs(src, dst)? {
                return Ok(false);
            }
        } else if !compare_files(src, dst)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn compare_files(one: &Path, two: &Path) -> Result<bool, io::Error> {
    let mut buf_one = String::new();
    fs::File::open(one)?.read_to_string(&mut buf_one)?;

    let mut buf_two = String::new();
    fs::File::open(two)?.read_to_string(&mut buf_two)?;

    let ans = buf_one == buf_two;

    Ok(ans)
}

use std::{fs, io};
use std::path::Path;
use std::io::Read;

pub fn recursive_copy(src: &Path, dst: &Path) -> Result<(), io::Error> {
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

pub fn compare_dirs(src: &Path, dst: &Path) -> Result<bool, io::Error> {
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

pub fn compare_files(one: &Path, two: &Path) -> Result<bool, io::Error> {
    let mut buf_one = String::new();
    fs::File::open(one)?.read_to_string(&mut buf_one)?;

    let mut buf_two = String::new();
    fs::File::open(two)?.read_to_string(&mut buf_two)?;

    let ans = buf_one == buf_two;

    Ok(ans)
}

use std::fs;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

use thiserror;

const BASE_PATH: &str = "/sys/bus/platform/drivers/ideapad_acpi";
const VPC: &str = "VPC";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read the IdeaPad ACPI sysfs directory: {0}")]
    BasePathError(io::Error),

    #[error("IO error happened while reading the IdeaPad ACPI sysfs directory: {0}")]
    DirectoryIOError(io::Error),

    #[error("Path to an entry in the IdeaPad ACPI sysfs directory has invalid characters")]
    PathHasInvalidCharacters,

    #[error("Failed to open a sysfs file: {0}")]
    FileError(io::Error),

    #[error("'VPC...' directory was not fild in the IdeaPad ACPI sysfs directory")]
    VPCDirectoryNotFound,

    #[error("Failed to read a sysfs file: {0}")]
    ReadError(io::Error),

    #[error("Failed to parse a sysfs file: {0}")]
    FileParseError(ParseIntError),

    #[error("Expected a sysfs file to be binary, while it is not")]
    FileIsNotBinary,

    #[error("Failed to write to a sysfs file: {0}")]
    WriteError(io::Error),
}

pub enum Action {
    On,
    Off,
    Set(u8),
    Load,
    Toggle,
}

impl Action {
    pub fn perform(&self, filename: &str) -> Result<(), Error> {
        let filepath = construct_filepath(filename)?;
        match self {
            Action::On => set(&filepath, 1),
            Action::Off => set(&filepath, 0),
            Action::Set(value) => set(&filepath, *value),
            Action::Load => {
                let value = load(&filepath)?;
                println!("{}", value);
                Ok(())
            }
            Action::Toggle => toggle(&filepath),
        }
    }
}

fn construct_filepath(filename: &str) -> Result<PathBuf, Error> {
    let path: &Path = Path::new(BASE_PATH);

    let it = fs::read_dir(path).map_err(|err| Error::BasePathError(err))?;
    for dir_entry in it {
        let dirname = dir_entry
            .map_err(|err| Error::DirectoryIOError(err))?
            .file_name()
            .into_string()
            .map_err(|_| Error::PathHasInvalidCharacters)?;
        if !dirname.starts_with(VPC) {
            continue;
        }

        return Ok(path.join(dirname).join(filename));
    }

    Err(Error::VPCDirectoryNotFound)
}

fn open_file(path: &PathBuf, open_options: &fs::OpenOptions) -> Result<fs::File, Error> {
    Ok(open_options
        .open(path)
        .map_err(|err| Error::FileError(err))?)
}

fn read_from_file(file: &mut fs::File) -> Result<u8, Error> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|err| Error::ReadError(err))?;
    buf.trim().parse().map_err(|err| Error::FileParseError(err))
}

fn write_to_file(file: &mut fs::File, value: u8) -> Result<(), Error> {
    let buf = value.to_string();
    Ok(file
        .write_all(buf.as_bytes())
        .map_err(|err| Error::WriteError(err))?)
}

fn set(path: &PathBuf, value: u8) -> Result<(), Error> {
    let mut options = fs::OpenOptions::new();
    let options = options.write(true);
    let mut file = open_file(path, &options)?;
    Ok(write_to_file(&mut file, value)?)
}

fn load(path: &PathBuf) -> Result<u8, Error> {
    let mut options = fs::OpenOptions::new();
    let options = options.read(true);
    let mut file = open_file(path, &options)?;
    Ok(read_from_file(&mut file)?)
}

fn toggle(path: &PathBuf) -> Result<(), Error> {
    let mut options = fs::OpenOptions::new();
    let options = options.read(true).write(true);
    let mut file = open_file(path, &options)?;
    let content = read_from_file(&mut file)?;

    let new_content;
    if content == 0 {
        new_content = 1;
    } else if content == 1 {
        new_content = 0;
    } else {
        return Err(Error::FileIsNotBinary);
    }

    Ok(write_to_file(&mut file, new_content)?)
}

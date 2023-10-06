use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

const BASE_PATH: &str = "/sys/bus/platform/drivers/ideapad_acpi";
const VPC: &str = "VPC";

pub enum Error {
    BasePathError(io::Error),
    DirectoryIOError(io::Error),
    PathHasInvalidCharacters,
    FileError(io::Error),
    VPCDirectoryNotFound,
    ReadError(io::Error),
    WriteError(io::Error),
}

pub fn open_file(
    sysfs_item: &impl crate::SysfsItem,
    open_options: &OpenOptions,
) -> Result<File, Error> {
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

        let path = path.join(dirname).join(sysfs_item.filename());
        let file = open_options
            .open(path)
            .map_err(|err| Error::FileError(err))?;
        return Ok(file);
    }

    Err(Error::VPCDirectoryNotFound)
}

pub fn read_from_file(file: &mut File) -> Result<String, Error> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|err| Error::ReadError(err))?;
    Ok(buf)
}

pub fn write_to_file(file: &mut File, buf: &str) -> Result<(), Error> {
    Ok(file
        .write_all(buf.as_bytes())
        .map_err(|err| Error::WriteError(err))?)
}

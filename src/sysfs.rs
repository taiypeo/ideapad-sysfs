use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::Path;

const BASE_PATH: &str = "/sys/bus/platform/drivers/ideapad_acpi";
const VPC: &str = "VPC";

enum SysfsError {
    BasePathError(io::Error),
    DirectoryIOError(io::Error),
    PathHasInvalidCharacters,
    FileError(io::Error),
    VPCDirectoryNotFound,
}

fn open_sysfs_file(
    sysfs_item: impl crate::SysfsItem,
    open_options: OpenOptions,
) -> Result<File, SysfsError> {
    let path: &Path = Path::new(BASE_PATH);

    let it = fs::read_dir(path).map_err(|err| SysfsError::BasePathError(err))?;
    for dir_entry in it {
        let dirname = dir_entry
            .map_err(|err| SysfsError::DirectoryIOError(err))?
            .file_name()
            .into_string()
            .map_err(|_| SysfsError::PathHasInvalidCharacters)?;
        if !dirname.starts_with(VPC) {
            continue;
        }

        let path = path.join(dirname).join(sysfs_item.filename());
        let file = open_options
            .open(path)
            .map_err(|err| SysfsError::FileError(err))?;
        return Ok(file);
    }

    Err(SysfsError::VPCDirectoryNotFound)
}

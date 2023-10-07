use std::fs;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

const BASE_PATH: &str = "/sys/bus/platform/drivers/ideapad_acpi";
const VPC: &str = "VPC";

pub enum Error {
    BasePathError(io::Error),
    DirectoryIOError(io::Error),
    PathHasInvalidCharacters,
    FileError(io::Error),
    VPCDirectoryNotFound,
    ReadError(io::Error),
    InvalidFileContent(ParseIntError),
    FileIsNotBinary,
    WriteError(io::Error),
}

enum File {
    CameraPower,
    ConservationMode,
    FnLock,
    FanMode,
}

impl File {
    fn filename(&self) -> &'static str {
        match self {
            File::CameraPower => "camera_power",
            File::ConservationMode => "conservation_mode",
            File::FnLock => "fn_lock",
            File::FanMode => "fan_mode",
        }
    }
}

impl From<crate::ReadableSysfsItem> for File {
    fn from(item: crate::ReadableSysfsItem) -> Self {
        match item {
            crate::ReadableSysfsItem::CameraPower => File::CameraPower,
            crate::ReadableSysfsItem::ConservationMode => File::ConservationMode,
            crate::ReadableSysfsItem::FnLock => File::FnLock,
            crate::ReadableSysfsItem::FanMode => File::FanMode,
        }
    }
}

impl From<crate::BinarySysfsItem> for File {
    fn from(item: crate::BinarySysfsItem) -> Self {
        match item {
            crate::BinarySysfsItem::CameraPower => File::CameraPower,
            crate::BinarySysfsItem::ConservationMode => File::ConservationMode,
            crate::BinarySysfsItem::FnLock => File::FnLock,
        }
    }
}

impl From<crate::SettableSysfsItem> for File {
    fn from(item: crate::SettableSysfsItem) -> Self {
        match item {
            crate::SettableSysfsItem::FanMode { value: _ } => File::FanMode,
        }
    }
}

enum Action {
    On,
    Off,
    Set(u8),
    Load,
    Toggle,
}

impl Action {
    fn perform(&self, filename: &str) -> Result<(), Error> {
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

impl From<crate::Action> for Action {
    fn from(action: crate::Action) -> Self {
        match action {
            crate::Action::Toggle { sysfs_item: _ } => Action::Toggle,
            crate::Action::On { sysfs_item: _ } => Action::On,
            crate::Action::Off { sysfs_item: _ } => Action::Off,
            crate::Action::Set { sysfs_item } => match sysfs_item {
                crate::SettableSysfsItem::FanMode { value } => Action::Set(value),
            },
            crate::Action::Read { sysfs_item: _ } => Action::Load,
        }
    }
}

pub struct FileAction {
    file: File,
    action: Action,
}

impl FileAction {
    fn new(file: File, action: Action) -> FileAction {
        FileAction { file, action }
    }

    fn perform(&self) -> Result<(), Error> {
        self.action.perform(self.file.filename())
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
    buf.parse().map_err(|err| Error::InvalidFileContent(err))
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

    let mut new_content;
    if content == 0 {
        new_content = 1;
    } else if content == 1 {
        new_content = 0;
    } else {
        return Err(Error::FileIsNotBinary);
    }

    Ok(write_to_file(&mut file, new_content)?)
}

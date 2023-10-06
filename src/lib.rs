use clap::{Parser, Subcommand};
use std::fs::OpenOptions;

mod sysfs;

pub trait SysfsItem {
    fn filename(&self) -> &'static str;
}

#[derive(Debug, Subcommand)]
pub enum BinarySysfsItem {
    /// Camera power
    CameraPower,

    /// Battery conservation mode
    ConservationMode,

    /// Fn key lock
    FnLock,
}

impl SysfsItem for BinarySysfsItem {
    fn filename(&self) -> &'static str {
        match self {
            BinarySysfsItem::CameraPower => "camera_power",
            BinarySysfsItem::ConservationMode => "conservation_mode",
            BinarySysfsItem::FnLock => "fn_lock",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum SettableSysfsItem {
    /// Fan mode
    FanMode,
}

impl SysfsItem for SettableSysfsItem {
    fn filename(&self) -> &'static str {
        match self {
            SettableSysfsItem::FanMode => "fan_mode",
        }
    }
}

#[derive(Subcommand)]
pub enum Action {
    /// Toggle value
    Toggle {
        #[command(subcommand)]
        sysfs_item: BinarySysfsItem,
    },

    /// Turn on
    On {
        #[command(subcommand)]
        sysfs_item: BinarySysfsItem,
    },

    /// Turn off
    Off {
        #[command(subcommand)]
        sysfs_item: BinarySysfsItem,
    },

    /// Set
    Set {
        #[command(subcommand)]
        sysfs_item: SettableSysfsItem,
    },
}

impl Action {
    fn act(&self) -> Result<(), String> {
        match self {
            Action::Toggle { sysfs_item } => todo!(),
            Action::On { sysfs_item } => todo!(),
            Action::Off { sysfs_item } => todo!(),
            Action::Set { sysfs_item } => todo!(),
        }
    }

    fn toggle(sysfs_item: &BinarySysfsItem) -> Result<(), sysfs::Error> {
        let mut options = OpenOptions::new();
        let options = options.read(true).write(true);
        let mut file = sysfs::open_file(sysfs_item, options)?;
        let content = sysfs::read_from_file(&mut file)?;
        todo!();
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

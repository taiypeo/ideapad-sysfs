use clap::{Parser, Subcommand};

mod sysfs;

trait SysfsItem {
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

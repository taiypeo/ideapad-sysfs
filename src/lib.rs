use clap::{Parser, Subcommand};

mod sysfs;

#[derive(Subcommand)]
pub enum ReadableSysfsItem {
    /// Camera power
    CameraPower,

    /// Battery conservation mode
    ConservationMode,

    /// Fn key lock
    FnLock,

    /// Fan mode
    FanMode,
}

#[derive(Subcommand)]
pub enum BinarySysfsItem {
    /// Camera power
    CameraPower,

    /// Battery conservation mode
    ConservationMode,

    /// Fn key lock
    FnLock,
}

#[derive(Subcommand)]
pub enum SettableSysfsItem {
    /// Fan mode
    FanMode {
        #[arg(short, long)]
        value: u8,
    },
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

    /// Read
    Read {
        #[command(subcommand)]
        sysfs_item: ReadableSysfsItem,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

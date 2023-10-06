use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum BinarySysfsItem {
    /// Camera power
    CameraPower,

    /// Battery conservation mode
    ConservationMode,

    /// Fn key lock
    FnLock,
}

impl BinarySysfsItem {
    fn filename(&self) -> &'static str {
        match self {
            BinarySysfsItem::CameraPower => "camera_power",
            BinarySysfsItem::ConservationMode => "conservation_mode",
            BinarySysfsItem::FnLock => "fn_lock",
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
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

use clap::{Parser, Subcommand, ValueEnum};

mod sysfs;

// TODO: figure out how to fix this horrible enum usage

#[derive(ValueEnum, Clone, Copy)]
pub enum FanSpeeds {
    SuperSilentMode = 0,
    StandardMode = 1,
    DustCleaningMode = 2,
    EfficientThermalDissipationMode = 4,
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
        #[arg(value_enum, short, long)]
        value: FanSpeeds,
    },
}

#[derive(Subcommand)]
pub enum ReadableSysfsItem {
    #[command(flatten)]
    Binary(BinarySysfsItem),

    #[command(flatten)]
    Settable(SettableSysfsItem),
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
    action: Action,
}

impl Args {
    pub fn run(&self) -> Result<(), sysfs::Error> {
        let transformed_action = sysfs::Action::new(&self.action);
        let file_action = match &self.action {
            Action::Toggle { sysfs_item } => {
                sysfs::FileAction::new(sysfs::File::new_binary(&sysfs_item), transformed_action)
            }
            Action::On { sysfs_item } => {
                sysfs::FileAction::new(sysfs::File::new_binary(&sysfs_item), transformed_action)
            }
            Action::Off { sysfs_item } => {
                sysfs::FileAction::new(sysfs::File::new_binary(&sysfs_item), transformed_action)
            }
            Action::Set { sysfs_item } => {
                sysfs::FileAction::new(sysfs::File::new_settable(&sysfs_item), transformed_action)
            }
            Action::Read { sysfs_item } => {
                sysfs::FileAction::new(sysfs::File::new_readable(&sysfs_item), transformed_action)
            }
        };
        file_action.perform()
    }
}

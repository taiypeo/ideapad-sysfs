use clap::{Parser, Subcommand, ValueEnum};

mod sysfs;
use sysfs::{Action, Error, File, FileAction};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum FanSpeed {
    SuperSilentMode = 0,
    StandardMode = 1,
    DustCleaningMode = 2,
    EfficientThermalDissipationMode = 4,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    sysfs_item: SysfsItemCommand,
}

impl Args {
    pub fn run(&self) -> Result<(), Error> {
        match &self.sysfs_item {
            SysfsItemCommand::CameraPower { action } => {
                FileAction::new(File::CameraPower, action.get_action())
            }
            SysfsItemCommand::ConservationMode { action } => {
                FileAction::new(File::ConservationMode, action.get_action())
            }
            SysfsItemCommand::FnLock { action } => {
                FileAction::new(File::FnLock, action.get_action())
            }
            SysfsItemCommand::FanMode { action } => {
                FileAction::new(File::FanMode, action.get_action())
            }
        }
        .perform()
    }
}

#[derive(Subcommand, Debug)]
enum SysfsItemCommand {
    /// Camera power
    CameraPower {
        #[command(subcommand)]
        action: BinaryActionCommand,
    },

    /// Battery conservation mode
    ConservationMode {
        #[command(subcommand)]
        action: BinaryActionCommand,
    },

    /// Fn key lock
    FnLock {
        #[command(subcommand)]
        action: BinaryActionCommand,
    },

    /// FanMode
    FanMode {
        #[command(subcommand)]
        action: FanSpeedActionCommand,
    },
}

trait ActionCommand {
    fn get_action(&self) -> Action;
}

#[derive(Subcommand, Debug)]
enum BinaryActionCommand {
    Toggle,
    On,
    Off,
    Read,
}

impl ActionCommand for BinaryActionCommand {
    fn get_action(&self) -> Action {
        match self {
            BinaryActionCommand::Toggle => Action::Toggle,
            BinaryActionCommand::On => Action::On,
            BinaryActionCommand::Off => Action::Off,
            BinaryActionCommand::Read => Action::Load,
        }
    }
}

#[derive(Subcommand, Debug)]
enum FanSpeedActionCommand {
    Set {
        #[arg(value_enum)]
        value: FanSpeed,
    },
    Read,
}

impl ActionCommand for FanSpeedActionCommand {
    fn get_action(&self) -> Action {
        match self {
            FanSpeedActionCommand::Set { value } => Action::Set(*value as u8),
            FanSpeedActionCommand::Read => Action::Load,
        }
    }
}

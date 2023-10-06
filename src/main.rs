use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    sysfs_item: SysfsItem,
}

#[derive(Subcommand)]
enum SysfsItem {
    /// Camera power
    CameraPower {
        #[command(subcommand)]
        action: BoolAction,
    },

    /// Battery conservation mode
    ConservationMode {
        #[command(subcommand)]
        action: BoolAction,
    },

    /// Fn key lock
    FnLock {
        #[command(subcommand)]
        action: BoolAction,
    },
}

#[derive(Subcommand, Debug)]
enum BoolAction {
    /// Toggle value
    Toggle,

    /// Turn on
    On,

    /// Turn off
    Off,
}

fn main() {
    let args = Args::parse();
    match args.sysfs_item {
        SysfsItem::CameraPower { action } => println!("{:?}", action),
        SysfsItem::ConservationMode { action } => println!("{:?}", action),
        SysfsItem::FnLock { action } => println!("{:?}", action),
    }
}

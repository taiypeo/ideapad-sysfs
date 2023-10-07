use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    ideapad_sysfs::Args::parse().run().map_err(|err| err.into())
}

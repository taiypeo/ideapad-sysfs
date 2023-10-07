use clap::Parser;

fn main() {
    let args = ideapad_sysfs::Args::parse();
    ideapad_sysfs::run(args.action).unwrap();
}

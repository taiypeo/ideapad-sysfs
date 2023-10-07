use clap::Parser;

fn main() {
    let args = ideapad_sysfs::Args::parse();
    args.run().unwrap(); // TODO: have something better than .unwrap()
}

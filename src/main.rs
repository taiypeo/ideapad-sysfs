use clap::Parser;

fn main() {
    let args = ideapad_sysfs::Args::parse();
    match args.action {
        ideapad_sysfs::Action::Toggle { sysfs_item } => println!("Toggle {:?}", sysfs_item),
        ideapad_sysfs::Action::On { sysfs_item } => println!("On {:?}", sysfs_item),
        ideapad_sysfs::Action::Off { sysfs_item } => println!("Off {:?}", sysfs_item),
        ideapad_sysfs::Action::Set { sysfs_item } => println!("Set {:?}", sysfs_item),
    }
}

# ideapad-sysfs

A Lenovo Ideapad sysfs configuration tool.

Uses the sysfs files that are available on the IdeaPad 3 15ARE05, according to the
[documentation](https://github.com/torvalds/linux/blob/master/Documentation/ABI/testing/sysfs-platform-ideapad-laptop).
**Note:** `sudo` privileges are required to edit sysfs files.

## Installation
```bash
cargo install --path .
```

## Usage
```bash
ideapad-sysfs --help
```

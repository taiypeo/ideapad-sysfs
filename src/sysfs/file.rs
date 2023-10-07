pub enum File {
    CameraPower,
    ConservationMode,
    FnLock,
    FanMode,
}

impl File {
    pub fn new_readable(item: &crate::ReadableSysfsItem) -> Self {
        match item {
            crate::ReadableSysfsItem::Binary(item) => File::new_binary(item),
            crate::ReadableSysfsItem::Settable(item) => File::new_settable(item),
        }
    }

    pub fn new_binary(item: &crate::BinarySysfsItem) -> Self {
        match item {
            crate::BinarySysfsItem::CameraPower => File::CameraPower,
            crate::BinarySysfsItem::ConservationMode => File::ConservationMode,
            crate::BinarySysfsItem::FnLock => File::FnLock,
        }
    }

    pub fn new_settable(item: &crate::SettableSysfsItem) -> Self {
        match item {
            crate::SettableSysfsItem::FanMode { value: _ } => File::FanMode,
        }
    }

    pub fn filename(&self) -> &'static str {
        match self {
            File::CameraPower => "camera_power",
            File::ConservationMode => "conservation_mode",
            File::FnLock => "fn_lock",
            File::FanMode => "fan_mode",
        }
    }
}

pub enum File {
    CameraPower,
    ConservationMode,
    FnLock,
    FanMode,
}

impl File {
    pub fn filename(&self) -> &'static str {
        match self {
            File::CameraPower => "camera_power",
            File::ConservationMode => "conservation_mode",
            File::FnLock => "fn_lock",
            File::FanMode => "fan_mode",
        }
    }
}

impl From<crate::ReadableSysfsItem> for File {
    fn from(item: crate::ReadableSysfsItem) -> Self {
        match item {
            crate::ReadableSysfsItem::CameraPower => File::CameraPower,
            crate::ReadableSysfsItem::ConservationMode => File::ConservationMode,
            crate::ReadableSysfsItem::FnLock => File::FnLock,
            crate::ReadableSysfsItem::FanMode => File::FanMode,
        }
    }
}

impl From<crate::BinarySysfsItem> for File {
    fn from(item: crate::BinarySysfsItem) -> Self {
        match item {
            crate::BinarySysfsItem::CameraPower => File::CameraPower,
            crate::BinarySysfsItem::ConservationMode => File::ConservationMode,
            crate::BinarySysfsItem::FnLock => File::FnLock,
        }
    }
}

impl From<crate::SettableSysfsItem> for File {
    fn from(item: crate::SettableSysfsItem) -> Self {
        match item {
            crate::SettableSysfsItem::FanMode { value: _ } => File::FanMode,
        }
    }
}
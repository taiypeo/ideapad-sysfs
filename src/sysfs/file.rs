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

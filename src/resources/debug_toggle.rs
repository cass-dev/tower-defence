pub struct DebugToggle {
    pub should_show: bool,
    pub can_toggle: bool,
}

impl Default for DebugToggle {
    fn default() -> Self {
        DebugToggle {
            should_show: false,
            can_toggle: true,
        }
    }
}

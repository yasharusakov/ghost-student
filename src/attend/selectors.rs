/// Selectors for Zoom web client
pub mod zoom {
    pub const JOIN_BUTTON: &str = ".zoom-button.zoom-button--large.zoom-button--secondary.g7nkJFrV";
    pub const JOIN_MEETING_BUTTON: &str = ".preview-join-button";
    pub const WEBCLIENT_FRAME: &str = "#webclient";
    pub const PARTICIPANTS_COUNT: &str = ".footer-button__number-counter>span";
    pub const PRE_LEAVE_BUTTON: &str = ".footer__leave-btn-container";
    pub const LEAVE_BUTTON: &str = ".zmu-btn.leave-meeting-options__btn.leave-meeting-options__btn--default.leave-meeting-options__btn--danger.zmu-btn--default.zmu-btn__outline--white";
}

/// Selectors for Google Meet web client
pub mod google_meet {
    pub const JOIN_BUTTON: &str = ".UywwFc-vQzf8d";
    pub const PARTICIPANTS_COUNT: &str = ".egzc7c > div > div";
    pub const LEAVE_BUTTON: &str =
        ".VYBDae-Bz112c-LgbsSe.VYBDae-Bz112c-LgbsSe-OWXEXe-SfQLQb-suEOdc.hk9qKe.Iootmd.vLQezd";
}

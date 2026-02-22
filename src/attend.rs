use tokio::time::{Duration, sleep};
use crate::config::{Meeting, Platform};
use crate::scheduler;
use thirtyfour::{error::WebDriverResult, WebDriver};

mod common;
mod selectors;
mod google_meet;
mod zoom;

pub async fn attend_meeting(driver: &WebDriver, meeting: &Meeting) -> WebDriverResult<()> {
    loop {
        if scheduler::is_meeting_time(meeting) {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }

    match meeting.platform {
        Platform::GoogleMeet => google_meet::attend(driver, meeting).await,
        Platform::Zoom => zoom::attend(driver, meeting).await,
    }
}

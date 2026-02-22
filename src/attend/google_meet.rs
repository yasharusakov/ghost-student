use crate::attend::common::{attendance_loop, wait_and_click, wait_for_clickable, wait_for_element};
use crate::attend::selectors::google_meet as sel;
use crate::config::Meeting;
use thirtyfour::{By, WebDriver, error::WebDriverResult};

pub async fn attend(driver: &WebDriver, meeting: &Meeting) -> WebDriverResult<()> {
    driver.goto(&meeting.url).await?;

    // Click "Join" button on the pre-join screen
    wait_and_click(driver, By::Css(sel::JOIN_BUTTON)).await?;

    // Wait for participants count element to appear
    let participants_count_element =
        wait_for_element(driver, By::Css(sel::PARTICIPANTS_COUNT)).await?;

    let leave_button = wait_for_clickable(driver, By::Css(sel::LEAVE_BUTTON)).await?;

    attendance_loop(driver, meeting, &participants_count_element, &leave_button).await?;

    Ok(())
}
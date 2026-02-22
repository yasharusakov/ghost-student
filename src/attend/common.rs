use crate::attend::selectors::zoom as zoom_selector;
use crate::config::Meeting;
use crate::screenshot::screenshot;
use crate::{config, scheduler};
use thirtyfour::{
    By, WebDriver, WebElement,
    error::WebDriverResult,
    prelude::{ElementQueryable, ElementWaitable},
};
use tokio::time::{Duration, sleep};

pub const WAIT_TIMEOUT: Duration = Duration::from_mins(30);
pub const POLL_INTERVAL: Duration = Duration::from_millis(500);
const LOOP_INTERVAL: Duration = Duration::from_secs(10);
const MIN_PARTICIPANTS_THRESHOLD: u32 = 10;

pub async fn wait_and_click(driver: &WebDriver, by: By) -> WebDriverResult<()> {
    let el = driver
        .query(by)
        .wait(WAIT_TIMEOUT, POLL_INTERVAL)
        .first()
        .await?;
    el.wait_until().clickable().await?;
    el.click().await?;
    Ok(())
}

pub async fn wait_for_element(driver: &WebDriver, by: By) -> WebDriverResult<WebElement> {
    let el = driver
        .query(by)
        .wait(WAIT_TIMEOUT, POLL_INTERVAL)
        .first()
        .await?;
    el.wait_until().displayed().await?;
    Ok(el)
}

pub async fn wait_for_clickable(driver: &WebDriver, by: By) -> WebDriverResult<WebElement> {
    let el = driver
        .query(by)
        .wait(WAIT_TIMEOUT, POLL_INTERVAL)
        .first()
        .await?;
    el.wait_until().clickable().await?;
    Ok(el)
}

async fn leave_meeting(
    driver: &WebDriver,
    leave_button: &WebElement,
    meeting: &Meeting,
) -> WebDriverResult<()> {
    match driver
        .action_chain()
        .move_to_element_center(&leave_button)
        .click()
        .perform()
        .await
    {
        Ok(_) => {
            if let config::Platform::Zoom = meeting.platform {
                match wait_for_clickable(driver, By::Css(zoom_selector::LEAVE_BUTTON)).await {
                    Ok(confirm_leave_button) => match confirm_leave_button.click().await {
                        Ok(_) => println!("Confirmed leave meeting successfully."),
                        Err(e) => eprintln!("Failed to click confirm leave button: {}", e),
                    },
                    Err(e) => eprintln!("Failed to find confirm leave button: {}", e),
                }
            }

            println!("Clicked leave button successfully.");
        }
        Err(e) => {
            eprintln!("Failed to click leave button: {}", e);

            match driver.close_window().await {
                Ok(_) => println!("Closed meeting window successfully."),
                Err(e) => eprintln!("Failed to close meeting window: {}", e),
            }
        }
    };

    Ok(())
}

pub async fn attendance_loop(
    driver: &WebDriver,
    meeting: &Meeting,
    participants_el: &WebElement,
    leave_button: &WebElement,
) -> WebDriverResult<()> {
    let path = format!("screenshots/start_{}.png", meeting.name.replace(" ", "_"));

    match screenshot(driver, &path).await {
        Ok(_) => println!("Screenshot saved to {}", &path),
        Err(e) => eprintln!("Failed to take screenshot: {}", e),
    }

    let mut max_participants = 0;

    loop {
        if scheduler::is_meeting_over(meeting) {
            leave_meeting(driver, leave_button, meeting).await?;
            break;
        }

        match participants_el.text().await {
            Ok(text) => {
                let count: u32 = match text.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Failed to parse participants count: {}", text);
                        continue;
                    }
                };

                println!("Current participants count: {}", count);

                if count > max_participants {
                    max_participants = count;
                }

                // if count dropped by 50% from the max, leave the meeting
                // MIN_PARTICIPANTS_THRESHOLD is used to avoid leaving
                // the meeting if the max participants count is very low (e.g., 2 or 3)
                if max_participants > MIN_PARTICIPANTS_THRESHOLD && count <= max_participants / 2 {
                    println!("Participants dropped by 50%, leaving the meeting.");
                    leave_meeting(driver, leave_button, meeting).await?;
                    break;
                }
            }
            Err(e) => eprintln!("Failed to get participants count: {}", e),
        }

        sleep(LOOP_INTERVAL).await;
    }

    Ok(())
}

use crate::attend::common::{
    attendance_loop, wait_and_click, wait_for_clickable, wait_for_element,
};
use crate::attend::selectors::zoom as sel;
use crate::config::Meeting;
use thirtyfour::{By, WebDriver, error::WebDriverResult};

pub async fn attend(driver: &WebDriver, meeting: &Meeting) -> WebDriverResult<()> {
    driver.goto(&meeting.url).await?;

    // Click the initial "Join" button on the landing page
    wait_and_click(driver, By::Css(sel::JOIN_BUTTON)).await?;

    // Switch into the Zoom webclient iframe
    wait_for_element(driver, By::Css(sel::WEBCLIENT_FRAME)).await?;
    driver.enter_frame(0).await?;

    // Force show the footer to ensure we can access the participants count and leave button
    force_show_footer(driver).await?;

    // Click "Join Meeting" inside the iframe
    wait_and_click(driver, By::Css(sel::JOIN_MEETING_BUTTON)).await?;

    // Wait for participants count element to appear
    let participants_count_element =
        wait_for_element(driver, By::Css(sel::PARTICIPANTS_COUNT)).await?;

    let leave_button = wait_for_clickable(driver, By::Css(sel::PRE_LEAVE_BUTTON)).await?;

    attendance_loop(driver, meeting, &participants_count_element, &leave_button).await?;

    Ok(())
}

/// Injects CSS to force the Zoom footer to always be visible.
/// Zoom hides the footer after a few seconds of inactivity — this prevents that.
async fn force_show_footer(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .execute(
            r#"
            let style = document.createElement('style');
            style.textContent = `
                #wc-footer {
                    opacity: 1 !important;
                    visibility: visible !important;
                    display: block !important;
                }
                #wc-footer.footer__hidden {
                    transform: none !important;
                    opacity: 1 !important;
                    visibility: visible !important;
                }
                .footer__hidable {
                    opacity: 1 !important;
                    visibility: visible !important;
                }
            `;
            document.head.appendChild(style);

            let footer = document.querySelector('#wc-footer');
            if (footer) {
                footer.classList.remove('footer__hidden');
            }
        "#,
            vec![],
        )
        .await?;

    Ok(())
}

use std::path::Path;
use thirtyfour::{WebDriver, error::WebDriverResult};

pub async fn screenshot(driver: &WebDriver, path: &str) -> WebDriverResult<()> {
    driver.screenshot(Path::new(path)).await
}
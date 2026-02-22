use std::process::{Child, Command};
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver, error::WebDriverError};

const DRIVER_PORT: u16 = 9515;

pub fn kill_edge() {
    Command::new("taskkill")
        .args(["/F", "/IM", "msedge.exe"])
        .output()
        .ok();

    println!("[🔪] Edge closed");
}

pub fn start_driver() -> Result<Child, String> {
    let driver_path = "./msedgedriver.exe";

    let child = match Command::new(&driver_path)
        .arg(format!("--port={}", DRIVER_PORT))
        .spawn()
    {
        Ok(child) => child,
        Err(e) => return Err(format!("Failed to start msedgedriver: {}", e)),
    };

    println!("[🚀] msedgedriver started on port {}", DRIVER_PORT);

    Ok(child)
}

pub async fn create_driver() -> Result<WebDriver, WebDriverError> {
    let mut caps = DesiredCapabilities::edge();

    let local = std::env::var("LOCALAPPDATA").unwrap();
    let user_data = format!(r"{}\Microsoft\Edge\User Data", local);

    caps.add_arg(&format!("--user-data-dir={}", user_data))?;
    caps.add_arg("--profile-directory=Default")?;

    caps.add_arg("--disable-blink-features=AutomationControlled")?;

    caps.add_arg("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36")?;

    let url = format!("http://localhost:{}", DRIVER_PORT);
    let driver = WebDriver::new(url, caps).await?;

    Ok(driver)
}

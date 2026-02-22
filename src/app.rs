use crate::{attend, browser, config, scheduler};
use tokio::signal::ctrl_c;
use tokio::{
    fs,
    time::{Duration, sleep},
};

pub fn print_welcome_message() {
    println!("Welcome to the Ghost Student");
    println!("Author: {}", "https://github.com/yasharusakov");
    println!(
        "Project GitHub: {}",
        "https://github.com/yasharusakov/ghost-student"
    );
    println!("🦀 Made with Rust 🦀");
    println!();
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    print_welcome_message();

    let cfg = config::Config::load("./settings.json")?;

    let today_meetings = scheduler::schedule_meetings(&cfg.meetings).await;

    browser::kill_edge();

    let mut driver_process = browser::start_driver()?;

    sleep(Duration::from_secs(2)).await;

    let driver = browser::create_driver().await?;

    fs::create_dir_all("screenshots").await?;

    for meeting in today_meetings {
        match attend::attend_meeting(&driver, meeting).await {
            Ok(_) => println!("Successfully attended meeting: {}", meeting.name),
            Err(e) => eprintln!(
                "An error occurred while attending meeting {}: {}",
                meeting.name, e
            ),
        }
    }

    driver.quit().await.ok();
    driver_process.kill().ok();

    println!("Press Ctrl+C to exit");

    match ctrl_c().await {
        Ok(()) => println!("Exiting..."),
        Err(e) => eprintln!("Failed to listen for Ctrl+C: {}", e),
    }

    Ok(())
}

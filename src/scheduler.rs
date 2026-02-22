use crate::config::Meeting;
use chrono::{Datelike, Duration, Local, NaiveTime};

pub async fn schedule_meetings(meetings: &[Meeting]) -> Vec<&Meeting> {
    let today_day = Local::now().day();

    let mut today_meetings: Vec<&Meeting> =
        meetings.iter().filter(|m| m.day == today_day).collect();

    today_meetings.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    today_meetings
}

pub fn is_meeting_time(meeting: &Meeting) -> bool {
    let now = Local::now().time();

    let start = match NaiveTime::parse_from_str(&meeting.start_time, "%H:%M") {
        Ok(t) => t,
        Err(_) => return false,
    };

    // 10 minute buffer to ensure we don't miss the meeting start time
    let deadline = start + Duration::minutes(10);

    now >= start && now <= deadline
}

pub fn is_meeting_over(meeting: &Meeting) -> bool {
    let now = Local::now().time();

    let end = match NaiveTime::parse_from_str(&meeting.end_time, "%H:%M") {
        Ok(t) => t,
        Err(_) => return true,
    };

    now >= end
}

# 👻 Ghost Student

A Rust-based bot that automatically joins your online classes on **Google Meet** and **Zoom** at scheduled times — so you can just sleep.

> **Platform:** Windows 10 / 11 only · Requires Microsoft Edge

## Quick Start (for end users)

1. **Install Edge** — [download here](https://www.microsoft.com/edge)
2. **Log in to Google Meet** — open Edge and sign in at [meet.google.com](https://meet.google.com)
3. **Log in to Zoom** — open Edge and sign in at [zoom.us](https://zoom.us)
4. **Disable microphone & camera** — open Edge settings
5. Go to [Releases](https://github.com/yasharusakov/ghost-student/releases) and download the latest `ghost-student.exe`
6. Download the matching [msedgedriver.exe](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver) for your Edge version
7. Create a `settings.json` based on [settings-example.json](settings-example.json)
8. Place everything in one folder:

```
📁 any-folder/
├── ghost-student.exe
├── msedgedriver.exe
├── settings.json
└── 📁 screenshots/       ← created automatically on first run
```

9. Run `ghost-student.exe`

---

## Features

- Supports **Google Meet** and **Zoom**
- Schedules meetings by day and time from a config file
- Runs a browser via **MS Edge WebDriver** (thirtyfour)
- Automatically joins and leaves meetings based on start/end times
- Leaves early if participants drop by 50% (class is likely over)
- Takes a screenshot when joining each meeting

## Configuration

Copy `settings-example.json` to `settings.json` and fill in your meetings:

```json
{
    "meetings": [
        {
            "platform": "google-meet",
            "name": "Math Lecture",
            "day": 1,
            "start_time": "09:00",
            "end_time": "10:30",
            "url": "https://meet.google.com/xxx-xxxx-xxx"
        },
        {
            "platform": "zoom",
            "name": "Physics Lab",
            "day": 3,
            "start_time": "11:00",
            "end_time": "12:00",
            "url": "https://us05web.zoom.us/j/xxxxxxxxx"
        }
    ]
}
```

| Field | Description |
|-------|-------------|
| `platform` | `"google-meet"` or `"zoom"` |
| `name` | A label for the meeting |
| `day` | Day of the month (e.g. `19` for the 19th) |
| `start_time` | Join time in `HH:MM` format |
| `end_time` | Leave time in `HH:MM` format |
| `url` | Full meeting link |

## Development

```bash
git clone https://github.com/yasharusakov/ghost-student.git
cd ghost-student
cargo fetch   # pre-download dependencies
cargo run     # run in dev mode
```

### Project Structure

```
📁 ghost-student/
├── 📁 src/
│   ├── main.rs             ← entry point
│   ├── app.rs              ← main loop
│   ├── config.rs           ← settings parsing
│   ├── browser.rs          ← WebDriver setup
│   ├── scheduler.rs        ← meeting time logic
│   ├── screenshot.rs       ← screenshot helper
│   └── 📁 attend/
│       ├── common.rs       ← shared helpers & attendance loop
│       ├── selectors.rs    ← all CSS selectors in one place
│       ├── google_meet.rs  ← Google Meet join logic
│       └── zoom.rs         ← Zoom join logic
├── Cargo.toml
├── settings.json           ← your local config (not committed)
├── settings-example.json
└── msedgedriver.exe
```

## Building for Release

```bash
cargo build --release
```

The binary will be at `target/release/ghost-student.exe`. Copy it along with the required files into one folder:

```
📁 any-folder/
├── ghost-student.exe
├── msedgedriver.exe
├── settings.json
└── 📁 screenshots/       ← created automatically on first run
```

## Roadmap

- [ ] Enhanced logging — detailed per-session logs with timestamps (join, leave, errors)
- [ ] Telegram bot notifications — get notified about everything happening during a class

## Author

[yasharusakov](https://github.com/yasharusakov/ghost-student) — 🦀 Made with Rust 🦀

## License

See [LICENSE](LICENSE).


mod config;
mod timer;
mod ui;

use clap::Parser;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    error::Error,
    io,
    time::Duration,
};

use crate::ui::App;

/// A simple terminal-based countdown timer for perfect ramen noodles
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    after_help = "TIME FORMAT:\n  M:SS  - Minutes and seconds (e.g., 3:00 for 3 minutes)\n  SS    - Seconds only (e.g., 180 for 3 minutes)\n\nEXAMPLES:\n  ramen-timer         # Default 3 minutes\n  ramen-timer 5:00    # 5 minutes\n  ramen-timer 90      # 90 seconds"
)]
struct Args {
    /// Timer duration (e.g., "3:00" for 3 minutes, "30" for 30 seconds)
    #[arg(value_name = "TIME")]
    time: Option<String>,
}

/// Parse time string in format "M:SS" or "SS" to seconds
fn parse_time(time_str: &str) -> Result<u32, String> {
    if time_str.contains(':') {
        // Format: "M:SS"
        let parts: Vec<&str> = time_str.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid time format. Use 'M:SS' or 'SS'".to_string());
        }

        let minutes = parts[0].parse::<u32>()
            .map_err(|_| "Invalid minutes value".to_string())?;
        let seconds = parts[1].parse::<u32>()
            .map_err(|_| "Invalid seconds value".to_string())?;

        if seconds >= 60 {
            return Err("Seconds must be less than 60".to_string());
        }

        Ok(minutes * 60 + seconds)
    } else {
        // Format: "SS"
        time_str.parse::<u32>()
            .map_err(|_| "Invalid time value".to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let timer_seconds = match args.time {
        Some(time_str) => match parse_time(&time_str) {
            Ok(seconds) => seconds,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        None => config::DEFAULT_TIMER_SECONDS,
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::with_duration(timer_seconds);
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') | KeyCode::Char('p') => app.timer.toggle_pause(),
                        KeyCode::Char('r') => app.timer.reset(),
                        KeyCode::Char('h') | KeyCode::Char('?') => app.toggle_help(),
                        _ => {}
                    }
                }
            }
        }

        app.timer.update();
    }
}

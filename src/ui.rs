use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::timer::{Timer, TimerState};

pub struct App {
    pub timer: Timer,
    pub show_help: bool,
}

impl App {
    pub fn with_duration(seconds: u32) -> App {
        App {
            timer: Timer::with_duration(seconds),
            show_help: false,
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    if size.width < 25 || size.height < 10 {
        let error_text = Text::from("Terminal too small!\nMinimum: 25x10");
        let error_paragraph = Paragraph::new(error_text)
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        f.render_widget(error_paragraph, size);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(size);

    let main_block = Block::default()
        .title("Ramen Timer")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let inner_area = main_block.inner(chunks[0]);
    f.render_widget(main_block, chunks[0]);

    let time_text = match app.timer.state {
        TimerState::Finished => {
            let large_ramen = format_large_ramen();
            let mut lines = vec![];
            for line in large_ramen {
                lines.push(Line::from(vec![
                    Span::styled(
                        line,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::SLOW_BLINK),
                    )
                ]));
            }
            Text::from(lines)
        }
        TimerState::Paused => {
            let large_time = format_large_time(app.timer.remaining_seconds);
            let mut lines = vec![];
            for line in large_time {
                lines.push(Line::from(vec![
                    Span::styled(
                        line,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                ]));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(
                    "PAUSED",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ]));
            Text::from(lines)
        }
        _ => {
            let large_time = format_large_time(app.timer.remaining_seconds);
            let mut lines = vec![];
            for line in large_time {
                lines.push(Line::from(vec![
                    Span::styled(
                        line,
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                ]));
            }
            Text::from(lines)
        }
    };

    let timer_paragraph = Paragraph::new(time_text)
        .alignment(Alignment::Center);

    // Layout to vertically center the timer
    let timer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Min(10),
            Constraint::Percentage(25),
        ])
        .split(inner_area);

    f.render_widget(timer_paragraph, timer_chunks[1]);

    let controls_text = Text::from(vec![
        Line::from("Space/p: Start/Pause  |  r: Reset  |  q/Esc: Quit  |  h/?: Help"),
    ]);

    let controls_paragraph = Paragraph::new(controls_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .alignment(Alignment::Center);

    f.render_widget(controls_paragraph, chunks[1]);

    if app.show_help {
        let help_area = centered_rect(80, 80, size);
        f.render_widget(Clear, help_area);

        let help_text = Text::from(vec![
            Line::from("🍜 Ramen Timer Help 🍜"),
            Line::from(""),
            Line::from("Controls:"),
            Line::from("  Space or p    - Start/Pause timer"),
            Line::from("  r             - Reset timer to initial time"),
            Line::from("  q or Esc      - Quit application"),
            Line::from("  h or ?        - Show/hide this help"),
            Line::from(""),
            Line::from("Timer starts at the specified time and counts down."),
            Line::from("When it reaches 0:00, you'll see RAMEN!"),
            Line::from(""),
            Line::from("Press any key to close this help..."),
        ]);

        let help_paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title("Help")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow))
            )
            .alignment(Alignment::Left);

        f.render_widget(help_paragraph, help_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn format_large_time(remaining_seconds: u32) -> Vec<String> {
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;
    let time_str = format!("{:02}:{:02}", minutes, seconds);

    let mut lines = vec![];
    for row in 0..7 {
        let mut line = String::new();
        for ch in time_str.chars() {
            line.push_str(&get_large_digit(ch, row));
            line.push(' ');
        }
        lines.push(line);
    }
    lines
}

fn get_large_digit(digit: char, row: usize) -> &'static str {
    match digit {
        '0' => match row {
            0 => "████",
            1 => "█  █",
            2 => "█  █",
            3 => "█  █",
            4 => "█  █",
            5 => "█  █",
            6 => "████",
            _ => "    ",
        },
        '1' => match row {
            0 => "  █ ",
            1 => " ██ ",
            2 => "  █ ",
            3 => "  █ ",
            4 => "  █ ",
            5 => "  █ ",
            6 => "████",
            _ => "    ",
        },
        '2' => match row {
            0 => "████",
            1 => "   █",
            2 => "   █",
            3 => "████",
            4 => "█   ",
            5 => "█   ",
            6 => "████",
            _ => "    ",
        },
        '3' => match row {
            0 => "████",
            1 => "   █",
            2 => "   █",
            3 => "████",
            4 => "   █",
            5 => "   █",
            6 => "████",
            _ => "    ",
        },
        '4' => match row {
            0 => "█  █",
            1 => "█  █",
            2 => "█  █",
            3 => "████",
            4 => "   █",
            5 => "   █",
            6 => "   █",
            _ => "    ",
        },
        '5' => match row {
            0 => "████",
            1 => "█   ",
            2 => "█   ",
            3 => "████",
            4 => "   █",
            5 => "   █",
            6 => "████",
            _ => "    ",
        },
        '6' => match row {
            0 => "████",
            1 => "█   ",
            2 => "█   ",
            3 => "████",
            4 => "█  █",
            5 => "█  █",
            6 => "████",
            _ => "    ",
        },
        '7' => match row {
            0 => "████",
            1 => "   █",
            2 => "   █",
            3 => "  █ ",
            4 => "  █ ",
            5 => " █  ",
            6 => " █  ",
            _ => "    ",
        },
        '8' => match row {
            0 => "████",
            1 => "█  █",
            2 => "█  █",
            3 => "████",
            4 => "█  █",
            5 => "█  █",
            6 => "████",
            _ => "    ",
        },
        '9' => match row {
            0 => "████",
            1 => "█  █",
            2 => "█  █",
            3 => "████",
            4 => "   █",
            5 => "   █",
            6 => "████",
            _ => "    ",
        },
        ':' => match row {
            0 => "  ",
            1 => "  ",
            2 => "██",
            3 => "  ",
            4 => "██",
            5 => "  ",
            6 => "  ",
            _ => "  ",
        },
        _ => "    ",
    }
}

pub fn format_large_ramen() -> Vec<String> {
    vec![
        "██████   █████  ███    ███ ███████ ███    ██ ██".to_string(),
        "██   ██ ██   ██ ████  ████ ██      ████   ██ ██".to_string(),
        "██████  ███████ ██ ████ ██ █████   ██ ██  ██ ██".to_string(),
        "██   ██ ██   ██ ██  ██  ██ ██      ██  ██ ██   ".to_string(),
        "██   ██ ██   ██ ██      ██ ███████ ██   ████ ██".to_string(),
        "                                              ".to_string(),
        "        🍜 🍜 🍜 🍜 🍜 🍜 🍜 🍜 🍜          ".to_string(),
    ]
}

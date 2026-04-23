use std::error::Error;
use std::io;

use clocktui::app::ClockApp;
use clocktui::config;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, tty::IsTty};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

fn main() -> Result<(), Box<dyn Error>> {
    if !io::stdout().is_tty() {
        return Err("This app requires an interactive terminal (TTY)".into());
    }

    let mut app = ClockApp::new(config::from_env())?;
    let mut terminal = init_terminal()?;
    let run_result = app.run(&mut terminal);
    restore_terminal(&mut terminal)?;
    run_result?;
    Ok(())
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

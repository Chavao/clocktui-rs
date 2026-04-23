use std::io;
use std::time::Instant;

use chrono::Utc;
use chrono_tz::Tz;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::Terminal;
use ratatui::prelude::Backend;

use crate::config::{Config, TimezoneSpec};

#[derive(Clone, Debug)]
pub struct ZoneClock {
    pub(crate) label: String,
    pub(crate) tz: Tz,
}

#[derive(Debug)]
pub struct ClockApp {
    pub(crate) config: Config,
    pub(crate) primary_tz: Tz,
    pub(crate) zones: Vec<ZoneClock>,
    pub(crate) show_quit: bool,
    pub(crate) select_yes: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum AppAction {
    None,
    Redraw,
    Quit,
}

impl ClockApp {
    pub fn new(config: Config) -> Result<Self, String> {
        let primary_tz: Tz = config
            .primary_zone
            .parse()
            .map_err(|_| format!("Invalid primary timezone: {}", config.primary_zone))?;

        let zones = config
            .timezones
            .iter()
            .map(parse_timezone)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            config,
            primary_tz,
            zones,
            show_quit: false,
            select_yes: true,
        })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut last_tick = Instant::now();

        loop {
            let now = Utc::now();
            terminal.draw(|frame| self.render(frame, now))?;

            let timeout = self
                .config
                .refresh_interval
                .saturating_sub(last_tick.elapsed());

            if event::poll(timeout)?
                && let Event::Key(key) = event::read()?
            {
                if self.handle_key(key) == AppAction::Quit {
                    return Ok(());
                }
            }

            if last_tick.elapsed() >= self.config.refresh_interval {
                last_tick = Instant::now();
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> AppAction {
        if is_ctrl_q(key) && !self.show_quit {
            self.show_quit = true;
            self.select_yes = true;
            return AppAction::Redraw;
        }

        if !self.show_quit {
            return AppAction::None;
        }

        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                if self.select_yes {
                    self.select_yes = false;
                    AppAction::Redraw
                } else {
                    AppAction::None
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.select_yes {
                    self.select_yes = true;
                    AppAction::Redraw
                } else {
                    AppAction::None
                }
            }
            KeyCode::Enter => {
                if self.select_yes {
                    AppAction::Quit
                } else {
                    self.show_quit = false;
                    AppAction::Redraw
                }
            }
            KeyCode::Esc => {
                self.show_quit = false;
                AppAction::Redraw
            }
            _ => AppAction::None,
        }
    }
}

fn parse_timezone(spec: &TimezoneSpec) -> Result<ZoneClock, String> {
    let tz = spec
        .name
        .parse::<Tz>()
        .map_err(|_| format!("Invalid timezone: {}", spec.name))?;
    Ok(ZoneClock {
        label: spec.label.clone(),
        tz,
    })
}

fn is_ctrl_q(key: KeyEvent) -> bool {
    key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL)
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use crate::config::Config;

    use super::{AppAction, ClockApp};

    #[test]
    fn ctrl_q_opens_confirmation_modal() {
        let mut app = ClockApp::new(Config::default()).expect("valid config");
        let action = app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL));
        assert_eq!(action, AppAction::Redraw);
        assert!(app.show_quit);
        assert!(app.select_yes);
    }

    #[test]
    fn no_then_enter_closes_confirmation() {
        let mut app = ClockApp::new(Config::default()).expect("valid config");
        app.show_quit = true;
        app.select_yes = true;

        let action = app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        assert_eq!(action, AppAction::Redraw);
        assert!(!app.select_yes);

        let action = app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(action, AppAction::Redraw);
        assert!(!app.show_quit);
    }

    #[test]
    fn yes_then_enter_quits() {
        let mut app = ClockApp::new(Config::default()).expect("valid config");
        app.show_quit = true;
        app.select_yes = false;

        let action = app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(action, AppAction::Redraw);
        assert!(app.select_yes);

        let action = app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(action, AppAction::Quit);
    }
}

use chrono::{DateTime, Utc};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Frame, Line, Span, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

use crate::app::ClockApp;
use crate::glyphs::render_big_clock;

impl ClockApp {
    pub(crate) fn render(&self, frame: &mut Frame<'_>, now: DateTime<Utc>) {
        let area = frame.area();
        let content_area = inset(area, 2, 0);
        let primary_now = now.with_timezone(&self.primary_tz);
        let clock_lines = render_big_clock(&primary_now.format("%H:%M:%S").to_string());

        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Rgb(7, 10, 16))),
            area,
        );

        if content_area.width == 0 || content_area.height == 0 {
            return;
        }

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(clock_lines.len() as u16 + 4),
                Constraint::Length(1),
                Constraint::Length(7),
                Constraint::Min(1),
            ])
            .split(content_area);

        self.render_clock(frame, main_layout[1], &clock_lines);
        self.render_timezones(frame, main_layout[3], now);

        if self.show_quit {
            self.render_quit_modal(frame, content_area);
        }
    }

    fn render_clock(&self, frame: &mut Frame<'_>, area: Rect, lines: &[String]) {
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(80, 180, 220)))
                .title(Span::styled(
                    format!(" {} ", self.config.primary_zone),
                    Style::default().fg(Color::Rgb(190, 240, 255)),
                ))
                .style(Style::default().bg(Color::Rgb(10, 14, 24))),
            area,
        );

        let inner = inset(area, 2, 1);
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        let content_area = center_vertical(inner, lines.len() as u16);
        let shadow_area = Rect {
            x: content_area.x.saturating_add(1),
            y: content_area.y.saturating_add(1),
            width: content_area.width.saturating_sub(1),
            height: content_area.height.saturating_sub(1),
        };

        let shadow = Paragraph::new(
            lines
                .iter()
                .map(|l| Line::from(l.as_str()))
                .collect::<Vec<_>>(),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Rgb(24, 34, 44)));
        frame.render_widget(shadow, shadow_area);

        let colors = [
            Color::Rgb(247, 247, 250),
            Color::Rgb(245, 246, 252),
            Color::Rgb(231, 238, 252),
            Color::Rgb(218, 232, 252),
            Color::Rgb(202, 223, 252),
            Color::Rgb(183, 209, 252),
            Color::Rgb(167, 197, 248),
        ];

        let rendered = lines
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                Line::from(vec![Span::styled(
                    line.clone(),
                    Style::default().fg(colors[idx.min(colors.len() - 1)]),
                )])
            })
            .collect::<Vec<_>>();

        frame.render_widget(
            Paragraph::new(rendered).alignment(Alignment::Center),
            content_area,
        );
    }

    fn render_timezones(&self, frame: &mut Frame<'_>, area: Rect, now: DateTime<Utc>) {
        if self.zones.is_empty() {
            return;
        }

        let constraints = vec![Constraint::Ratio(1, self.zones.len() as u32); self.zones.len()];
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .spacing(1)
            .split(area);

        for (index, (zone, chunk)) in self.zones.iter().zip(chunks.iter()).enumerate() {
            let local = now.with_timezone(&zone.tz);
            let clock = local.format("%H:%M").to_string();
            let offset = local.format("%:z").to_string();
            let accent = zone_accent(index);

            frame.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(accent))
                    .style(Style::default().bg(Color::Rgb(18, 26, 36))),
                *chunk,
            );

            let content_area = center_vertical(inset(*chunk, 1, 1), 3);
            frame.render_widget(
                Paragraph::new(vec![
                    Line::from(Span::styled(
                        zone.label.as_str(),
                        Style::default().fg(Color::Rgb(220, 228, 236)),
                    )),
                    Line::from(Span::styled(
                        clock,
                        Style::default().fg(Color::Rgb(255, 255, 255)),
                    )),
                    Line::from(Span::styled(
                        offset,
                        Style::default().fg(Color::Rgb(148, 171, 190)),
                    )),
                ])
                .alignment(Alignment::Center),
                content_area,
            );
        }
    }

    fn render_quit_modal(&self, frame: &mut Frame<'_>, area: Rect) {
        let modal = centered_rect(46, 7, area);
        let no = if self.select_yes {
            Span::styled(" No ", Style::default().fg(Color::Rgb(205, 213, 223)))
        } else {
            Span::styled(
                " No ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(124, 234, 255)),
            )
        };

        let yes = if self.select_yes {
            Span::styled(
                " Yes ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(124, 234, 255)),
            )
        } else {
            Span::styled(" Yes ", Style::default().fg(Color::Rgb(205, 213, 223)))
        };

        frame.render_widget(Clear, modal);
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(124, 234, 255)))
                .style(Style::default().bg(Color::Rgb(12, 18, 30)))
                .title(Span::styled(
                    " Confirm Exit ",
                    Style::default().fg(Color::Rgb(200, 242, 255)),
                )),
            modal,
        );

        let content_area = center_vertical(inset(modal, 2, 1), 3);
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled(
                    "Do you want to exit?",
                    Style::default().fg(Color::Rgb(232, 238, 244)),
                )),
                Line::from(vec![Span::raw(" "), no, Span::raw("  "), yes]),
                Line::from(Span::styled(
                    "Use left/right and Enter",
                    Style::default().fg(Color::Rgb(142, 168, 186)),
                )),
            ])
            .alignment(Alignment::Center),
            content_area,
        );
    }
}

fn inset(area: Rect, x: u16, y: u16) -> Rect {
    Rect {
        x: area.x.saturating_add(x),
        y: area.y.saturating_add(y),
        width: area.width.saturating_sub(x.saturating_mul(2)),
        height: area.height.saturating_sub(y.saturating_mul(2)),
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

fn center_vertical(area: Rect, content_height: u16) -> Rect {
    if area.height <= content_height || content_height == 0 {
        return area;
    }

    let top = (area.height - content_height) / 2;
    Rect {
        x: area.x,
        y: area.y + top,
        width: area.width,
        height: content_height,
    }
}

fn zone_accent(index: usize) -> Color {
    let accents = [
        Color::Rgb(122, 210, 255),
        Color::Rgb(145, 225, 209),
        Color::Rgb(255, 199, 122),
        Color::Rgb(202, 186, 255),
        Color::Rgb(255, 148, 171),
    ];
    accents[index % accents.len()]
}

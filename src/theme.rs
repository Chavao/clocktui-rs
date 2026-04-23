use std::fs;
use std::path::PathBuf;

use ratatui::prelude::Color;
use serde::Deserialize;

use crate::config;

#[derive(Clone, Debug)]
pub struct Theme {
    pub app_background: Color,
    pub clock_panel_background: Color,
    pub clock_panel_border: Color,
    pub clock_title: Color,
    pub clock_shadow: Color,
    pub clock_digits: Vec<Color>,
    pub timezone_panel_background: Color,
    pub timezone_panel_border_accents: Vec<Color>,
    pub timezone_label: Color,
    pub timezone_time: Color,
    pub timezone_offset: Color,
    pub modal_panel_background: Color,
    pub modal_panel_border: Color,
    pub modal_title: Color,
    pub modal_question: Color,
    pub modal_hint: Color,
    pub modal_option_default: Color,
    pub modal_option_selected_foreground: Color,
    pub modal_option_selected_background: Color,
}

impl Theme {
    pub fn clock_digit_color(&self, index: usize) -> Color {
        self.clock_digits[index.min(self.clock_digits.len() - 1)]
    }

    pub fn timezone_accent_color(&self, index: usize) -> Color {
        self.timezone_panel_border_accents[index % self.timezone_panel_border_accents.len()]
    }
}

#[derive(Debug, Deserialize)]
struct RawThemeFile {
    app: RawAppTheme,
    clock: RawClockTheme,
    timezones: RawTimezonesTheme,
    modal: RawModalTheme,
}

#[derive(Debug, Deserialize)]
struct RawAppTheme {
    background: String,
}

#[derive(Debug, Deserialize)]
struct RawClockTheme {
    panel_background: String,
    panel_border: String,
    title: String,
    shadow: String,
    digits: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawTimezonesTheme {
    panel_background: String,
    accents: Vec<String>,
    label: String,
    time: String,
    offset: String,
}

#[derive(Debug, Deserialize)]
struct RawModalTheme {
    panel_background: String,
    panel_border: String,
    title: String,
    question: String,
    hint: String,
    option_default: String,
    option_selected_foreground: String,
    option_selected_background: String,
}

pub fn load(theme_name: &str) -> Result<Theme, String> {
    let path = theme_file_path(theme_name)?;
    let (contents, source_path) = match fs::read_to_string(&path) {
        Ok(contents) => (contents, path.clone()),
        Err(primary_err) => {
            let legacy_path = legacy_theme_file_path(theme_name)?;
            match fs::read_to_string(&legacy_path) {
                Ok(contents) => (contents, legacy_path),
                Err(legacy_err) if theme_name == "default" => (
                    include_str!("../themes/default/theme.toml").to_string(),
                    path,
                ),
                Err(legacy_err) => {
                    return Err(format!(
                        "Failed to read {}: {primary_err}; also failed legacy path {}: {legacy_err}",
                        path.display(),
                        legacy_path.display(),
                    ));
                }
            }
        }
    };
    parse_theme_toml(&contents).map_err(|err| format!("Invalid {}: {err}", source_path.display()))
}

fn theme_file_path(theme_name: &str) -> Result<PathBuf, String> {
    Ok(theme_file_path_from_config_dir(
        config::config_dir()?,
        theme_name,
    ))
}

fn legacy_theme_file_path(theme_name: &str) -> Result<PathBuf, String> {
    Ok(config::config_dir()?.join(theme_name).join("theme.toml"))
}

fn theme_file_path_from_config_dir(config_dir: PathBuf, theme_name: &str) -> PathBuf {
    config_dir
        .join("themes")
        .join(theme_name)
        .join("theme.toml")
}

fn parse_theme_toml(contents: &str) -> Result<Theme, String> {
    let raw: RawThemeFile = toml::from_str(contents).map_err(|err| err.to_string())?;
    let clock_digits = parse_palette("clock.digits", raw.clock.digits)?;
    if clock_digits.is_empty() {
        return Err("clock.digits must include at least 1 color".to_string());
    }

    let timezone_panel_border_accents = parse_palette("timezones.accents", raw.timezones.accents)?;
    if timezone_panel_border_accents.is_empty() {
        return Err("timezones.accents must include at least 1 color".to_string());
    }

    Ok(Theme {
        app_background: parse_color("app.background", &raw.app.background)?,
        clock_panel_background: parse_color("clock.panel_background", &raw.clock.panel_background)?,
        clock_panel_border: parse_color("clock.panel_border", &raw.clock.panel_border)?,
        clock_title: parse_color("clock.title", &raw.clock.title)?,
        clock_shadow: parse_color("clock.shadow", &raw.clock.shadow)?,
        clock_digits,
        timezone_panel_background: parse_color(
            "timezones.panel_background",
            &raw.timezones.panel_background,
        )?,
        timezone_panel_border_accents,
        timezone_label: parse_color("timezones.label", &raw.timezones.label)?,
        timezone_time: parse_color("timezones.time", &raw.timezones.time)?,
        timezone_offset: parse_color("timezones.offset", &raw.timezones.offset)?,
        modal_panel_background: parse_color("modal.panel_background", &raw.modal.panel_background)?,
        modal_panel_border: parse_color("modal.panel_border", &raw.modal.panel_border)?,
        modal_title: parse_color("modal.title", &raw.modal.title)?,
        modal_question: parse_color("modal.question", &raw.modal.question)?,
        modal_hint: parse_color("modal.hint", &raw.modal.hint)?,
        modal_option_default: parse_color("modal.option_default", &raw.modal.option_default)?,
        modal_option_selected_foreground: parse_color(
            "modal.option_selected_foreground",
            &raw.modal.option_selected_foreground,
        )?,
        modal_option_selected_background: parse_color(
            "modal.option_selected_background",
            &raw.modal.option_selected_background,
        )?,
    })
}

fn parse_palette(key: &str, values: Vec<String>) -> Result<Vec<Color>, String> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| parse_color(&format!("{key}[{index}]"), value))
        .collect()
}

fn parse_color(key: &str, value: &str) -> Result<Color, String> {
    let hex = value
        .strip_prefix('#')
        .ok_or_else(|| format!("{key} must use #RRGGBB, got {value}"))?;

    if hex.len() != 6 {
        return Err(format!("{key} must use #RRGGBB, got {value}"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| format!("{key} has an invalid red component: {value}"))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| format!("{key} has an invalid green component: {value}"))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| format!("{key} has an invalid blue component: {value}"))?;

    Ok(Color::Rgb(r, g, b))
}

#[cfg(test)]
mod tests {
    use super::{parse_theme_toml, theme_file_path_from_config_dir};
    use ratatui::prelude::Color;
    use std::path::PathBuf;

    #[test]
    fn parses_valid_theme_toml() {
        let theme = parse_theme_toml(
            r##"
            [app]
            background = "#070A10"

            [clock]
            panel_background = "#0A0E18"
            panel_border = "#50B4DC"
            title = "#BEF0FF"
            shadow = "#18222C"
            digits = ["#FFFFFF", "#EEEEEE"]

            [timezones]
            panel_background = "#121A24"
            accents = ["#7AD2FF"]
            label = "#DCE4EC"
            time = "#FFFFFF"
            offset = "#94ABBE"

            [modal]
            panel_background = "#0C121E"
            panel_border = "#7CEAFF"
            title = "#C8F2FF"
            question = "#E8EEF4"
            hint = "#8EA8BA"
            option_default = "#CDD5DF"
            option_selected_foreground = "#000000"
            option_selected_background = "#7CEAFF"
            "##,
        )
        .expect("valid theme");

        assert_eq!(theme.app_background, Color::Rgb(7, 10, 16));
        assert_eq!(theme.clock_digits.len(), 2);
    }

    #[test]
    fn builds_theme_path_under_theme_directory() {
        let path = theme_file_path_from_config_dir(PathBuf::from("/tmp/clocktui"), "dark");
        assert_eq!(path, PathBuf::from("/tmp/clocktui/themes/dark/theme.toml"));
    }
}

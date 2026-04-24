use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TimezoneSpec {
    pub label: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub primary_zone: String,
    pub refresh_interval: Duration,
    pub timezones: Vec<TimezoneSpec>,
    pub theme_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            primary_zone: "America/Sao_Paulo".to_string(),
            refresh_interval: Duration::from_secs(1),
            timezones: Vec::new(),
            theme_name: "default".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawThemeConfig {
    theme: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawTimezoneSpec {
    label: Option<String>,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawTimezoneEntry {
    Name(String),
    Detailed(RawTimezoneSpec),
}

#[derive(Debug, Deserialize)]
struct RawConfigFile {
    primary_timezone: Option<String>,
    primary_zone: Option<String>,
    refresh_interval_ms: Option<u64>,
    timezones: Option<Vec<RawTimezoneEntry>>,
    theme: Option<RawThemeConfig>,
}

pub fn from_env() -> Result<Config, String> {
    let mut cfg = load_from_file()?.unwrap_or_default();
    apply_cli_overrides(&mut cfg, std::env::args().skip(1));
    Ok(cfg)
}

pub fn config_dir() -> Result<PathBuf, String> {
    if let Some(xdg_config_home) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(xdg_config_home).join("clocktui"));
    }

    let home = std::env::var_os("HOME")
        .ok_or_else(|| "HOME environment variable is not set".to_string())?;
    Ok(PathBuf::from(home).join(".config").join("clocktui"))
}

fn apply_cli_overrides<I>(cfg: &mut Config, args: I)
where
    I: Iterator<Item = String>,
{
    let mut args = args;
    let mut cli_timezones: Vec<TimezoneSpec> = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--primary" {
            if let Some(value) = args.next() {
                cfg.primary_zone = value;
            }
        } else if let Some(value) = arg.strip_prefix("--primary=") {
            cfg.primary_zone = value.to_string();
        } else if arg == "--theme" {
            if let Some(value) = args.next() {
                cfg.theme_name = value;
            }
        } else if let Some(value) = arg.strip_prefix("--theme=") {
            cfg.theme_name = value.to_string();
        } else if arg == "--timezone" {
            if let Some(value) = args.next() {
                cli_timezones.push(TimezoneSpec::from_name(value));
            }
        } else if let Some(value) = arg.strip_prefix("--timezone=") {
            cli_timezones.push(TimezoneSpec::from_name(value.to_string()));
        }
    }
    if !cli_timezones.is_empty() {
        cfg.timezones = cli_timezones;
    }
}

fn config_file_path() -> Result<PathBuf, String> {
    Ok(config_dir()?.join("config.toml"))
}

fn load_from_file() -> Result<Option<Config>, String> {
    let path = config_file_path()?;
    if !path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(&path)
        .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;

    let raw: RawConfigFile = toml::from_str(&contents)
        .map_err(|err| format!("Failed to parse {}: {err}", path.display()))?;

    let mut cfg = Config::default();
    apply_raw_config(&mut cfg, raw);
    Ok(Some(cfg))
}

fn apply_raw_config(cfg: &mut Config, raw: RawConfigFile) {
    if let Some(primary_zone) = raw.primary_timezone.or(raw.primary_zone) {
        cfg.primary_zone = primary_zone;
    }

    if let Some(refresh_interval_ms) = raw.refresh_interval_ms {
        cfg.refresh_interval = Duration::from_millis(refresh_interval_ms.max(1));
    }

    if let Some(timezones) = raw.timezones {
        cfg.timezones = timezones.into_iter().map(TimezoneSpec::from_raw).collect();
    }

    if let Some(theme_name) = raw.theme.and_then(|theme| theme.theme) {
        cfg.theme_name = theme_name;
    }
}

impl TimezoneSpec {
    fn from_name(name: String) -> Self {
        Self {
            label: label_from_timezone_name(&name),
            name,
        }
    }

    fn from_raw(raw: RawTimezoneEntry) -> Self {
        match raw {
            RawTimezoneEntry::Name(name) => Self::from_name(name),
            RawTimezoneEntry::Detailed(spec) => {
                let label = spec
                    .label
                    .map(|value| value.trim().to_string())
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| label_from_timezone_name(&spec.name));

                Self {
                    label,
                    name: spec.name,
                }
            }
        }
    }
}

fn label_from_timezone_name(name: &str) -> String {
    name.rsplit('/').next().unwrap_or(name).replace('_', " ")
}

#[cfg(test)]
mod tests {
    use super::{Config, RawConfigFile, apply_raw_config};

    #[test]
    fn applies_theme_from_config_file() {
        let raw: RawConfigFile = toml::from_str(
            r#"
            [theme]
            theme = "sunset"
            "#,
        )
        .expect("valid toml");

        let mut cfg = Config::default();
        apply_raw_config(&mut cfg, raw);

        assert_eq!(cfg.theme_name, "sunset");
    }

    #[test]
    fn applies_refresh_interval_from_config_file() {
        let raw: RawConfigFile = toml::from_str("refresh_interval_ms = 250").expect("valid toml");
        let mut cfg = Config::default();
        apply_raw_config(&mut cfg, raw);
        assert_eq!(cfg.refresh_interval.as_millis(), 250);
    }

    #[test]
    fn applies_primary_timezone_from_new_key() {
        let raw: RawConfigFile =
            toml::from_str(r#"primary_timezone = "America/New_York""#).expect("valid toml");
        let mut cfg = Config::default();
        apply_raw_config(&mut cfg, raw);
        assert_eq!(cfg.primary_zone, "America/New_York");
    }

    #[test]
    fn applies_timezones_from_string_list() {
        let raw: RawConfigFile = toml::from_str(
            r#"
            timezones = ["Europe/London", "Asia/Tokyo"]
            "#,
        )
        .expect("valid toml");

        let mut cfg = Config::default();
        apply_raw_config(&mut cfg, raw);

        assert_eq!(cfg.timezones.len(), 2);
        assert_eq!(cfg.timezones[0].name, "Europe/London");
        assert_eq!(cfg.timezones[0].label, "London");
        assert_eq!(cfg.timezones[1].name, "Asia/Tokyo");
        assert_eq!(cfg.timezones[1].label, "Tokyo");
    }

    #[test]
    fn generates_label_when_missing_in_table_format() {
        let raw: RawConfigFile = toml::from_str(
            r#"
            [[timezones]]
            name = "America/Argentina/Buenos_Aires"
            "#,
        )
        .expect("valid toml");

        let mut cfg = Config::default();
        apply_raw_config(&mut cfg, raw);

        assert_eq!(cfg.timezones.len(), 1);
        assert_eq!(cfg.timezones[0].label, "Buenos Aires");
        assert_eq!(cfg.timezones[0].name, "America/Argentina/Buenos_Aires");
    }
}

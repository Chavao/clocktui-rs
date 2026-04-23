use std::time::Duration;

#[derive(Clone, Debug)]
pub struct TimezoneSpec {
    pub label: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub primary_zone: String,
    pub refresh_interval: Duration,
    pub timezones: Vec<TimezoneSpec>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            primary_zone: "America/Sao_Paulo".to_string(),
            refresh_interval: Duration::from_secs(1),
            timezones: vec![
                TimezoneSpec {
                    label: "Los Angeles".to_string(),
                    name: "America/Los_Angeles".to_string(),
                },
                TimezoneSpec {
                    label: "Utah".to_string(),
                    name: "America/Denver".to_string(),
                },
                TimezoneSpec {
                    label: "Texas".to_string(),
                    name: "America/Chicago".to_string(),
                },
                TimezoneSpec {
                    label: "New York".to_string(),
                    name: "America/New_York".to_string(),
                },
                TimezoneSpec {
                    label: "Rio de Janeiro".to_string(),
                    name: "America/Sao_Paulo".to_string(),
                },
            ],
        }
    }
}

pub fn from_env() -> Config {
    let mut cfg = Config::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--primary" {
            if let Some(value) = args.next() {
                cfg.primary_zone = value;
            }
        } else if let Some(value) = arg.strip_prefix("--primary=") {
            cfg.primary_zone = value.to_string();
        }
    }
    cfg
}

# clocktui-rs - A TUI clock

<img width="753" height="539" alt="image" src="https://github.com/user-attachments/assets/6492daa3-7aea-46de-9834-8116d4befe67" />

## Run

A `bin` folder must exist in `$HOME` and be in `$PATH`.

```
$ make release
$ clocktui
```
Or

```
$ make build
$ ./target/release/clocktui
```

## Timezones configuration

Timezone settings are loaded from:

`$HOME/.config/clocktui/config.toml`

Example:

```toml
primary_timezone = "America/Sao_Paulo"

timezones = [
  "America/Los_Angeles",
  "America/Denver",
  "America/Chicago",
  "America/New_York",
  "America/Sao_Paulo",
]

[theme]
theme = "default"
```

Notes:

- `primary_timezone` controls the large primary clock.
- `timezones` controls the smaller comparison clocks.

CLI:

- `--primary` overrides `primary_timezone` from config when provided.
- `--timezone` overrides `timezones` from config when provided.

Example:

```
clocktui --primary America/New_York --timezone Asia/Tokyo --timezone Europe/Lisbon
```

## Themes

Theme files are loaded from:

`$HOME/.config/clocktui/themes/<theme>/theme.toml`

### Setup

1. Install the app:

```bash
make release
```

2. The install step copies all bundled theme folders from `./themes` into:

`$HOME/.config/clocktui/themes/`

3. Select a theme in your config file:

`$HOME/.config/clocktui/config.toml`

```toml
[theme]
theme = "default"
```

You can also override at runtime:

```bash
clocktui --theme dark
```

### Available Themes

- `default`
- `dark`

## Todo

- [x] Update README to add build/install instructions
- [ ] Config for each `clock` individually

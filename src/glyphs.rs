pub const GLYPH_ROWS: usize = 7;

const ZERO: [&str; GLYPH_ROWS] = [
    " █████ ",
    "██   ██",
    "██   ██",
    "██   ██",
    "██   ██",
    "██   ██",
    " █████ ",
];
const ONE: [&str; GLYPH_ROWS] = [
    "   ██  ",
    " ████  ",
    "   ██  ",
    "   ██  ",
    "   ██  ",
    "   ██  ",
    " ██████",
];
const TWO: [&str; GLYPH_ROWS] = [
    " █████ ",
    "██   ██",
    "    ██ ",
    "  ███  ",
    " ██    ",
    "██     ",
    "███████",
];
const THREE: [&str; GLYPH_ROWS] = [
    " █████ ",
    "██   ██",
    "    ██ ",
    "  ███  ",
    "    ██ ",
    "██   ██",
    " █████ ",
];
const FOUR: [&str; GLYPH_ROWS] = [
    "██   ██",
    "██   ██",
    "██   ██",
    "███████",
    "     ██",
    "     ██",
    "     ██",
];
const FIVE: [&str; GLYPH_ROWS] = [
    "███████",
    "██     ",
    "█████  ",
    "    ██ ",
    "    ██ ",
    "██  ██ ",
    " ████  ",
];
const SIX: [&str; GLYPH_ROWS] = [
    " █████ ",
    "██     ",
    "█████  ",
    "██  ██ ",
    "██   ██",
    "██   ██",
    " █████ ",
];
const SEVEN: [&str; GLYPH_ROWS] = [
    "███████",
    "    ██ ",
    "   ██  ",
    "  ██   ",
    " ██    ",
    " ██    ",
    " ██    ",
];
const EIGHT: [&str; GLYPH_ROWS] = [
    " █████ ",
    "██   ██",
    "██   ██",
    " █████ ",
    "██   ██",
    "██   ██",
    " █████ ",
];
const NINE: [&str; GLYPH_ROWS] = [
    " █████ ",
    "██   ██",
    "██   ██",
    " ██████",
    "     ██",
    "    ██ ",
    " █████ ",
];
const COLON: [&str; GLYPH_ROWS] = ["   ", "██ ", "██ ", "   ", "██ ", "██ ", "   "];

fn glyph(ch: char) -> [&'static str; GLYPH_ROWS] {
    match ch {
        '0' => ZERO,
        '1' => ONE,
        '2' => TWO,
        '3' => THREE,
        '4' => FOUR,
        '5' => FIVE,
        '6' => SIX,
        '7' => SEVEN,
        '8' => EIGHT,
        '9' => NINE,
        ':' => COLON,
        _ => ZERO,
    }
}

pub fn render_big_clock(value: &str) -> Vec<String> {
    let mut lines = vec![String::new(); GLYPH_ROWS];
    for ch in value.chars() {
        let current = glyph(ch);
        for row in 0..GLYPH_ROWS {
            if !lines[row].is_empty() {
                lines[row].push_str("  ");
            }
            lines[row].push_str(current[row]);
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::{GLYPH_ROWS, render_big_clock};
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn clock_rows_have_consistent_width() {
        let lines = render_big_clock("10:28:04");
        assert_eq!(lines.len(), GLYPH_ROWS);
        let width = lines[0].width();
        assert!(width > 0);
        for line in lines {
            assert_eq!(line.width(), width);
        }
    }
}

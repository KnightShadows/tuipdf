// tuipdf
// ------
// A beautifully crafted, terminal-native PDF tool built in Rust.
// It aims to make compressing PDF files as fast, efficient and flexible
// as possible directly from your terminal.
//
// Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
//          Aditya Anand <aditya19study@gmail.com> (c) 2025
// Website: https://github.com/KnightShadows/tuipdf
// License: MPL-2.0 (see LICENSE file)

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct InputField {
    value: String,
    cursor: usize,
    scroll: usize,
}

impl InputField {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            scroll: 0,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn clamp_scroll(&mut self, visible_width: usize) {
        if visible_width == 0 {
            return;
        }
        if self.cursor < self.scroll {
            self.scroll = self.cursor;
        }
        if self.cursor >= self.scroll + visible_width {
            self.scroll = self.cursor - visible_width + 1;
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    return false;
                }
                let byte_pos = self.char_to_byte(self.cursor);
                self.value.insert(byte_pos, c);
                self.cursor += 1;
                true
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    let byte_pos = self.char_to_byte(self.cursor);
                    self.value.remove(byte_pos);
                }
                true
            }
            KeyCode::Delete => {
                let len = self.value.chars().count();
                if self.cursor < len {
                    let byte_pos = self.char_to_byte(self.cursor);
                    self.value.remove(byte_pos);
                }
                true
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                true
            }
            KeyCode::Right => {
                let len = self.value.chars().count();
                if self.cursor < len {
                    self.cursor += 1;
                }
                true
            }
            KeyCode::Home => {
                self.cursor = 0;
                true
            }
            KeyCode::End => {
                self.cursor = self.value.chars().count();
                true
            }
            _ => false,
        }
    }

    pub fn paste(&mut self, text: &str) {
        let trimmed = text.trim();
        let cleaned = trimmed
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .or_else(|| {
                trimmed
                    .strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
            })
            .unwrap_or(trimmed);

        if cleaned.is_empty() {
            return;
        }

        let byte_pos = self.char_to_byte(self.cursor);
        self.value.insert_str(byte_pos, cleaned);
        self.cursor += cleaned.chars().count();
    }

    fn char_to_byte(&self, char_idx: usize) -> usize {
        self.value
            .char_indices()
            .nth(char_idx)
            .map(|(b, _)| b)
            .unwrap_or(self.value.len())
    }
}

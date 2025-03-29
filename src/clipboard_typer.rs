use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

pub struct ClipboardTyper {
    clipboard: Clipboard,
    enigo: Enigo,
}

impl ClipboardTyper {
    pub fn new() -> Option<Self> {
        Some(ClipboardTyper {
            clipboard: Clipboard::new().ok()?,
            enigo: Enigo::new(&Settings::default()).ok()?,
        })
    }

    pub fn get_clipboard_text(&mut self) -> Option<String> {
        self.clipboard.get_text().ok()
    }

    pub fn type_text(&mut self, text: &str) {
        self.enigo.key(Key::Control, Direction::Release).unwrap();
        self.enigo.key(Key::Shift, Direction::Release).unwrap();

        for character in text.chars() {
            self.enigo.text(&character.to_string()).unwrap();
        }
    }

    pub fn type_clipboard_content(&mut self) {
        if let Some(text) = self.get_clipboard_text() {
            self.type_text(&text);
        }
    }
}

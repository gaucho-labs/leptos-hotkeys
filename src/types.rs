use std::fmt::{Display, Formatter, Result};

pub type Keys = Vec<String>;

#[derive(PartialEq, Hash, Eq)]
pub struct KeyboardModifiers {
    pub(crate) alt: bool,
    pub(crate) ctrl: bool,
    pub(crate) meta: bool,
    pub(crate) shift: bool,
}

impl Display for KeyboardModifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut modifiers = Vec::new();

        if self.alt {
            modifiers.push("Alt");
        }
        if self.ctrl {
            modifiers.push("Ctrl");
        }
        if self.meta {
            modifiers.push("Meta");
        }
        if self.shift {
            modifiers.push("Shift");
        }

        match modifiers.is_empty() {
            true => write!(f, ""),
            false => write!(f, ", modifiers: {}", modifiers.join(", ")),
        }
    }
}

impl Default for KeyboardModifiers {
    fn default() -> Self {
        KeyboardModifiers {
            alt: false,
            ctrl: false,
            meta: false,
            shift: false,
        }
    }
}

#[derive(PartialEq, Hash, Eq)]
pub struct Hotkey {
    pub(crate) modifiers: KeyboardModifiers,
    pub(crate) keys: Keys,
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let keys = self
            .keys
            .iter()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        match keys.is_empty() {
            true => write!(f, "{}", self.modifiers),
            false => write!(f, "keys: {} {}", keys, self.modifiers),
        }
    }
}

pub type RefType<T> = Option<T>;

pub type HotkeyEvent = Hotkey;

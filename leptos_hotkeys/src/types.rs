use std::fmt::{Display, Formatter, Result};

pub type Keys = Vec<String>;

#[derive(Debug, PartialEq, Hash, Eq, Default)]
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
            false => write!(f, "+{}", modifiers.join("+")),
        }
    }
}

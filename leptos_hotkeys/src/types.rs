use std::fmt::{Display, Formatter, Result};

pub type Keys = Vec<String>;

#[derive(Debug, PartialEq, Hash, Eq)]
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

#[derive(Debug, PartialEq, Hash, Eq)]
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
            .join("+");

        match keys.is_empty() {
            true => write!(f, "{}", self.modifiers),
            false => write!(f, "{}{}", keys, self.modifiers),
        }
    }
}

impl Hotkey {
    pub fn new(key_combination: &str) -> Self {
        let parts = key_combination
            .split('+')
            .map(str::trim)
            .collect::<Vec<&str>>();

        let mut modifiers = KeyboardModifiers::default();
        let mut keys = Vec::new();

        for part in parts {
            match part.to_lowercase().as_str() {
                "control" => modifiers.ctrl = true,
                "ctrl" => modifiers.ctrl = true,

                "alt" => modifiers.alt = true,
                "option" => modifiers.alt = true, // macos variant

                "meta" => modifiers.meta = true,
                "command" => modifiers.meta = true, // macos variant
                "cmd" => modifiers.meta = true,     // macos variant
                "super" => modifiers.meta = true,   // linux variant
                "win" => modifiers.meta = true,     // windows variant

                "shift" => modifiers.shift = true,

                key => keys.push(key.to_lowercase().to_string()),
            }
        }

        Hotkey { modifiers, keys }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_hotkeys_correctly() {
        let test_cases = vec![
            (
                "shift+r+meta".to_string(),
                Hotkey {
                    modifiers: KeyboardModifiers {
                        shift: true,
                        ctrl: false,
                        alt: false,
                        meta: true,
                    },
                    keys: vec!["r".into()],
                },
            ),
            (
                "alt + o + T".to_string(),
                Hotkey {
                    modifiers: KeyboardModifiers {
                        shift: false,
                        ctrl: false,
                        alt: true,
                        meta: false,
                    },
                    keys: vec!["o".into(), "t".into()],
                },
            ),
            (
                "control+L+ 8 + 8".to_string(),
                Hotkey {
                    modifiers: KeyboardModifiers {
                        shift: false,
                        ctrl: true,
                        alt: false,
                        meta: false,
                    },
                    keys: vec!["l".into(), "8".into(), "8".into()],
                },
            ),
            (
                "shift+ctrl+alt+t".to_string(),
                Hotkey {
                    modifiers: KeyboardModifiers {
                        shift: true,
                        ctrl: true,
                        alt: true,
                        meta: false,
                    },
                    keys: vec!["t".into()],
                },
            ),
            (
                "command+k".to_string(),
                Hotkey {
                    modifiers: KeyboardModifiers {
                        shift: false,
                        ctrl: false,
                        alt: false,
                        meta: true,
                    },
                    keys: vec!["k".into()],
                },
            ),
            (
                "cmd+k".to_string(),
                Hotkey {
                    modifiers: KeyboardModifiers {
                        shift: false,
                        ctrl: false,
                        alt: false,
                        meta: true,
                    },
                    keys: vec!["k".into()],
                },
            ),
        ];

        for (input, expected) in test_cases {
            let hotkey = Hotkey::new(&input);
            assert_eq!(hotkey, expected);
        }
    }
}

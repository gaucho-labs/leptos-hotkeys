use crate::types::Keys;
use crate::KeyboardModifiers;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Hotkey {
    pub(crate) modifiers: KeyboardModifiers,
    pub(crate) keys: Keys,
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
                "controlleft" => modifiers.ctrl = true,
                "controlright" => modifiers.ctrl = true,
                "ctrl" => modifiers.ctrl = true,
                "control" => modifiers.ctrl = true,

                "alt" => modifiers.alt = true,
                "altleft" => modifiers.alt = true,
                "altright" => modifiers.alt = true,
                "option" => modifiers.alt = true, // macos variant

                "metaleft" => modifiers.meta = true,
                "metaright" => modifiers.meta = true,
                "meta" => modifiers.meta = true,
                "command" => modifiers.meta = true, // macos variant
                "cmd" => modifiers.meta = true,     // macos variant
                "super" => modifiers.meta = true,   // linux variant
                "win" => modifiers.meta = true,     // windows variant

                "shiftleft" => modifiers.shift = true,
                "shiftright" => modifiers.shift = true,
                "shift" => modifiers.shift = true,

                key => keys.push(key.to_lowercase().to_string()),
            }
        }

        Hotkey { modifiers, keys }
    }
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
pub(crate) fn is_hotkey_match(
    hotkey: &Hotkey,
    pressed_keyset: &mut std::collections::HashMap<String, web_sys::KeyboardEvent>,
) -> bool {
    let mut modifiers_match = true;

    if hotkey.modifiers.ctrl {
        modifiers_match &= pressed_keyset.contains_key("control");
    }

    if hotkey.modifiers.shift {
        modifiers_match &= pressed_keyset.contains_key("shift");
    }

    if hotkey.modifiers.meta {
        modifiers_match &= pressed_keyset.contains_key("meta");
    }

    if hotkey.modifiers.alt {
        modifiers_match &= pressed_keyset.contains_key("alt");
    }

    if modifiers_match {
        let keys_match = hotkey.keys.iter().all(|key| {
            if let Some(event) = pressed_keyset.get_mut(key) {
                event.prevent_default();
                true
            } else {
                false
            }
        });

        modifiers_match && keys_match
    } else {
        false
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

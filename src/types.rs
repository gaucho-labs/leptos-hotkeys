pub type Keys = Vec<String>;

#[derive(PartialEq, Hash, Eq)]
pub struct KeyboardModifiers {
    pub(crate) alt: bool,
    pub(crate) ctrl: bool,
    pub(crate) meta: bool,
    pub(crate) shift: bool,
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
    // scopes: Scopes,
    pub(crate) description: String,
}

pub type RefType<T> = Option<T>;

pub type HotkeyEvent = Hotkey;

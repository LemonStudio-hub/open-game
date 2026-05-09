#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ,
    KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT,
    KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
    Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Space, Enter, Escape, Tab, Backspace, Delete,
    ShiftLeft, ShiftRight, ControlLeft, ControlRight, AltLeft, AltRight,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Minus, Equal, BracketLeft, BracketRight, Backslash, Semicolon,
    Quote, Backquote, Comma, Period, Slash,
    Home, End, PageUp, PageDown, Insert,
    NumLock, CapsLock, ScrollLock,
    Unknown,
}

impl KeyCode {
    pub fn from_code(code: &str) -> Self {
        match code {
            "KeyA" => Self::KeyA, "KeyB" => Self::KeyB, "KeyC" => Self::KeyC,
            "KeyD" => Self::KeyD, "KeyE" => Self::KeyE, "KeyF" => Self::KeyF,
            "KeyG" => Self::KeyG, "KeyH" => Self::KeyH, "KeyI" => Self::KeyI,
            "KeyJ" => Self::KeyJ, "KeyK" => Self::KeyK, "KeyL" => Self::KeyL,
            "KeyM" => Self::KeyM, "KeyN" => Self::KeyN, "KeyO" => Self::KeyO,
            "KeyP" => Self::KeyP, "KeyQ" => Self::KeyQ, "KeyR" => Self::KeyR,
            "KeyS" => Self::KeyS, "KeyT" => Self::KeyT, "KeyU" => Self::KeyU,
            "KeyV" => Self::KeyV, "KeyW" => Self::KeyW, "KeyX" => Self::KeyX,
            "KeyY" => Self::KeyY, "KeyZ" => Self::KeyZ,
            "Digit0" => Self::Digit0, "Digit1" => Self::Digit1, "Digit2" => Self::Digit2,
            "Digit3" => Self::Digit3, "Digit4" => Self::Digit4, "Digit5" => Self::Digit5,
            "Digit6" => Self::Digit6, "Digit7" => Self::Digit7, "Digit8" => Self::Digit8,
            "Digit9" => Self::Digit9,
            "ArrowUp" => Self::ArrowUp, "ArrowDown" => Self::ArrowDown,
            "ArrowLeft" => Self::ArrowLeft, "ArrowRight" => Self::ArrowRight,
            "Space" => Self::Space, "Enter" => Self::Enter, "Escape" => Self::Escape,
            "Tab" => Self::Tab, "Backspace" => Self::Backspace, "Delete" => Self::Delete,
            "ShiftLeft" => Self::ShiftLeft, "ShiftRight" => Self::ShiftRight,
            "ControlLeft" => Self::ControlLeft, "ControlRight" => Self::ControlRight,
            "AltLeft" => Self::AltLeft, "AltRight" => Self::AltRight,
            "F1" => Self::F1, "F2" => Self::F2, "F3" => Self::F3, "F4" => Self::F4,
            "F5" => Self::F5, "F6" => Self::F6, "F7" => Self::F7, "F8" => Self::F8,
            "F9" => Self::F9, "F10" => Self::F10, "F11" => Self::F11, "F12" => Self::F12,
            "Minus" => Self::Minus, "Equal" => Self::Equal,
            "BracketLeft" => Self::BracketLeft, "BracketRight" => Self::BracketRight,
            "Backslash" => Self::Backslash, "Semicolon" => Self::Semicolon,
            "Quote" => Self::Quote, "Backquote" => Self::Backquote,
            "Comma" => Self::Comma, "Period" => Self::Period, "Slash" => Self::Slash,
            "Home" => Self::Home, "End" => Self::End,
            "PageUp" => Self::PageUp, "PageDown" => Self::PageDown,
            "Insert" => Self::Insert,
            "NumLock" => Self::NumLock, "CapsLock" => Self::CapsLock,
            "ScrollLock" => Self::ScrollLock,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
    Unknown,
}

impl MouseButton {
    pub fn from_index(index: i16) -> Self {
        match index {
            0 => Self::Left,
            1 => Self::Middle,
            2 => Self::Right,
            3 => Self::Back,
            4 => Self::Forward,
            _ => Self::Unknown,
        }
    }
}

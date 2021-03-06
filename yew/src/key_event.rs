#[derive(Clone, Copy, PartialEq, Debug)]
pub enum KeyCode {
    PrintableCharacter(char),
    Unidentified,
    Alt,
    AltGraph,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Backspace,
    CapsLock,
    Clear,
    Control,
    Copy,
    CrSel,
    Cut,
    Delete,
    End,
    Enter,
    EraseEof,
    ExSel,
    Fn,
    FnLock,
    Home,
    Insert,
    Meta,
    NumLock,
    PageDown,
    PageUp,
    Paste,
    Redo,
    ScrollLock,
    Shift,
    Symbol,
    SymbolLock,
    Tab,
    Undo,
}

impl From<&str> for KeyCode {
    fn from(key: &str) -> Self {
        match key {
            "Alt" => KeyCode::Alt,
            "AltGraph" => KeyCode::AltGraph,
            "ArrowDown" => KeyCode::ArrowDown,
            "ArrowLeft" => KeyCode::ArrowLeft,
            "ArrowRight" => KeyCode::ArrowRight,
            "ArrowUp" => KeyCode::ArrowUp,
            "Backspace" => KeyCode::Backspace,
            "CapsLock" => KeyCode::CapsLock,
            "Clear" => KeyCode::Clear,
            "Control" => KeyCode::Control,
            "Copy" => KeyCode::Copy,
            "CrSel" => KeyCode::CrSel,
            "Cut" => KeyCode::Cut,
            "Delete" => KeyCode::Delete,
            "End" => KeyCode::End,
            "Enter" => KeyCode::Enter,
            "EraseEof" => KeyCode::EraseEof,
            "ExSel" => KeyCode::ExSel,
            "Fn" => KeyCode::Fn,
            "FnLock" => KeyCode::FnLock,
            "Home" => KeyCode::Home,
            "Insert" => KeyCode::Insert,
            "Meta" => KeyCode::Meta,
            "NumLock" => KeyCode::NumLock,
            "PageDown" => KeyCode::PageDown,
            "PageUp" => KeyCode::PageUp,
            "Paste" => KeyCode::Paste,
            "Redo" => KeyCode::Redo,
            "ScrollLock" => KeyCode::ScrollLock,
            "Shift" => KeyCode::Shift,
            "Symbol" => KeyCode::Symbol,
            "SymbolLock" => KeyCode::SymbolLock,
            "Tab" => KeyCode::Tab,
            "Undo" => KeyCode::Undo,
            c if c.len() == 1 => KeyCode::PrintableCharacter(c.chars().next().unwrap()),
            _ => KeyCode::Unidentified,
        }
    }
}

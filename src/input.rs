use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use tui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_input::InputRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Key {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    AltNum0,
    AltNum1,
    AltNum2,
    AltNum3,
    AltNum4,
    AltNum5,
    AltNum6,
    AltNum7,
    AltNum8,
    AltNum9,

    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab,
    Delete,
    Insert,
    Enter,
    Space,
    Tab,
    Esc,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    CtrlH,
    CtrlI,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,

    CtrlBackspace,
    CtrlLeft,
    CtrlRight,
    CtrlUp,
    CtrlDown,
    CtrlHome,
    CtrlEnd,
    CtrlPageUp,
    CtrlPageDown,
    CtrlBackTab,
    CtrlDelete,
    CtrlInsert,
    CtrlEnter,
    CtrlSpace,
    CtrlTab,
    CtrlEsc,

    AltA,
    AltB,
    AltC,
    AltD,
    AltE,
    AltF,
    AltG,
    AltH,
    AltI,
    AltJ,
    AltK,
    AltL,
    AltM,
    AltN,
    AltO,
    AltP,
    AltQ,
    AltR,
    AltS,
    AltT,
    AltU,
    AltV,
    AltW,
    AltX,
    AltY,
    AltZ,

    ShiftA,
    ShiftB,
    ShiftC,
    ShiftD,
    ShiftE,
    ShiftF,
    ShiftG,
    ShiftH,
    ShiftI,
    ShiftJ,
    ShiftK,
    ShiftL,
    ShiftM,
    ShiftN,
    ShiftO,
    ShiftP,
    ShiftQ,
    ShiftR,
    ShiftS,
    ShiftT,
    ShiftU,
    ShiftV,
    ShiftW,
    ShiftX,
    ShiftY,
    ShiftZ,

    Special(char),

    NotSupported,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_str = self
            .to_char()
            .map(|c| match c {
                ' ' => "space".into(),
                _ => c.to_string(),
            })
            .unwrap_or_else(|| {
                serde_yaml::to_value(self)
                    .ok()
                    .and_then(|v| v.as_str().map(|v| v.to_string()))
                    .unwrap_or_default()
            });

        write!(f, "{key_str}")
    }
}

impl Key {
    pub fn to_input_operation(&self) -> Option<InputOperation> {
        use InputOperation::*;
        use Key::*;

        match self {
            Backspace | CtrlH => Some(DeletePreviousCharacter),
            Delete => Some(DeleteNextCharacter),
            Tab => Some(InsertCharacter('\t')),
            Space => Some(InsertCharacter(' ')),
            Left | CtrlB => Some(GoToPreviousCharacter),
            CtrlLeft => Some(GoToPreviousWord),
            Right | CtrlF => Some(GoToNextCharacter),
            CtrlRight => Some(GoToNextWord),
            CtrlU => Some(DeleteLine),
            CtrlW => Some(DeletePreviousWord),
            CtrlDelete => Some(DeleteNextWord),
            CtrlA | Home => Some(GoToStart),
            CtrlE | End => Some(GoToEnd),
            CtrlK => Some(DeleteTillEnd),
            key => key.to_char().map(InsertCharacter),
        }
    }

    pub fn from_event(key: KeyEvent) -> Self {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                KeyCode::Char('a') => Key::CtrlA,
                KeyCode::Char('b') => Key::CtrlB,
                KeyCode::Char('c') => Key::CtrlC,
                KeyCode::Char('d') => Key::CtrlD,
                KeyCode::Char('e') => Key::CtrlE,
                KeyCode::Char('f') => Key::CtrlF,
                KeyCode::Char('g') => Key::CtrlG,
                KeyCode::Char('h') => Key::CtrlH,
                KeyCode::Char('i') => Key::CtrlI,
                KeyCode::Char('j') => Key::CtrlJ,
                KeyCode::Char('k') => Key::CtrlK,
                KeyCode::Char('l') => Key::CtrlL,
                KeyCode::Char('m') => Key::CtrlM,
                KeyCode::Char('n') => Key::CtrlN,
                KeyCode::Char('o') => Key::CtrlO,
                KeyCode::Char('p') => Key::CtrlP,
                KeyCode::Char('q') => Key::CtrlQ,
                KeyCode::Char('r') => Key::CtrlR,
                KeyCode::Char('s') => Key::CtrlS,
                KeyCode::Char('t') => Key::CtrlT,
                KeyCode::Char('u') => Key::CtrlU,
                KeyCode::Char('v') => Key::CtrlV,
                KeyCode::Char('w') => Key::CtrlW,
                KeyCode::Char('x') => Key::CtrlX,
                KeyCode::Char('y') => Key::CtrlY,
                KeyCode::Char('z') => Key::CtrlZ,
                KeyCode::Char(' ') => Key::CtrlSpace,

                KeyCode::Backspace => Key::CtrlBackspace,
                KeyCode::Left => Key::CtrlLeft,
                KeyCode::Right => Key::CtrlRight,
                KeyCode::Up => Key::CtrlUp,
                KeyCode::Down => Key::CtrlDown,
                KeyCode::Home => Key::CtrlHome,
                KeyCode::End => Key::CtrlEnd,
                KeyCode::PageUp => Key::CtrlPageUp,
                KeyCode::PageDown => Key::CtrlPageDown,
                KeyCode::BackTab => Key::CtrlBackTab,
                KeyCode::Delete => Key::CtrlDelete,
                KeyCode::Insert => Key::CtrlInsert,
                KeyCode::Enter => Key::CtrlEnter,
                KeyCode::Tab => Key::CtrlTab,
                KeyCode::Esc => Key::CtrlEsc,

                KeyCode::Char(c) => c.into(),

                _ => Key::NotSupported,
            },

            KeyModifiers::ALT => match key.code {
                KeyCode::Char('0') => Key::AltNum0,
                KeyCode::Char('1') => Key::AltNum1,
                KeyCode::Char('2') => Key::AltNum2,
                KeyCode::Char('3') => Key::AltNum3,
                KeyCode::Char('4') => Key::AltNum4,
                KeyCode::Char('5') => Key::AltNum5,
                KeyCode::Char('6') => Key::AltNum6,
                KeyCode::Char('7') => Key::AltNum7,
                KeyCode::Char('8') => Key::AltNum8,
                KeyCode::Char('9') => Key::AltNum9,

                KeyCode::Char('a') => Key::AltA,
                KeyCode::Char('b') => Key::AltB,
                KeyCode::Char('c') => Key::AltC,
                KeyCode::Char('d') => Key::AltD,
                KeyCode::Char('e') => Key::AltE,
                KeyCode::Char('f') => Key::AltF,
                KeyCode::Char('g') => Key::AltG,
                KeyCode::Char('h') => Key::AltH,
                KeyCode::Char('i') => Key::AltI,
                KeyCode::Char('j') => Key::AltJ,
                KeyCode::Char('k') => Key::AltK,
                KeyCode::Char('l') => Key::AltL,
                KeyCode::Char('m') => Key::AltM,
                KeyCode::Char('n') => Key::AltN,
                KeyCode::Char('o') => Key::AltO,
                KeyCode::Char('p') => Key::AltP,
                KeyCode::Char('q') => Key::AltQ,
                KeyCode::Char('r') => Key::AltR,
                KeyCode::Char('s') => Key::AltS,
                KeyCode::Char('t') => Key::AltT,
                KeyCode::Char('u') => Key::AltU,
                KeyCode::Char('v') => Key::AltV,
                KeyCode::Char('w') => Key::AltW,
                KeyCode::Char('x') => Key::AltX,
                KeyCode::Char('y') => Key::AltY,
                KeyCode::Char('z') => Key::AltZ,

                KeyCode::Char(c) => c.into(),

                _ => Key::NotSupported,
            },

            _ => match key.code {
                KeyCode::F(1) => Key::F1,
                KeyCode::F(2) => Key::F2,
                KeyCode::F(3) => Key::F3,
                KeyCode::F(4) => Key::F4,
                KeyCode::F(5) => Key::F5,
                KeyCode::F(6) => Key::F6,
                KeyCode::F(7) => Key::F7,
                KeyCode::F(8) => Key::F8,
                KeyCode::F(9) => Key::F9,
                KeyCode::F(10) => Key::F10,
                KeyCode::F(11) => Key::F11,
                KeyCode::F(13) => Key::F13,
                KeyCode::F(12) => Key::F13,
                KeyCode::F(14) => Key::F14,
                KeyCode::F(15) => Key::F15,
                KeyCode::F(16) => Key::F16,
                KeyCode::F(17) => Key::F17,
                KeyCode::F(18) => Key::F18,
                KeyCode::F(19) => Key::F19,
                KeyCode::F(20) => Key::F20,
                KeyCode::F(21) => Key::F21,
                KeyCode::F(22) => Key::F22,
                KeyCode::F(23) => Key::F23,
                KeyCode::F(24) => Key::F24,

                KeyCode::Backspace => Key::Backspace,
                KeyCode::Left => Key::Left,
                KeyCode::Right => Key::Right,
                KeyCode::Up => Key::Up,
                KeyCode::Down => Key::Down,
                KeyCode::Home => Key::Home,
                KeyCode::End => Key::End,
                KeyCode::PageUp => Key::PageUp,
                KeyCode::PageDown => Key::PageDown,
                KeyCode::BackTab => Key::BackTab,
                KeyCode::Delete => Key::Delete,
                KeyCode::Insert => Key::Insert,
                KeyCode::Enter => Key::Enter,
                KeyCode::Tab => Key::Tab,
                KeyCode::Esc => Key::Esc,

                KeyCode::Char(c) => c.into(),

                _ => Key::NotSupported,
            },
        }
    }

    pub fn is_alphabet(&self) -> bool {
        matches!(
            self,
            Self::A
                | Self::B
                | Self::C
                | Self::D
                | Self::E
                | Self::F
                | Self::G
                | Self::H
                | Self::I
                | Self::J
                | Self::K
                | Self::L
                | Self::M
                | Self::N
                | Self::O
                | Self::P
                | Self::Q
                | Self::R
                | Self::S
                | Self::T
                | Self::U
                | Self::V
                | Self::W
                | Self::X
                | Self::Y
                | Self::Z
                | Self::ShiftA
                | Self::ShiftB
                | Self::ShiftC
                | Self::ShiftD
                | Self::ShiftE
                | Self::ShiftF
                | Self::ShiftG
                | Self::ShiftH
                | Self::ShiftI
                | Self::ShiftJ
                | Self::ShiftK
                | Self::ShiftL
                | Self::ShiftM
                | Self::ShiftN
                | Self::ShiftO
                | Self::ShiftP
                | Self::ShiftQ
                | Self::ShiftR
                | Self::ShiftS
                | Self::ShiftT
                | Self::ShiftU
                | Self::ShiftV
                | Self::ShiftW
                | Self::ShiftX
                | Self::ShiftY
                | Self::ShiftZ
        )
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self,
            Self::Num0
                | Self::Num1
                | Self::Num2
                | Self::Num3
                | Self::Num4
                | Self::Num5
                | Self::Num6
                | Self::Num7
                | Self::Num8
                | Self::Num9
        )
    }

    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabet() || self.is_number()
    }

    pub fn is_special_character(&self) -> bool {
        matches!(&self, Self::Special(_))
    }

    pub fn is_character(&self) -> bool {
        self.is_alphanumeric() || self.is_special_character()
    }

    pub fn is_function(&self) -> bool {
        matches!(
            self,
            Self::F1
                | Self::F2
                | Self::F3
                | Self::F4
                | Self::F5
                | Self::F6
                | Self::F7
                | Self::F8
                | Self::F9
                | Self::F10
                | Self::F11
                | Self::F12
        )
    }

    pub fn is_navigation(&self) -> bool {
        matches!(
            self,
            Self::Backspace
                | Self::Left
                | Self::Right
                | Self::Up
                | Self::Down
                | Self::Home
                | Self::End
                | Self::PageUp
                | Self::PageDown
                | Self::BackTab
                | Self::Delete
                | Self::Insert
                | Self::Esc
                | Self::CtrlA
                | Self::CtrlB
                | Self::CtrlC
                | Self::CtrlD
                | Self::CtrlE
                | Self::CtrlF
                | Self::CtrlG
                | Self::CtrlH
                | Self::CtrlI
                | Self::CtrlJ
                | Self::CtrlK
                | Self::CtrlL
                | Self::CtrlM
                | Self::CtrlN
                | Self::CtrlO
                | Self::CtrlP
                | Self::CtrlQ
                | Self::CtrlR
                | Self::CtrlS
                | Self::CtrlT
                | Self::CtrlU
                | Self::CtrlV
                | Self::CtrlW
                | Self::CtrlX
                | Self::CtrlY
                | Self::CtrlZ
                | Self::CtrlBackspace
                | Self::CtrlLeft
                | Self::CtrlRight
                | Self::CtrlUp
                | Self::CtrlDown
                | Self::CtrlHome
                | Self::CtrlEnd
                | Self::CtrlPageUp
                | Self::CtrlPageDown
                | Self::CtrlBackTab
                | Self::CtrlDelete
                | Self::CtrlInsert
                | Self::CtrlEnter
                | Self::CtrlSpace
                | Self::CtrlTab
                | Self::CtrlEsc
                | Self::AltA
                | Self::AltB
                | Self::AltC
                | Self::AltD
                | Self::AltE
                | Self::AltF
                | Self::AltG
                | Self::AltH
                | Self::AltI
                | Self::AltJ
                | Self::AltK
                | Self::AltL
                | Self::AltM
                | Self::AltN
                | Self::AltO
                | Self::AltP
                | Self::AltQ
                | Self::AltR
                | Self::AltS
                | Self::AltT
                | Self::AltU
                | Self::AltV
                | Self::AltW
                | Self::AltX
                | Self::AltY
                | Self::AltZ
        )
    }

    pub fn to_char(&self) -> Option<char> {
        match self {
            Self::Num0 => Some('0'),
            Self::Num1 => Some('1'),
            Self::Num2 => Some('2'),
            Self::Num3 => Some('3'),
            Self::Num4 => Some('4'),
            Self::Num5 => Some('5'),
            Self::Num6 => Some('6'),
            Self::Num7 => Some('7'),
            Self::Num8 => Some('8'),
            Self::Num9 => Some('9'),

            Self::A => Some('a'),
            Self::B => Some('b'),
            Self::C => Some('c'),
            Self::D => Some('d'),
            Self::E => Some('e'),
            Self::F => Some('f'),
            Self::G => Some('g'),
            Self::H => Some('h'),
            Self::I => Some('i'),
            Self::J => Some('j'),
            Self::K => Some('k'),
            Self::L => Some('l'),
            Self::M => Some('m'),
            Self::N => Some('n'),
            Self::O => Some('o'),
            Self::P => Some('p'),
            Self::Q => Some('q'),
            Self::R => Some('r'),
            Self::S => Some('s'),
            Self::T => Some('t'),
            Self::U => Some('u'),
            Self::V => Some('v'),
            Self::W => Some('w'),
            Self::X => Some('x'),
            Self::Y => Some('y'),
            Self::Z => Some('z'),

            Self::ShiftA => Some('A'),
            Self::ShiftB => Some('B'),
            Self::ShiftC => Some('C'),
            Self::ShiftD => Some('D'),
            Self::ShiftE => Some('E'),
            Self::ShiftF => Some('F'),
            Self::ShiftG => Some('G'),
            Self::ShiftH => Some('H'),
            Self::ShiftI => Some('I'),
            Self::ShiftJ => Some('J'),
            Self::ShiftK => Some('K'),
            Self::ShiftL => Some('L'),
            Self::ShiftM => Some('M'),
            Self::ShiftN => Some('N'),
            Self::ShiftO => Some('O'),
            Self::ShiftP => Some('P'),
            Self::ShiftQ => Some('Q'),
            Self::ShiftR => Some('R'),
            Self::ShiftS => Some('S'),
            Self::ShiftT => Some('T'),
            Self::ShiftU => Some('U'),
            Self::ShiftV => Some('V'),
            Self::ShiftW => Some('W'),
            Self::ShiftX => Some('X'),
            Self::ShiftY => Some('Y'),
            Self::ShiftZ => Some('Z'),

            Self::Space => Some(' '),
            Self::Special(c) => Some(*c),

            _ => None,
        }
    }
}

impl From<char> for Key {
    fn from(c: char) -> Self {
        match c {
            '0' => Key::Num0,
            '1' => Key::Num1,
            '2' => Key::Num2,
            '3' => Key::Num3,
            '4' => Key::Num4,
            '5' => Key::Num5,
            '6' => Key::Num6,
            '7' => Key::Num7,
            '8' => Key::Num8,
            '9' => Key::Num9,

            'a' => Key::A,
            'b' => Key::B,
            'c' => Key::C,
            'd' => Key::D,
            'e' => Key::E,
            'f' => Key::F,
            'g' => Key::G,
            'h' => Key::H,
            'i' => Key::I,
            'j' => Key::J,
            'k' => Key::K,
            'l' => Key::L,
            'm' => Key::M,
            'n' => Key::N,
            'o' => Key::O,
            'p' => Key::P,
            'q' => Key::Q,
            'r' => Key::R,
            's' => Key::S,
            't' => Key::T,
            'u' => Key::U,
            'v' => Key::V,
            'w' => Key::W,
            'x' => Key::X,
            'y' => Key::Y,
            'z' => Key::Z,

            'A' => Key::ShiftA,
            'B' => Key::ShiftB,
            'C' => Key::ShiftC,
            'D' => Key::ShiftD,
            'E' => Key::ShiftE,
            'F' => Key::ShiftF,
            'G' => Key::ShiftG,
            'H' => Key::ShiftH,
            'I' => Key::ShiftI,
            'J' => Key::ShiftJ,
            'K' => Key::ShiftK,
            'L' => Key::ShiftL,
            'M' => Key::ShiftM,
            'N' => Key::ShiftN,
            'O' => Key::ShiftO,
            'P' => Key::ShiftP,
            'Q' => Key::ShiftQ,
            'R' => Key::ShiftR,
            'S' => Key::ShiftS,
            'T' => Key::ShiftT,
            'U' => Key::ShiftU,
            'V' => Key::ShiftV,
            'W' => Key::ShiftW,
            'X' => Key::ShiftX,
            'Y' => Key::ShiftY,
            'Z' => Key::ShiftZ,

            ' ' => Key::Space,
            '\t' => Key::Tab,
            '\n' => Key::Enter,

            c => Key::Special(c),
        }
    }
}

impl From<&str> for Key {
    fn from(string: &str) -> Self {
        if string.len() == 1 {
            string
                .chars()
                .next()
                .map(|c| c.into())
                .unwrap_or(Key::NotSupported)
        } else {
            Key::NotSupported
        }
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.to_string().cmp(&self.to_string())
    }
}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputOperation {
    SetCursor(usize),
    InsertCharacter(char),
    GoToPreviousCharacter,
    GoToNextCharacter,
    GoToPreviousWord,
    GoToNextWord,
    GoToStart,
    GoToEnd,
    DeletePreviousCharacter,
    DeleteNextCharacter,
    DeletePreviousWord,
    DeleteNextWord,
    DeleteLine,
    DeleteTillEnd,
}

impl Into<InputRequest> for InputOperation {
    fn into(self) -> InputRequest {
        match self {
            Self::SetCursor(i) => InputRequest::SetCursor(i),
            Self::InsertCharacter(c) => InputRequest::InsertChar(c),
            Self::GoToPreviousCharacter => InputRequest::GoToPrevChar,
            Self::GoToNextCharacter => InputRequest::GoToNextChar,
            Self::GoToPreviousWord => InputRequest::GoToPrevWord,
            Self::GoToNextWord => InputRequest::GoToNextWord,
            Self::GoToStart => InputRequest::GoToStart,
            Self::GoToEnd => InputRequest::GoToEnd,
            Self::DeletePreviousCharacter => InputRequest::DeletePrevChar,
            Self::DeleteNextCharacter => InputRequest::DeleteNextChar,
            Self::DeletePreviousWord => InputRequest::DeletePrevWord,
            Self::DeleteNextWord => InputRequest::DeleteNextWord,
            Self::DeleteLine => InputRequest::DeleteLine,
            Self::DeleteTillEnd => InputRequest::DeleteTillEnd,
        }
    }
}

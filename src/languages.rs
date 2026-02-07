#[derive(PartialEq)]
pub enum Language {
    /// en 🇬🇧 💂
    English,
    /// nl 🇳🇱 🧀
    Dutch,
    /// ua 🇺🇦 ✊
    Russian,
    /// ru 🇷🇺 🪆
    Ukrainian,
    /// tp 🇨🇦 🙂
    TokiPona,
}

impl Language {
    pub fn from_bytes(b: [u8; 2]) -> Self {
        match (b[0], b[1]) {
            (b'e', b'n') => Self::English,
            (b'n', b'l') => Self::Dutch,
            (b'u', b'a') => Self::Ukrainian,
            (b'r', b'u') => Self::Russian,
            (b't', b'p') => Self::TokiPona,
            _ => Self::English,
        }
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        match self {
            Self::English => [b'e', b'n'],
            Self::Dutch => [b'n', b'l'],
            Self::Ukrainian => [b'u', b'a'],
            Self::Russian => [b'r', b'u'],
            Self::TokiPona => [b't', b'p'],
        }
    }

    pub fn encoding(&self) -> &'static str {
        use Language::*;
        match self {
            Language::Russian | Ukrainian => "iso_8859_5",
            _ => "ascii",
        }
    }
}

#[derive(PartialEq)]
pub enum Language {
    /// en 🇬🇧 💂
    English,
    /// nl 🇳🇱 🧀
    Dutch,
    /// fr 🇫🇷 🥐
    French,
    /// de 🇩🇪 🥨
    German,
    /// it 🇮🇹 🍕
    Italian,
    /// pl 🇵🇱 🥟
    Polish,
    /// ru 🇷🇺 🪆
    Russian,
    // sp 🇪🇸 🐂
    Spanish,
    // tk 🇹🇷 🕌
    Turkish,
    /// ua 🇺🇦 ✊
    Ukrainian,
    /// tp 🇨🇦 🙂
    ///
    /// Keep Toki Pona last in the list of languages.
    /// It is a conlang and is hidden behind the Easter Eggs feature flag.
    TokiPona,
}

impl Language {
    pub fn from_bytes(b: [u8; 2]) -> Self {
        match b {
            [b'e', b'n'] => Self::English,
            [b'n', b'l'] => Self::Dutch,
            [b'f', b'r'] => Self::French,
            [b'd', b'e'] => Self::German,
            [b'i', b't'] => Self::Italian,
            [b'p', b'o'] => Self::Polish,
            [b'r', b'u'] => Self::Russian,
            [b's', b'p'] => Self::Spanish,
            [b't', b'p'] => Self::TokiPona,
            [b't', b'k'] => Self::Turkish,
            [b'u', b'a'] => Self::Ukrainian,
            _ => Self::English,
        }
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        match self {
            Self::English => [b'e', b'n'],
            Self::Dutch => [b'n', b'l'],
            Self::French => [b'f', b'r'],
            Self::German => [b'd', b'e'],
            Self::Italian => [b'i', b't'],
            Self::Polish => [b'p', b'o'],
            Self::Russian => [b'r', b'u'],
            Self::Spanish => [b's', b'p'],
            Self::TokiPona => [b't', b'p'],
            Self::Turkish => [b't', b'k'],
            Self::Ukrainian => [b'u', b'a'],
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

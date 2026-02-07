pub enum Language {
    English,
    Dutch,
    Russian,
    Ukrainian,
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
}

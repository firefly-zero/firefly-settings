use crate::*;

#[derive(Clone, Copy)]
pub enum Message {
    // Pages
    Language,
    Timezone,
    Time,
    Screen,
    Interface,
    Misc,

    // Languages
    English,
    Dutch,
    Russian,
}

impl Message {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::English => self.translate_english(),
            Language::Dutch => self.translate_dutch(),
            Language::Russian => self.translate_russian(),
        }
    }

    fn translate_english(&self) -> &'static str {
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::Time => "Time",
            Self::Screen => "Screen",
            Self::Interface => "Interface",
            Self::Misc => "Misc",
            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::Russian => "Russian",
        }
    }

    fn translate_dutch(&self) -> &'static str {
        match self {
            Self::Language => "Taal",
            Self::Timezone => "Tijdzone",
            Self::Time => "Tijd",
            Self::Screen => "Beeldscherm",
            Self::Interface => "Interface",
            Self::Misc => "Diversen",
            Self::English => "Engels",
            Self::Dutch => "Nederlands",
            Self::Russian => "Russisch",
        }
    }

    fn translate_russian(&self) -> &'static str {
        match self {
            Self::Language => "Язык",
            Self::Timezone => "Часовой пояс",
            Self::Time => "Время",
            Self::Screen => "Экран",
            Self::Interface => "Интерфейс",
            Self::Misc => "Разное",
            Self::English => "Английский",
            Self::Dutch => "Голландский",
            Self::Russian => "Русский",
        }
    }
}

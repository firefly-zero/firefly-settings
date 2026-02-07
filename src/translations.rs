use crate::*;

pub enum Message {
    Language,
    Timezone,
    Time,
    Screen,
    Interface,
    Misc,
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
        }
    }
}

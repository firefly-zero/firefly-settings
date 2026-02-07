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
    Ukrainian,
    Russian,
}

impl Message {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::English => self.translate_english(),
            Language::Dutch => self.translate_dutch(),
            Language::Ukrainian => self.translate_ukrainian(),
            Language::Russian => self.translate_russian(),
        }
    }

    fn translate_english(&self) -> &'static str {
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::Time => "Date and time",
            Self::Screen => "Screen",
            Self::Interface => "Interface",
            Self::Misc => "Misc",
            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::Ukrainian => "Ukrainian",
            Self::Russian => "Russian",
        }
    }

    fn translate_dutch(&self) -> &'static str {
        match self {
            Self::Language => "Taal",
            Self::Timezone => "Tijdzone",
            Self::Time => "Datum en tijd",
            Self::Screen => "Beeldscherm",
            Self::Interface => "Interface",
            Self::Misc => "Diversen",
            Self::English => "Engels",
            Self::Dutch => "Nederlands",
            Self::Ukrainian => "Oekraïens",
            Self::Russian => "Russisch",
        }
    }

    fn translate_russian(&self) -> &'static str {
        match self {
            Self::Language => "Язык",
            Self::Timezone => "Часовой пояс",
            Self::Time => "Дата и время",
            Self::Screen => "Экран",
            Self::Interface => "Интерфейс",
            Self::Misc => "Разное",
            Self::English => "Английский",
            Self::Dutch => "Голландский",
            Self::Ukrainian => "Украинский",
            Self::Russian => "Русский",
        }
    }

    fn translate_ukrainian(&self) -> &'static str {
        match self {
            Self::Language => "Мова",
            Self::Timezone => "Часовий пояс",
            Self::Time => "Дата та час",
            Self::Screen => "Екран",
            Self::Interface => "Інтерфейс",
            Self::Misc => "Різне",
            Self::English => "Англійська",
            Self::Dutch => "Голландська",
            Self::Ukrainian => "Українська",
            Self::Russian => "Російська",
        }
    }
}

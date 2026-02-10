use crate::*;

#[derive(PartialEq)]
pub enum Page {
    Language,
    DateTime,
    Interface,
    Misc,
}

impl Page {
    pub fn title(&self) -> Message {
        match self {
            Page::Language => Message::Language,
            Page::DateTime => Message::DateTime,
            Page::Interface => Message::Interface,
            Page::Misc => Message::Misc,
        }
    }

    pub fn lines(&self) -> &'static [Message] {
        match self {
            Page::Language => &[
                Message::English,
                Message::Dutch,
                Message::French,
                Message::German,
                Message::Italian,
                Message::Polish,
                Message::Russian,
                Message::Spanish,
                Message::Turkish,
                Message::Ukrainian,
                // Keep Toki Pona last in the list of languages.
                // It is a conlang without native speakers
                // and it is hidden behind the Easter Eggs feature flag.
                Message::TokiPona,
            ],
            Page::DateTime => &[
                //
                Message::Date,
                Message::Time,
                Message::Timezone,
            ],
            Page::Interface => &[
                Message::ColorScheme,
                Message::Contrast,
                Message::ScreenBrightness,
                Message::ReduceFlashing,
                Message::RotateScreen,
                Message::AutoLock,
            ],
            Page::Misc => &[
                Message::GamepadMode,
                Message::Telemetry,
                Message::EasterEggs,
                Message::ResetAll,
            ],
        }
    }

    pub fn next(&self) -> Self {
        use Page::*;
        match self {
            Language => DateTime,
            DateTime => Interface,
            Interface => Misc,
            Misc => Language,
        }
    }

    pub fn prev(&self) -> Self {
        use Page::*;
        match self {
            Language => Misc,
            DateTime => Language,
            Interface => DateTime,
            Misc => Interface,
        }
    }
}

use crate::*;

#[derive(PartialEq)]
pub enum Page {
    Language,
    Interface,
    Misc,
    SystemInfo,
}

impl Page {
    pub fn title(&self) -> Message {
        match self {
            Page::Language => Message::Language,
            Page::Interface => Message::Interface,
            Page::Misc => Message::Misc,
            Page::SystemInfo => Message::SystemInfo,
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
                Message::Romanian,
                Message::Russian,
                Message::Spanish,
                Message::Swedish,
                Message::Turkish,
                Message::Ukrainian,
                // Keep Toki Pona last in the list of languages.
                // It is a conlang without native speakers
                // and it is hidden behind the Easter Eggs feature flag.
                Message::TokiPona,
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
            Page::SystemInfo => &[
                Message::Name,
                Message::SerialNumber,
                Message::OS,
                Message::Drivers,
            ],
        }
    }

    pub fn next(&self) -> Self {
        use Page::*;
        match self {
            Language => Interface,
            Interface => Misc,
            Misc => SystemInfo,
            SystemInfo => Language,
        }
    }

    pub fn prev(&self) -> Self {
        use Page::*;
        match self {
            Language => SystemInfo,
            Interface => Language,
            Misc => Interface,
            SystemInfo => Misc,
        }
    }
}

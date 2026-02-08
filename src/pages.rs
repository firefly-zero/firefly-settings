use crate::*;

#[derive(PartialEq)]
pub enum Page {
    Language,
    Timezone,
    Time,
    Screen,
    Interface,
    Misc,
}

impl Page {
    pub fn title(&self) -> Message {
        match self {
            Page::Language => Message::Language,
            Page::Timezone => Message::Timezone,
            Page::Time => Message::Time,
            Page::Screen => Message::Screen,
            Page::Interface => Message::Interface,
            Page::Misc => Message::Misc,
        }
    }

    pub fn lines(&self) -> &'static [Message] {
        match self {
            Page::Language => &[
                Message::English,
                Message::Dutch,
                Message::Ukrainian,
                Message::Russian,
                Message::TokiPona,
            ],
            Page::Timezone => &[Message::EuropeAmsterdam],
            Page::Time => &[Message::Empty],
            Page::Screen => &[
                Message::RotateScreen,
                Message::ScreenBrightness,
                Message::ReduceFlashing,
                Message::Contrast,
            ],
            Page::Interface => &[
                //
                Message::AutoLock,
                Message::ColorScheme,
                Message::EasterEggs,
            ],
            Page::Misc => &[
                //
                Message::GamepadMode,
                Message::Telemetry,
                Message::ResetAll,
            ],
        }
    }

    pub fn next(&self) -> Self {
        use Page::*;
        match self {
            Language => Timezone,
            Timezone => Time,
            Time => Screen,
            Screen => Interface,
            Interface => Misc,
            Misc => Language,
        }
    }

    pub fn prev(&self) -> Self {
        use Page::*;
        match self {
            Language => Misc,
            Timezone => Language,
            Time => Timezone,
            Screen => Time,
            Interface => Screen,
            Misc => Interface,
        }
    }
}

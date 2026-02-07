use crate::*;

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
            Page::Timezone => &[],
            Page::Time => &[],
            Page::Screen => &[],
            Page::Interface => &[],
            Page::Misc => &[],
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Page::Language => Page::Timezone,
            Page::Timezone => Page::Time,
            Page::Time => Page::Screen,
            Page::Screen => Page::Interface,
            Page::Interface => Page::Misc,
            Page::Misc => Page::Language,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Page::Language => Page::Interface,
            Page::Timezone => Page::Language,
            Page::Time => Page::Timezone,
            Page::Screen => Page::Time,
            Page::Interface => Page::Screen,
            Page::Misc => Page::Interface,
        }
    }
}

// xp
// badges
// name

// lang
//     English
//     Dutch
// timezone
//     Europe
//     USA
//     Asia
// time

// screen
//     rotate_screen
//     screen_brightness
//     reduce_flashing
//     contrast

// speakers_volume
// headphones_volume

// theme
//     auto_lock
//     easter_eggs
//     color scheme

// misc
//     leds_brightness
//     gamepad_mode
//     telemetry

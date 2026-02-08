use firefly_rust::Color;

#[derive(Clone, Copy)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub bg: Color,
}

pub static THEMES: &[Theme] = &[
    // -- LIGHT THEMES -- //
    // Default green
    Theme {
        primary: Color::Black,
        secondary: Color::LightGray,
        accent: Color::Green,
        bg: Color::White,
    },
    // Greener green
    Theme {
        primary: Color::DarkGreen,
        secondary: Color::LightGray,
        accent: Color::Green,
        bg: Color::White,
    },
    // Orange
    Theme {
        primary: Color::Black,
        secondary: Color::Yellow,
        accent: Color::Orange,
        bg: Color::White,
    },
    // Red
    Theme {
        primary: Color::Black,
        secondary: Color::LightGray,
        accent: Color::Red,
        bg: Color::White,
    },
    // Blue
    Theme {
        primary: Color::DarkBlue,
        secondary: Color::LightGray,
        accent: Color::Blue,
        bg: Color::White,
    },
    // -- DARK THEMES -- //
    // Green
    Theme {
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Green,
        bg: Color::Black,
    },
    // Orange
    Theme {
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Orange,
        bg: Color::Black,
    },
    // Red
    Theme {
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Red,
        bg: Color::Black,
    },
    // Blue
    Theme {
        primary: Color::White,
        secondary: Color::DarkBlue,
        accent: Color::Blue,
        bg: Color::Black,
    },
];

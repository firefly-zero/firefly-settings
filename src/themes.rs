use firefly_rust::Color;

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    /// The main color of text and boxes.
    pub primary: Color,
    // The color of disable options, muted text, etc.
    pub secondary: Color,
    // The color of important elements, active options, etc.
    pub accent: Color,
    // The background color, the most contrast color to primary.
    pub bg: Color,
}

pub static THEMES: &[Theme] = &[
    // -- PRIMARY THEMES -- //
    Theme {
        name: "default",
        primary: Color::Black,
        secondary: Color::LightGray,
        accent: Color::Green,
        bg: Color::White,
    },
    Theme {
        name: "high contrast",
        primary: Color::Black,
        secondary: Color::Black,
        accent: Color::Red,
        bg: Color::White,
    },
    // -- LIGHT THEMES -- //
    Theme {
        name: "light green",
        primary: Color::DarkGreen,
        secondary: Color::LightGray,
        accent: Color::Green,
        bg: Color::White,
    },
    Theme {
        name: "light orange",
        primary: Color::Black,
        secondary: Color::Yellow,
        accent: Color::Orange,
        bg: Color::White,
    },
    Theme {
        name: "light red",
        primary: Color::Black,
        secondary: Color::LightGray,
        accent: Color::Red,
        bg: Color::White,
    },
    Theme {
        name: "light blue",
        primary: Color::DarkBlue,
        secondary: Color::LightGray,
        accent: Color::Blue,
        bg: Color::White,
    },
    Theme {
        name: "light gray",
        primary: Color::DarkGray,
        secondary: Color::LightGray,
        accent: Color::Gray,
        bg: Color::White,
    },
    // -- DARK THEMES -- //
    Theme {
        name: "dark green",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Green,
        bg: Color::Black,
    },
    Theme {
        name: "dark orange",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Orange,
        bg: Color::Black,
    },
    Theme {
        name: "dark red",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Red,
        bg: Color::Black,
    },
    Theme {
        name: "dark blue",
        primary: Color::White,
        secondary: Color::DarkBlue,
        accent: Color::Blue,
        bg: Color::Black,
    },
    Theme {
        name: "dark very blue",
        primary: Color::LightBlue,
        secondary: Color::DarkBlue,
        accent: Color::Blue,
        bg: Color::Black,
    },
    Theme {
        name: "dark gray",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::LightGray,
        bg: Color::Black,
    },
];

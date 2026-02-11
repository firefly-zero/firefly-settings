use firefly_rust::Color;

#[derive(Clone, Copy)]
pub struct ThemeInfo {
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

pub static THEMES: &[ThemeInfo] = &[
    // -- PRIMARY THEMES -- //
    ThemeInfo {
        name: "default",
        primary: Color::Black,
        secondary: Color::LightGray,
        accent: Color::Green,
        bg: Color::White,
    },
    ThemeInfo {
        name: "light contrast",
        primary: Color::Black,
        secondary: Color::Black,
        accent: Color::Red,
        bg: Color::White,
    },
    // -- LIGHT THEMES -- //
    ThemeInfo {
        name: "light green",
        primary: Color::DarkGreen,
        secondary: Color::LightGray,
        accent: Color::Green,
        bg: Color::White,
    },
    ThemeInfo {
        name: "light orange",
        primary: Color::Black,
        secondary: Color::Yellow,
        accent: Color::Orange,
        bg: Color::White,
    },
    ThemeInfo {
        name: "light red",
        primary: Color::Black,
        secondary: Color::LightGray,
        accent: Color::Red,
        bg: Color::White,
    },
    ThemeInfo {
        name: "light blue",
        primary: Color::DarkBlue,
        secondary: Color::LightGray,
        accent: Color::Blue,
        bg: Color::White,
    },
    ThemeInfo {
        name: "light gray",
        primary: Color::DarkGray,
        secondary: Color::LightGray,
        accent: Color::Gray,
        bg: Color::White,
    },
    // -- DARK THEMES -- //
    ThemeInfo {
        name: "dark green",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Green,
        bg: Color::Black,
    },
    ThemeInfo {
        name: "dark orange",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Orange,
        bg: Color::Black,
    },
    ThemeInfo {
        name: "dark red",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::Red,
        bg: Color::Black,
    },
    ThemeInfo {
        name: "dark blue",
        primary: Color::White,
        secondary: Color::DarkBlue,
        accent: Color::Blue,
        bg: Color::Black,
    },
    ThemeInfo {
        name: "dark-bu-dee",
        primary: Color::LightBlue,
        secondary: Color::DarkBlue,
        accent: Color::Blue,
        bg: Color::Black,
    },
    ThemeInfo {
        name: "dark gray",
        primary: Color::White,
        secondary: Color::DarkGray,
        accent: Color::LightGray,
        bg: Color::Black,
    },
    ThemeInfo {
        name: "dark contrast",
        primary: Color::White,
        secondary: Color::White,
        accent: Color::LightGray,
        bg: Color::Black,
    },
];

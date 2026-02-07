use firefly_rust::Color;

#[derive(Clone, Copy)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub bg: Color,
}

pub static THEMES: &[Theme] = &[Theme {
    primary: Color::Black,
    secondary: Color::LightGray,
    accent: Color::Green,
    bg: Color::White,
}];

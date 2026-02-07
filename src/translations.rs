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
    TokiPona,

    // Timezones
    EuropeAmsterdam,

    // Misc
    Empty,
    RotateScreen,
    ScreenBrightness,
    ReduceFlashing,
    Contrast,
    AutoLock,
    EasterEggs,
    ColorScheme,
    LedsBrightness,
    GamepadMode,
    Telemetry,
}

impl Message {
    pub fn translate(&self, lang: &Language) -> &'static str {
        use Language::*;
        match lang {
            English => self.translate_english(),
            Dutch => self.translate_dutch(),
            Ukrainian => self.translate_ukrainian(),
            Russian => self.translate_russian(),
            TokiPona => self.translate_toki_pona(),
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
            Self::TokiPona => "Toki Pona",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::LedsBrightness => "LEDs brightness",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
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
            Self::TokiPona => "Toki Pona",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::Empty => "",
            Self::RotateScreen => "Scherm draaien",
            Self::ScreenBrightness => "Schermhelderheid",
            Self::ReduceFlashing => "Knipperen verminderen",
            Self::Contrast => "Contrast",
            Self::AutoLock => "Automatische vergrendeling",
            Self::EasterEggs => "Easter eggs",
            Self::ColorScheme => "Kleurenschema",
            Self::LedsBrightness => "LED-helderheid",
            Self::GamepadMode => "Gamepadmodus",
            Self::Telemetry => "Telemetrie",
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
            Self::TokiPona => "Токи Пона",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::Empty => "",
            Self::RotateScreen => "перевернуть изображение",
            Self::ScreenBrightness => "яркость экрана",
            Self::ReduceFlashing => "уменьшить мигание",
            Self::Contrast => "контрастность",
            Self::AutoLock => "автоматическая блокировка",
            Self::EasterEggs => "пасхалки",
            Self::ColorScheme => "цветовая схема",
            Self::LedsBrightness => "яркость светодиодов",
            Self::GamepadMode => "режим джойстика",
            Self::Telemetry => "телеметрия",
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
            Self::TokiPona => "Токі Пона",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::Empty => "",
            Self::RotateScreen => "поворот екрана",
            Self::ScreenBrightness => "яскравість екрана",
            Self::ReduceFlashing => "зменшення миготіння",
            Self::Contrast => "контрастніст",
            Self::AutoLock => "автоматичне блокування",
            Self::EasterEggs => "пасхалки",
            Self::ColorScheme => "колірна схема",
            Self::LedsBrightness => "яскравість світлодіодів",
            Self::GamepadMode => "режим геймпада",
            Self::Telemetry => "телеметраці",
        }
    }

    fn translate_toki_pona(&self) -> &'static str {
        match self {
            Self::Language => "toki",
            Self::Timezone => "tenpo ma",
            Self::Time => "tenpo",
            Self::Screen => "supa lukin",
            Self::Interface => "pali",
            Self::Misc => "ale",
            Self::English => "toki Inli",
            Self::Dutch => "toki Netelan",
            Self::Ukrainian => "toki Ukrajini",
            Self::Russian => "toki Losi",
            Self::TokiPona => "toki Pona",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::Empty => "",
            Self::RotateScreen => "supa nasa",
            Self::ScreenBrightness => "supa suno",
            Self::ReduceFlashing => "suno lili",
            Self::Contrast => "kule alte",
            Self::AutoLock => "ilo pini kama",
            Self::EasterEggs => "kijetesantakalu",
            Self::ColorScheme => "kule",
            Self::LedsBrightness => "suno kule suno",
            Self::GamepadMode => "ilo musi",
            Self::Telemetry => "lukin alasa",
        }
    }
}

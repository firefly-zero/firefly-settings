use firefly_rust::Language;

#[derive(Clone, Copy)]
pub enum Message {
    // Pages
    Language,
    DateTime,
    Interface,
    Misc,

    // Languages
    English,
    Dutch,
    French,
    German,
    Italian,
    Polish,
    Russian,
    Spanish,
    Turkish,
    Ukrainian,
    TokiPona,

    // Misc
    Empty,
    Timezone,
    RotateScreen,
    ScreenBrightness,
    ReduceFlashing,
    Contrast,
    AutoLock,
    EasterEggs,
    ColorScheme,
    GamepadMode,
    Telemetry,
    ResetAll,
}

impl Message {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::English => self.translate_english(),
            Language::Dutch => self.translate_dutch(),
            Language::French => self.translate_french(),
            Language::German => self.translate_german(),
            Language::Italian => self.translate_italian(),
            Language::Polish => self.translate_polish(),
            Language::Russian => self.translate_russian(),
            Language::Spanish => self.translate_spanish(),
            Language::Turkish => self.translate_turkish(),
            Language::Ukrainian => self.translate_ukrainian(),
            Language::TokiPona => self.translate_toki_pona(),
        }
    }

    fn translate_english(&self) -> &'static str {
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::DateTime => "Date and time",
            Self::Interface => "Interface",
            Self::Misc => "Misc",

            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::French => "French",
            Self::German => "German",
            Self::Italian => "Italian",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_dutch(&self) -> &'static str {
        match self {
            Self::Language => "Taal",
            Self::Timezone => "Tijdzone",
            Self::DateTime => "Datum en tijd",
            Self::Interface => "Interface",
            Self::Misc => "Diversen",

            Self::English => "Engels",
            Self::Dutch => "Nederlands",
            Self::French => "French",   // TODO: translate
            Self::German => "German",   // TODO: translate
            Self::Italian => "Italian", // TODO: translate
            Self::Polish => "Polish",   // TODO: translate
            Self::Russian => "Russisch",
            Self::Spanish => "Spanish", // TODO: translate
            Self::Turkish => "Turkish", // TODO: translate
            Self::Ukrainian => "Oekraiens",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "scherm draaien",
            Self::ScreenBrightness => "schermhelderheid",
            Self::ReduceFlashing => "knipperen verminderen",
            Self::Contrast => "hoog contrast",
            Self::AutoLock => "automatische vergrendeling",
            Self::EasterEggs => "paaseieren",
            Self::ColorScheme => "kleurenschema",
            Self::GamepadMode => "gamepadmodus",
            Self::Telemetry => "telemetrie",
            Self::ResetAll => "alle instellingen resetten",
        }
    }

    fn translate_french(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::DateTime => "Date and time",
            Self::Interface => "Interface",
            Self::Misc => "Misc",

            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::French => "French",
            Self::German => "German",
            Self::Italian => "Italian",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_german(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::DateTime => "Date and time",
            Self::Interface => "Interface",
            Self::Misc => "Misc",

            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::French => "French",
            Self::German => "German",
            Self::Italian => "Italian",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_italian(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::DateTime => "Date and time",
            Self::Interface => "Interface",
            Self::Misc => "Misc",

            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::French => "French",
            Self::German => "German",
            Self::Italian => "Italian",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_polish(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::DateTime => "Date and time",
            Self::Interface => "Interface",
            Self::Misc => "Misc",

            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::French => "French",
            Self::German => "German",
            Self::Italian => "Italian",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_russian(&self) -> &'static str {
        match self {
            Self::Language => "Язык",
            Self::Timezone => "Часовой пояс",
            Self::DateTime => "Дата и время",
            Self::Interface => "Интерфейс",
            Self::Misc => "Разное",

            Self::English => "Английский",
            Self::Dutch => "Голландский",
            Self::French => "French",   // TODO: translate
            Self::German => "German",   // TODO: translate
            Self::Italian => "Italian", // TODO: translate
            Self::Polish => "Polish",   // TODO: translate
            Self::Russian => "Русский",
            Self::Spanish => "Spanish", // TODO: translate
            Self::Turkish => "Turkish", // TODO: translate
            Self::Ukrainian => "Украинский",
            Self::TokiPona => "Токи Пона",

            Self::Empty => "",
            Self::RotateScreen => "перевернуть изображение",
            Self::ScreenBrightness => "яркость экрана",
            Self::ReduceFlashing => "уменьшить мигание",
            Self::Contrast => "контрастность",
            Self::AutoLock => "автоматическая блокировка",
            Self::EasterEggs => "пасхалки",
            Self::ColorScheme => "цветовая схема",
            Self::GamepadMode => "режим джойстика",
            Self::Telemetry => "телеметрия",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_spanish(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::DateTime => "Date and time",
            Self::Interface => "Interface",
            Self::Misc => "Misc",

            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::French => "French",
            Self::German => "German",
            Self::Italian => "Italian",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_turkish(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::Language => "Language",
            Self::Timezone => "Timezone",
            Self::DateTime => "Date and time",
            Self::Interface => "Interface",
            Self::Misc => "Misc",

            Self::English => "English",
            Self::Dutch => "Dutch",
            Self::French => "French",
            Self::German => "German",
            Self::Italian => "Italian",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "rotate screen",
            Self::ScreenBrightness => "screen brightness",
            Self::ReduceFlashing => "reduce flashing",
            Self::Contrast => "high contrast",
            Self::AutoLock => "auto lock",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "color scheme",
            Self::GamepadMode => "gamepad mode",
            Self::Telemetry => "telemetry",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_ukrainian(&self) -> &'static str {
        match self {
            Self::Language => "Мова",
            Self::Timezone => "Часовий пояс",
            Self::DateTime => "Дата та час",
            Self::Interface => "Інтерфейс",
            Self::Misc => "Різне",

            Self::English => "Англійська",
            Self::Dutch => "Голландська",
            Self::French => "French",   // TODO: translate
            Self::German => "German",   // TODO: translate
            Self::Italian => "Italian", // TODO: translate
            Self::Polish => "Polish",   // TODO: translate
            Self::Russian => "Російська",
            Self::Spanish => "Spanish", // TODO: translate
            Self::Turkish => "Turkish", // TODO: translate
            Self::Ukrainian => "Українська",
            Self::TokiPona => "Токі Пона",

            Self::Empty => "",
            Self::RotateScreen => "поворот екрана",
            Self::ScreenBrightness => "яскравість екрана",
            Self::ReduceFlashing => "зменшення миготіння",
            Self::Contrast => "контрастніст",
            Self::AutoLock => "автоматичне блокування",
            Self::EasterEggs => "пасхалки",
            Self::ColorScheme => "колірна схема",
            Self::GamepadMode => "режим геймпада",
            Self::Telemetry => "телеметраці",
            Self::ResetAll => "reset all settings",
        }
    }

    fn translate_toki_pona(&self) -> &'static str {
        match self {
            Self::Language => "toki",
            Self::Timezone => "tenpo ma",
            Self::DateTime => "tenpo",
            Self::Interface => "namako",
            Self::Misc => "ante",

            Self::English => "toki Inli",
            Self::Dutch => "toki Netelan",
            Self::French => "French",   // TODO: translate
            Self::German => "German",   // TODO: translate
            Self::Italian => "Italian", // TODO: translate
            Self::Polish => "Polish",   // TODO: translate
            Self::Russian => "toki Losi",
            Self::Spanish => "Spanish", // TODO: translate
            Self::Turkish => "Turkish", // TODO: translate
            Self::Ukrainian => "toki Ukrajini",
            Self::TokiPona => "toki Pona",

            Self::Empty => "",
            Self::RotateScreen => "supa nasa",
            Self::ScreenBrightness => "supa suno",
            Self::ReduceFlashing => "suno lili",
            Self::Contrast => "kule alte",
            Self::AutoLock => "ilo pini kama",
            Self::EasterEggs => "kijetesantakalu",
            Self::ColorScheme => "kule",
            Self::GamepadMode => "ilo musi",
            Self::Telemetry => "lukin alasa",
            Self::ResetAll => "ale li sin",
        }
    }
}

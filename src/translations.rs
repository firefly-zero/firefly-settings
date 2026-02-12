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
    Swedish,
    Turkish,
    Ukrainian,
    TokiPona,

    // Misc
    Date,
    Time,
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
            Language::Swedish => self.translate_swedish(),
            Language::Turkish => self.translate_turkish(),
            Language::Ukrainian => self.translate_ukrainian(),
            Language::TokiPona => self.translate_toki_pona(),
        }
    }

    fn translate_english(&self) -> &'static str {
        match self {
            Self::Language => "Language",
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
            Self::Swedish => "Swedish",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::TokiPona => "Toki Pona",

            Self::Date => "date",
            Self::Time => "time",
            Self::Timezone => "timezone",
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
            Self::DateTime => "Datum en tijd",
            Self::Interface => "Interface",
            Self::Misc => "Diversen",

            Self::English => "Engels",
            Self::Dutch => "Nederlands",
            Self::French => "Frans",
            Self::German => "Duits",
            Self::Italian => "Italiaans",
            Self::Polish => "Pools",
            Self::Russian => "Russisch",
            Self::Spanish => "Spaans",
            Self::Swedish => "Zweeds",
            Self::Turkish => "Turks",
            Self::Ukrainian => "Oekraiens",
            Self::TokiPona => "Toki Pona",

            Self::Date => "datum",
            Self::Time => "tijd",
            Self::Timezone => "tijdzone",
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
        // TODO: Manually validate and correct the machine translation.
        match self {
            Self::Language => "Langue",
            Self::DateTime => "Date et heure",
            Self::Interface => "Interface",
            Self::Misc => "Divers",

            Self::English => "Anglais",
            Self::Dutch => "Néerlandais",
            Self::French => "Français",
            Self::German => "Allemand",
            Self::Italian => "Italien",
            Self::Polish => "Polonais",
            Self::Russian => "Russe",
            Self::Spanish => "Espagnol",
            Self::Swedish => "Suédois",
            Self::Turkish => "Turc",
            Self::Ukrainian => "Ukrainien",
            Self::TokiPona => "Toki Pona",

            Self::Date => "date",
            Self::Time => "heure",
            Self::Timezone => "fuseau horaire",
            Self::RotateScreen => "rotation de l'écran",
            Self::ScreenBrightness => "luminosité de l'écran",
            Self::ReduceFlashing => "réduire le clignotement",
            Self::Contrast => "contraste élevé",
            Self::AutoLock => "verrouillage automatique",
            Self::EasterEggs => "fonctionnalités cachées",
            Self::ColorScheme => "schéma de couleurs",
            Self::GamepadMode => "mode manette",
            Self::Telemetry => "télémétrie",
            Self::ResetAll => "réinitialiser tous les paramètres",
        }
    }

    fn translate_german(&self) -> &'static str {
        match self {
            Self::Language => "Sprache",
            Self::DateTime => "Datum und Uhrzeit",
            Self::Interface => "Schnittstelle",
            Self::Misc => "Verschiedenes",

            Self::English => "Englisch",
            Self::Dutch => "Niederländisch",
            Self::French => "Französisch",
            Self::German => "Deutsch",
            Self::Italian => "Italienisch",
            Self::Polish => "Polnisch",
            Self::Russian => "Russisch",
            Self::Spanish => "Spanisch",
            Self::Swedish => "Schwedisch",
            Self::Turkish => "Türkisch",
            Self::Ukrainian => "Ukrainisch",
            Self::TokiPona => "Toki Pona",

            Self::Date => "Datum",
            Self::Time => "Uhrzeit",
            Self::Timezone => "Zeitzone",
            Self::RotateScreen => "Bildschirm drehen",
            Self::ScreenBrightness => "Bildschirmhelligkeit",
            Self::ReduceFlashing => "Blitzlichter vermeiden",
            Self::Contrast => "Hoher Kontrast",
            Self::AutoLock => "Auto Lock",
            Self::EasterEggs => "Easter Eggs",
            Self::ColorScheme => "Farbschema",
            Self::GamepadMode => "Gamepadmodus",
            Self::Telemetry => "Telemetrie",
            Self::ResetAll => "Alle Einstellungen zurücksetzen",
        }
    }

    fn translate_italian(&self) -> &'static str {
        // TODO: Manually validate and correct the machine translation.
        match self {
            Self::Language => "Lingua",
            Self::DateTime => "Data e ora",
            Self::Interface => "Interfaccia",
            Self::Misc => "Varie",

            Self::English => "Inglese",
            Self::Dutch => "Olandese",
            Self::French => "Francese",
            Self::German => "Tedesco",
            Self::Italian => "Italiano",
            Self::Polish => "Polacco",
            Self::Russian => "Russo",
            Self::Spanish => "Spagnolo",
            Self::Swedish => "Svedese",
            Self::Turkish => "Turco",
            Self::Ukrainian => "Ucraino",
            Self::TokiPona => "Toki Pona",

            Self::Date => "data",
            Self::Time => "ora",
            Self::Timezone => "fuso orario",
            Self::RotateScreen => "ruota schermo",
            Self::ScreenBrightness => "luminosità schermo",
            Self::ReduceFlashing => "riduci lampeggiamento",
            Self::Contrast => "contrasto elevato",
            Self::AutoLock => "blocco automatico",
            Self::EasterEggs => "easter egg",
            Self::ColorScheme => "combinazione colori",
            Self::GamepadMode => "modalità gamepad",
            Self::Telemetry => "telemetria",
            Self::ResetAll => "ripristina tutte le impostazioni",
        }
    }

    fn translate_polish(&self) -> &'static str {
        match self {
            Self::Language => "Język",
            Self::DateTime => "Data i czas",
            Self::Interface => "Interfejs",
            Self::Misc => "Pozostałe",

            Self::English => "angielski",
            Self::Dutch => "holenderski",
            Self::French => "francuski",
            Self::German => "niemiecki",
            Self::Italian => "włoski",
            Self::Polish => "polski",
            Self::Russian => "rosyjski",
            Self::Spanish => "hiszpański",
            Self::Swedish => "szwedzki",
            Self::Turkish => "turecki",
            Self::Ukrainian => "ukraiński",
            Self::TokiPona => "toki pona",

            Self::Date => "data",
            Self::Time => "czas",
            Self::Timezone => "strefa czasowa",
            Self::RotateScreen => "obróć ekran",
            Self::ScreenBrightness => "jasność ekranu",
            Self::ReduceFlashing => "unikaj migotania ekranu",
            Self::Contrast => "wysoki kontrast",
            Self::AutoLock => "blokuj automatycznie",
            Self::EasterEggs => "easter eggi",
            Self::ColorScheme => "motyw kolorów",
            Self::GamepadMode => "tryb gamepada",
            Self::Telemetry => "telemetria",
            Self::ResetAll => "zresetuj ustawienia",
        }
    }

    fn translate_russian(&self) -> &'static str {
        match self {
            Self::Language => "Язык",
            Self::DateTime => "Дата и время",
            Self::Interface => "Интерфейс",
            Self::Misc => "Разное",

            Self::English => "английский",
            Self::Dutch => "голландский",
            Self::French => "французский",
            Self::German => "немецкий",
            Self::Italian => "итальянский",
            Self::Polish => "польский",
            Self::Russian => "русский",
            Self::Spanish => "испанский",
            Self::Swedish => "шведский",
            Self::Turkish => "турецкий",
            Self::Ukrainian => "украинский",
            Self::TokiPona => "токи пона",

            Self::Date => "дата",
            Self::Time => "время",
            Self::Timezone => "часовой пояс",
            Self::RotateScreen => "перевернуть изображение",
            Self::ScreenBrightness => "яркость экрана",
            Self::ReduceFlashing => "уменьшить мигание",
            Self::Contrast => "контрастность",
            Self::AutoLock => "автоматическая блокировка",
            Self::EasterEggs => "пасхалки",
            Self::ColorScheme => "цветовая схема",
            Self::GamepadMode => "режим джойстика",
            Self::Telemetry => "телеметрия",
            Self::ResetAll => "сбросить все настройки",
        }
    }

    fn translate_spanish(&self) -> &'static str {
        // TODO: Manually validate and correct the machine translation.
        match self {
            Self::Language => "Idioma",
            Self::DateTime => "Fecha y hora",
            Self::Interface => "Interfaz",
            Self::Misc => "Varios",

            Self::English => "Inglés",
            Self::Dutch => "Holandés",
            Self::French => "Francés",
            Self::German => "Alemán",
            Self::Italian => "Italiano",
            Self::Polish => "Polaco",
            Self::Russian => "Ruso",
            Self::Spanish => "Español",
            Self::Swedish => "Sueco",
            Self::Turkish => "Turco",
            Self::Ukrainian => "Ucraniano",
            Self::TokiPona => "Toki Pona",

            Self::Date => "fecha",
            Self::Time => "hora",
            Self::Timezone => "zona horaria",
            Self::RotateScreen => "girar pantalla",
            Self::ScreenBrightness => "brillo de pantalla",
            Self::ReduceFlashing => "reducir parpadeo",
            Self::Contrast => "alto contraste",
            Self::AutoLock => "bloqueo automático",
            Self::EasterEggs => "easter eggs",
            Self::ColorScheme => "esquema de colores",
            Self::GamepadMode => "modo gamepad",
            Self::Telemetry => "telemetría",
            Self::ResetAll => "restablecer todos los ajustes",
        }
    }

    fn translate_swedish(&self) -> &'static str {
        match self {
            Self::Language => "Språk",
            Self::DateTime => "Datum och tid",
            Self::Interface => "Gränssnitt",
            Self::Misc => "Övrigt",

            Self::English => "Engelska",
            Self::Dutch => "Holländska",
            Self::French => "Franska",
            Self::German => "Tyska",
            Self::Italian => "Italienska",
            Self::Polish => "Polska",
            Self::Russian => "Ryska",
            Self::Spanish => "Spanska",
            Self::Swedish => "Svenska",
            Self::Turkish => "Turkiska",
            Self::Ukrainian => "Ukrainska",
            Self::TokiPona => "Toki pona",

            Self::Date => "datum",
            Self::Time => "tid",
            Self::Timezone => "timezon",
            Self::RotateScreen => "rotera skärmen",
            Self::ScreenBrightness => "skärmljusstyrka",
            Self::ReduceFlashing => "reducera skärmflimmer",
            Self::Contrast => "högkontrast",
            Self::AutoLock => "autolås",
            Self::EasterEggs => "påskägg",
            Self::ColorScheme => "färgschema",
            Self::GamepadMode => "handkontrollsläge",
            Self::Telemetry => "telemetri",
            Self::ResetAll => "återställ alla inställningar",
        }
    }

    fn translate_turkish(&self) -> &'static str {
        // TODO: Manually validate and correct the machine translation.
        match self {
            Self::Language => "Dil",
            Self::DateTime => "Tarih ve saat",
            Self::Interface => "Arayüz",
            Self::Misc => "Diğer",

            Self::English => "İngilizce",
            Self::Dutch => "Hollandaca",
            Self::French => "Fransızca",
            Self::German => "Almanca",
            Self::Italian => "İtalyanca",
            Self::Polish => "Lehçe",
            Self::Russian => "Rusça",
            Self::Spanish => "İspanyolca",
            Self::Swedish => "İsveççe",
            Self::Turkish => "Türkçe",
            Self::Ukrainian => "Ukraynaca",
            Self::TokiPona => "Toki Pona",

            Self::Date => "tarih",
            Self::Time => "saat",
            Self::Timezone => "saat dilimi",
            Self::RotateScreen => "ekranı döndür",
            Self::ScreenBrightness => "ekran parlaklığı",
            Self::ReduceFlashing => "yanıp sönmeyi azalt",
            Self::Contrast => "yüksek kontrast",
            Self::AutoLock => "otomatik kilit",
            Self::EasterEggs => "gizli özellikler",
            Self::ColorScheme => "renk şeması",
            Self::GamepadMode => "oyun kumandası modu",
            Self::Telemetry => "telemetri",
            Self::ResetAll => "tüm ayarları sıfırla",
        }
    }

    fn translate_ukrainian(&self) -> &'static str {
        match self {
            Self::Language => "Мова",
            Self::DateTime => "Дата та час",
            Self::Interface => "Інтерфейс",
            Self::Misc => "Різне",

            Self::English => "англійська",
            Self::Dutch => "голландська",
            Self::French => "французька",
            Self::German => "німецький",
            Self::Italian => "італійська",
            Self::Polish => "польський",
            Self::Russian => "російська",
            Self::Spanish => "іспанська",
            Self::Swedish => "шведська",
            Self::Turkish => "турецька",
            Self::Ukrainian => "українська",
            Self::TokiPona => "токі пона",

            Self::Date => "дата",
            Self::Time => "час",
            Self::Timezone => "часовий пояс",
            Self::RotateScreen => "поворот екрана",
            Self::ScreenBrightness => "яскравість екрана",
            Self::ReduceFlashing => "зменшення миготіння",
            Self::Contrast => "контрастніст",
            Self::AutoLock => "автоматичне блокування",
            Self::EasterEggs => "пасхалки",
            Self::ColorScheme => "колірна схема",
            Self::GamepadMode => "режим геймпада",
            Self::Telemetry => "телеметраці",
            Self::ResetAll => "скинути всі налаштування",
        }
    }

    fn translate_toki_pona(&self) -> &'static str {
        match self {
            Self::Language => "toki",
            Self::DateTime => "tenpo",
            Self::Interface => "namako",
            Self::Misc => "ante",

            Self::English => "toki Inli",
            Self::Dutch => "toki Netelan",
            Self::French => "toki Kanse",
            Self::German => "toki Tosi",
            Self::Italian => "toki Italija",
            Self::Polish => "toki Posuka",
            Self::Russian => "toki Losi",
            Self::Spanish => "toki Epanja",
            Self::Swedish => "toki Wensa",
            Self::Turkish => "toki Tuki",
            Self::Ukrainian => "toki Ukrajini",
            Self::TokiPona => "toki Pona",

            Self::Date => "tenpo suli",
            Self::Time => "tenpo lili",
            Self::Timezone => "tenpo ma",
            Self::RotateScreen => "supa nasa",
            Self::ScreenBrightness => "supa suno",
            Self::ReduceFlashing => "suno lili",
            Self::Contrast => "kule alte",
            Self::AutoLock => "ilo pini kama",
            Self::EasterEggs => "kijetesantakalu",
            Self::ColorScheme => "kule",
            Self::GamepadMode => "ilo musi",
            Self::Telemetry => "owe",
            Self::ResetAll => "ale li sin",
        }
    }
}

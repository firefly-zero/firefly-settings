use crate::*;
use core::cell::OnceCell;
use firefly_rust::*;
use firefly_types::{Encode, Settings};

static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    pub settings: Settings,
    pub font: FileBuf,
    pub page: Page,
    pub theme: Theme,
    pub lang: Language,
    pub scroll: u8,
    pub cursor: u8,
    pub dpad: DPad4,
    pub btns: Buttons,
}

impl State {
    pub fn translate(&self, m: Message) -> &'static str {
        m.translate(&self.lang)
    }

    pub fn set_theme(&mut self, idx: u8) {
        let t = THEMES[usize::from(idx)];
        self.theme = t;
        let encoded: u32 = encode_color(t.primary);
        let encoded = encoded << 4 | encode_color(t.secondary);
        let encoded = encoded << 4 | encode_color(t.accent);
        let encoded = encoded << 4 | encode_color(t.bg);
        let encoded = encoded << 8 | u32::from(idx);
        self.settings.theme = encoded;
    }

    pub fn save_settings(&mut self) {
        let raw = self.settings.encode_vec().unwrap();
        sudo::dump_file("sys/config", &raw);
    }

    pub fn refresh(&mut self) {
        self.theme = THEMES[self.settings.theme as u8 as usize];
        self.lang = Language::from_code(self.settings.lang).unwrap_or_default();
        self.font = load_file_buf(self.lang.encoding()).unwrap();
        self.apply_contrast();
    }

    pub fn apply_contrast(&self) {
        if self.settings.contrast {
            set_color(Color::Black, RGB::new(0, 0, 0));
            set_color(Color::White, RGB::new(255, 255, 255));
        } else {
            set_color(Color::Black, RGB::new(0x1a, 0x1c, 0x2c));
            set_color(Color::White, RGB::new(0xf4, 0xf4, 0xf4));
        }
    }

    pub fn hide_toki_pona(&self) -> bool {
        if self.page != Page::Language {
            return false;
        }
        !self.settings.easter_eggs && self.settings.lang != [b't', b'p']
    }
}

fn encode_color(c: Color) -> u32 {
    u32::from(u8::from(c) - 1)
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub fn load_state() {
    let raw_settings = sudo::load_file_buf("sys/config").unwrap();
    let settings = Settings::decode(raw_settings.as_bytes()).unwrap();
    let theme = THEMES[settings.theme as u8 as usize];
    let lang = Language::from_code(settings.lang).unwrap_or_default();
    let encoding = lang.encoding();
    let font = load_file_buf(encoding).unwrap();

    // On the first launch, show the "Language" page,
    // so that if the user has a wrong language selected by default
    // and stumbles around trying to fin the language selector,
    // we land them right away to the right page.
    // This is also the reason why language selection
    // takes the whole page instead of one line like "color scheme".
    //
    // On subsequent launches, the user is unlikely to keep
    // changing languages, so we land them to the "Interface" page.
    let page = if get_file_size("launched") != 0 {
        Page::Interface
    } else {
        Page::Language
    };
    dump_file("launched", b"y");

    let state = State {
        settings,
        font,
        page,
        theme,
        lang,
        scroll: 0,
        cursor: 0,
        dpad: DPad4::default(),
        btns: Buttons::default(),
    };
    state.apply_contrast();
    #[allow(static_mut_refs)]
    unsafe { STATE.set(state) }.ok().unwrap();
}

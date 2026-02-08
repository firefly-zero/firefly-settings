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

    pub fn save_settings(&mut self) {
        let raw = self.settings.encode_vec().unwrap();
        sudo::dump_file("sys/config", &raw);
    }

    pub fn refresh(&mut self) {
        self.theme = THEMES[self.settings.theme as usize];
        self.lang = Language::from_bytes(self.settings.lang);
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
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub fn load_state() {
    let raw_settings = sudo::load_file_buf("sys/config").unwrap();
    let settings = Settings::decode(raw_settings.as_bytes()).unwrap();
    let theme = THEMES[settings.theme as usize];
    let lang = Language::from_bytes(settings.lang);
    let font = load_file_buf(lang.encoding()).unwrap();
    let state = State {
        settings,
        font,
        page: Page::Language,
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

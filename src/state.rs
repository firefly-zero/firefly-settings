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
    pub cursor: u8,
    pub dpad: DPad4,
    pub btns: Buttons,
}

impl State {
    pub fn translate(&self, m: Message) -> &'static str {
        m.translate(&self.lang)
    }
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub fn load_state() {
    let font = load_file_buf("font").unwrap();
    let raw_settings = sudo::load_file_buf("sys/config").unwrap();
    let settings = Settings::decode(raw_settings.as_bytes()).unwrap();
    let theme = THEMES[settings.theme as usize];
    let lang = Language::from_bytes(settings.lang);
    let state = State {
        settings,
        font,
        page: Page::Language,
        theme,
        lang,
        cursor: 0,
        dpad: DPad4::default(),
        btns: Buttons::default(),
    };
    #[allow(static_mut_refs)]
    unsafe { STATE.set(state) }.ok().unwrap();
}

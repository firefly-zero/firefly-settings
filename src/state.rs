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
    let state = State {
        settings,
        font,
        page: Page::Language,
        theme,
    };
    #[allow(static_mut_refs)]
    unsafe { STATE.set(state) }.ok().unwrap();
}

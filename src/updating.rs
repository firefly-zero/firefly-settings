use crate::*;
use firefly_rust::*;
use firefly_types::Settings;

pub fn update_state(state: &mut State) {
    handle_pad(state);
    handle_btns(state);
}

fn handle_pad(state: &mut State) {
    // Generally, you never want to use "get_me" peer
    // for anything but visual rendering. However,
    // the settings app is special. Settings must be
    // applied only on one device.
    let peer = get_me();

    let pad = read_pad(peer).unwrap_or_default();
    let dpad = pad.as_dpad4();
    let pressed = dpad.just_pressed(state.dpad);
    state.dpad = dpad;
    match pressed {
        DPad4::Left => {
            if state.cursor == 0 {
                state.page = state.page.prev();
            } else {
                let on_theme = state.page == Page::Interface && state.cursor == 2;
                if on_theme {
                    let new_theme = if state.settings.theme == 0 {
                        THEMES.len() as u8 - 1
                    } else {
                        state.settings.theme - 1
                    };
                    state.settings.theme = new_theme;
                    state.theme = THEMES[new_theme as usize];
                } else {
                    state.scroll = 0;
                    state.cursor = 0;
                }
            }
        }
        DPad4::Right => {
            if state.cursor == 0 {
                state.page = state.page.next();
            } else {
                let on_theme = state.page == Page::Interface && state.cursor == 2;
                if on_theme {
                    let new_theme = (state.settings.theme + 1) % THEMES.len() as u8;
                    state.settings.theme = new_theme;
                    state.theme = THEMES[new_theme as usize];
                }
            }
        }
        DPad4::Up => {
            if state.cursor > 0 {
                if state.cursor <= 5 && state.scroll != 0 {
                    state.scroll -= 1;
                }
                state.cursor -= 1;
            }
        }
        DPad4::Down => {
            let mut n_lines = state.page.lines().len() as u8;
            // Hide Toki Pona if Easter Eggs are disabled.
            if state.page == Page::Language && !state.settings.easter_eggs {
                n_lines -= 1;
            }
            if state.cursor < n_lines {
                if state.cursor - state.scroll > 6 {
                    state.scroll += 1;
                }
                state.cursor += 1;
            }
        }
        DPad4::None => {}
    }
}

fn handle_btns(state: &mut State) {
    // Generally, you never want to use "get_me" peer
    // for anything but visual rendering. However,
    // the settings app is special. Settings must be
    // applied only on one device.
    let peer = get_me();

    let btns = read_buttons(peer);
    let released = btns.just_released(&state.btns);
    if released.s || released.e {
        if state.cursor == 0 {
            state.page = state.page.next();
        } else {
            select_option(state);
        }
    }
    state.btns = btns;
}

fn select_option(state: &mut State) {
    let s = &mut state.settings;
    match state.page {
        Page::Language => {
            let lang = match state.cursor {
                1 => Language::English,
                2 => Language::Dutch,
                3 => Language::French,
                4 => Language::German,
                5 => Language::Italian,
                6 => Language::Polish,
                7 => Language::Russian,
                8 => Language::Spanish,
                9 => Language::Turkish,
                10 => Language::Ukrainian,
                11 => Language::TokiPona,
                _ => Language::English,
            };
            s.lang = lang.code_array();
            let encoding = lang.encoding();
            if encoding != state.lang.encoding() {
                state.font = load_file_buf(encoding).unwrap();
            }
            state.lang = lang;
        }
        Page::Timezone => {}
        Page::Time => {}
        Page::Interface => match state.cursor {
            1 => {
                s.theme = (s.theme + 1) % THEMES.len() as u8;
                state.theme = THEMES[s.theme as usize];
            }
            2 => {
                s.contrast = !s.contrast;
                state.apply_contrast();
            }
            3 => s.screen_brightness = s.screen_brightness.wrapping_add(64),
            4 => s.reduce_flashing = !s.reduce_flashing,
            5 => s.rotate_screen = !s.rotate_screen,
            6 => s.auto_lock = if s.auto_lock != 0 { 0 } else { 5 },
            _ => {}
        },
        Page::Misc => match state.cursor {
            1 => s.gamepad_mode = !s.gamepad_mode,
            2 => s.telemetry = !s.telemetry,
            3 => s.easter_eggs = !s.easter_eggs,
            4 => {
                *s = Settings::default();
                state.refresh();
            }
            _ => {}
        },
    }
}

use crate::*;
use firefly_rust::*;
use firefly_types::Settings;
use firefly_ui::Input;

pub fn update_state(state: &mut State) {
    state.input.update();
    match state.input.get() {
        Input::Left => {
            if state.cursor == 0 {
                state.page = state.page.prev();
            } else {
                let on_theme = state.page == Page::Interface && state.cursor == 1;
                if on_theme {
                    let old_theme = state.settings.theme as u8;
                    let new_theme = if old_theme == 0 {
                        THEMES.len() as u8 - 1
                    } else {
                        old_theme - 1
                    };
                    state.set_theme(new_theme);
                } else {
                    state.scroll = 0;
                    state.cursor = 0;
                }
            }
        }
        Input::Right => {
            if state.cursor == 0 {
                state.page = state.page.next();
            } else {
                let on_theme = state.page == Page::Interface && state.cursor == 1;
                if on_theme {
                    let new_theme = (state.settings.theme as u8 + 1) % THEMES.len() as u8;
                    state.set_theme(new_theme);
                }
            }
        }
        Input::Up => {
            if state.cursor > 0 {
                if state.cursor <= 7 && state.scroll != 0 {
                    state.scroll -= 1;
                }
                state.cursor -= 1;
            }
        }
        Input::Down => {
            let mut n_lines = state.page.lines().len() as u8;
            // Hide Toki Pona if Easter Eggs are disabled.
            if state.hide_toki_pona() {
                n_lines -= 1;
            }
            if state.cursor < n_lines {
                if state.cursor - state.scroll > 6 {
                    state.scroll += 1;
                }
                state.cursor += 1;
            }
        }
        Input::Select => {
            if state.cursor == 0 {
                state.page = state.page.next();
            } else {
                select_option(state);
            }
        }
        Input::Back => quit(),
        Input::None => {}
    }
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
                7 => Language::Romanian,
                8 => Language::Russian,
                9 => Language::Spanish,
                10 => Language::Swedish,
                11 => Language::Turkish,
                12 => Language::Ukrainian,
                13 => Language::TokiPona,
                _ => Language::English,
            };
            s.lang = lang.code_array();
            let encoding = lang.encoding();
            if encoding != state.lang.encoding() {
                state.font = load_file_buf(encoding).unwrap();
            }
            state.lang = lang;
        }
        Page::DateTime => {}
        Page::Interface => match state.cursor {
            1 => {
                let new_theme = (s.theme as u8 + 1) % THEMES.len() as u8;
                state.set_theme(new_theme);
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
                // Reset settings to default.
                // We only reset settings that are configurable
                // through this app. Other settings, like device name
                // and earned xp, should stay untouched.
                let def = Settings::default();
                s.lang = def.lang;
                s.theme = def.theme;
                s.contrast = def.contrast;
                s.screen_brightness = def.screen_brightness;
                s.reduce_flashing = def.reduce_flashing;
                s.rotate_screen = def.rotate_screen;
                s.auto_lock = def.auto_lock;
                s.gamepad_mode = def.gamepad_mode;
                s.telemetry = def.telemetry;
                s.easter_eggs = def.easter_eggs;
                state.refresh();
            }
            _ => {}
        },
    }
}

use crate::*;
use firefly_rust::*;

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
                state.cursor = 0;
            }
        }
        DPad4::Right => {
            if state.cursor == 0 {
                state.page = state.page.next();
            }
        }
        DPad4::Up => {
            if state.cursor > 0 {
                state.cursor -= 1;
            }
        }
        DPad4::Down => {
            let n_lines = state.page.lines().len() as u8;
            if state.cursor < n_lines {
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
    match state.page {
        Page::Language => {
            let lang = match state.cursor {
                1 => Language::English,
                2 => Language::Dutch,
                3 => Language::Ukrainian,
                4 => Language::Russian,
                5 => Language::TokiPona,
                _ => Language::English,
            };
            state.settings.lang = lang.as_bytes();
            let encoding = lang.encoding();
            if encoding != state.lang.encoding() {
                state.font = load_file_buf(encoding).unwrap();
            }
            state.lang = lang;
        }
        Page::Timezone => {}
        Page::Time => {}
        Page::Screen => match state.cursor {
            1 => state.settings.rotate_screen = !state.settings.rotate_screen,
            3 => state.settings.reduce_flashing = !state.settings.reduce_flashing,
            4 => state.settings.contrast = !state.settings.contrast,
            _ => {}
        },
        Page::Interface => match state.cursor {
            1 => state.settings.auto_lock = if state.settings.auto_lock != 0 { 0 } else { 5 },
            3 => state.settings.easter_eggs = !state.settings.easter_eggs,
            _ => {}
        },
        Page::Misc => match state.cursor {
            1 => state.settings.gamepad_mode = !state.settings.gamepad_mode,
            2 => state.settings.telemetry = !state.settings.telemetry,
            _ => {}
        },
    }
}

#![no_std]
#![no_main]
mod background;
mod languages;
mod pages;
mod state;
mod themes;
mod translations;

use crate::background::*;
use crate::languages::*;
use crate::pages::*;
use crate::state::*;
use crate::themes::*;
use crate::translations::*;

use firefly_rust::*;

const MARGIN: i32 = 20;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    load_state();
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();
    handle_pad(state);
    update_bg(state);
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
                state.page = state.page.prev()
            }
        }
        DPad4::Right => {
            if state.cursor == 0 {
                state.page = state.page.next()
            }
        }
        DPad4::Up => {}
        DPad4::Down => {}
        DPad4::None => {}
    }
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    draw_bg(state);
    draw_title(state);
    draw_lines(state);
}

fn draw_title(state: &State) {
    let title = state.translate(state.page.title());
    let font = state.font.as_font();
    let point = Point::new(MARGIN, MARGIN + font.char_height() as i32);
    draw_text(title, &font, point, state.theme.accent);
}

fn draw_lines(state: &State) {
    let font = state.font.as_font();
    for (line, i) in state.page.lines().iter().zip(3..) {
        let point = Point::new(MARGIN, MARGIN + i * font.char_height() as i32);
        let line = state.translate(*line);
        draw_text(line, &font, point, state.theme.primary);
    }
}

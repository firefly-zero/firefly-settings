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
    update_bg(state);
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

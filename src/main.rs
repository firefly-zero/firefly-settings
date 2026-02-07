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

const PAGE_MARGIN: i32 = 20;
const CURSOR_MARGIN: i32 = 4;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    load_state();
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();
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
            state.lang = lang;
        }
        Page::Timezone => {}
        Page::Time => {}
        Page::Screen => {}
        Page::Interface => {}
        Page::Misc => {}
    }
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    draw_bg(state);
    draw_cursor(state);
    draw_title(state);
    draw_title_arrows(state);
    draw_lines(state);
}

fn draw_title(state: &State) {
    let title = state.translate(state.page.title());
    let font = state.font.as_font();
    let mut point = Point::new(
        (WIDTH - font.line_width(title) as i32) / 2,
        PAGE_MARGIN + font.char_height() as i32,
    );
    if state.cursor == 0 && (state.btns.s || state.btns.e) {
        point.x += 1;
        point.y += 1;
    }
    draw_text(title, &font, point, state.theme.accent);
}

fn draw_title_arrows(state: &State) {
    let style = Style::solid(state.theme.accent);
    let mut p = Point::new(PAGE_MARGIN + CURSOR_MARGIN, PAGE_MARGIN + 3);
    if state.cursor == 0 && (state.btns.s || state.btns.e) {
        p.x += 1;
        p.y += 1;
    }
    draw_triangle(
        Point::new(p.x, p.y + 4),
        Point::new(p.x + 4, p.y),
        Point::new(p.x + 4, p.y + 8),
        style,
    );

    p.x += WIDTH - 2 * (PAGE_MARGIN + CURSOR_MARGIN) - 1;
    draw_triangle(
        Point::new(p.x, p.y + 4),
        Point::new(p.x - 4, p.y),
        Point::new(p.x - 4, p.y + 8),
        style,
    );
}

fn draw_lines(state: &State) {
    let font = state.font.as_font();
    let line_h = font.char_height() as i32 + CURSOR_MARGIN;
    for (line, i) in state.page.lines().iter().zip(2..) {
        let mut point = Point::new(
            PAGE_MARGIN + CURSOR_MARGIN,
            PAGE_MARGIN + i * line_h - CURSOR_MARGIN,
        );
        if i - 1 == state.cursor as i32 && (state.btns.s || state.btns.e) {
            point.x += 1;
            point.y += 1;
        }
        let line = state.translate(*line);
        draw_text(line, &font, point, state.theme.primary);
    }
}

fn draw_cursor(state: &State) {
    let font = state.font.as_font();
    let line_h = font.char_height() as i32 + CURSOR_MARGIN;
    let y = PAGE_MARGIN + state.cursor as i32 * line_h + 1;
    let mut point = Point::new(PAGE_MARGIN, y);
    let bbox = Size::new(
        WIDTH - PAGE_MARGIN * 2,
        font.char_height() as i32 + CURSOR_MARGIN,
    );
    let corner = Size::new(4, 4);

    if state.btns.s || state.btns.e {
        point.x += 1;
        point.y += 1;
    } else {
        let style = Style::solid(state.theme.primary);
        let shadow_point = Point::new(point.x + 1, point.y + 1);
        draw_rounded_rect(shadow_point, bbox, corner, style);
    }

    let style = Style {
        fill_color: state.theme.bg,
        stroke_color: state.theme.primary,
        stroke_width: 1,
    };
    draw_rounded_rect(point, bbox, corner, style);
}

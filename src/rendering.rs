use crate::*;
use firefly_rust::*;

const BOX_ML: i32 = 20;
const BOX_MR: i32 = BOX_ML;
const BOX_MT: i32 = 20;
const BOX_Y: i32 = BOX_MT;

const CURSOR_ML: i32 = 4;
const LINE_M: i32 = 4;
const CURSOR_X: i32 = BOX_ML + CURSOR_ML;

const PER_PAGE: usize = 8;

pub fn render_state(state: &State) {
    draw_bg(state);
    draw_cursor(state);
    draw_title(state);
    draw_title_arrows(state);
    draw_lines(state);
    draw_selections(state);
}

fn draw_title(state: &State) {
    let title = state.translate(state.page.title());
    let font = state.font.as_font();
    let mut point = Point::new(
        (WIDTH - line_width(&font, title)) / 2,
        BOX_Y + font.char_height() as i32,
    );
    if state.cursor == 0 && (state.btns.s || state.btns.e) {
        point.x += 1;
        point.y += 1;
    }
    draw_text(title, &font, point, state.theme.accent);
}

fn draw_title_arrows(state: &State) {
    let style = Style::solid(state.theme.accent);
    let mut p = Point::new(CURSOR_X + 1, BOX_Y + 3);
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

    p.x += WIDTH - 2 * CURSOR_X - 3;
    draw_triangle(
        Point::new(p.x, p.y + 4),
        Point::new(p.x - 4, p.y),
        Point::new(p.x - 4, p.y + 8),
        style,
    );
}

fn draw_lines(state: &State) {
    // Get the list of settings lines to display.
    let mut lines = state.page.lines();
    if state.page == Page::Language && !state.settings.easter_eggs {
        // Hide Toki Pona if Easter Eggs are disabled.
        lines = &lines[..lines.len() - 1];
    }
    lines = &lines[state.scroll as usize..];
    let has_more = lines.len() > PER_PAGE;
    if has_more {
        lines = &lines[..PER_PAGE];
    }

    let font = state.font.as_font();
    let line_h = font.char_height() as i32 + LINE_M;
    for (line, i) in lines.iter().zip(2..) {
        let mut point = Point::new(CURSOR_X, BOX_Y + i * line_h - LINE_M);
        if i - 1 == state.cursor as i32 && (state.btns.s || state.btns.e) {
            point.x += 1;
            point.y += 1;
        }
        let line = state.translate(*line);
        draw_text(line, &font, point, state.theme.primary);
    }

    if has_more {
        let y = HEIGHT - BOX_Y - 2;
        draw_triangle(
            Point::new(CURSOR_X, y),
            Point::new(CURSOR_X + 4, y + 4),
            Point::new(CURSOR_X + 8, y),
            Style::solid(state.theme.accent),
        );
    }
}

fn draw_cursor(state: &State) {
    let font = state.font.as_font();
    let line_h = font.char_height() as i32 + LINE_M;
    let y = BOX_MT + (state.cursor - state.scroll) as i32 * line_h + 1;
    let mut point = Point::new(BOX_ML, y);
    let bbox = Size::new(WIDTH - BOX_ML - BOX_MR, font.char_height() as i32 + LINE_M);
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

fn draw_selections(state: &State) {
    match state.page {
        Page::Language => draw_lang_selection(state),
        Page::Timezone => draw_tz_selection(state),
        Page::Time => {}
        Page::Screen => draw_screen_selections(state),
        Page::Interface => draw_interface_selections(state),
        Page::Misc => draw_misc_selections(state),
    }
}

/// Render selection marker next to the currently active language.
fn draw_lang_selection(state: &State) {
    let mut idx: i32 = match state.lang {
        Language::English => 1,
        Language::Dutch => 2,
        Language::French => 3,
        Language::German => 4,
        Language::Italian => 5,
        Language::Polish => 6,
        Language::Russian => 7,
        Language::Spanish => 8,
        Language::Turkish => 9,
        Language::Ukrainian => 10,
        Language::TokiPona => 11,
    };
    idx -= state.scroll as i32;
    if idx < 1 || idx > PER_PAGE as i32 {
        return;
    }
    draw_marker(state, idx);
}

fn draw_tz_selection(state: &State) {
    if state.settings.timezone == "Europe/Amsterdam" {
        draw_marker(state, 1);
    }
}

fn draw_screen_selections(state: &State) {
    draw_switch(state, 1, state.settings.rotate_screen);
    draw_switch(state, 3, state.settings.reduce_flashing);
    draw_switch(state, 4, state.settings.contrast);
}

fn draw_interface_selections(state: &State) {
    draw_switch(state, 1, state.settings.auto_lock != 0);
    draw_switch(state, 3, state.settings.easter_eggs);
}

fn draw_misc_selections(state: &State) {
    draw_switch(state, 1, state.settings.gamepad_mode);
    draw_switch(state, 2, state.settings.telemetry);
}

fn draw_marker(state: &State, idx: i32) {
    let font = state.font.as_font();
    let h = font.char_height() as i32;
    let x = WIDTH - CURSOR_X - h;
    let line_h = font.char_height() as i32 + LINE_M;
    let y = CURSOR_X + idx * line_h - 1;
    let mut point = Point::new(x, y);
    if idx == state.cursor as i32 && (state.btns.s || state.btns.e) {
        point.x += 1;
        point.y += 1;
    }
    let style = Style::solid(state.theme.accent);
    draw_circle(point, h, style);
}

fn draw_switch(state: &State, idx: i32, is_on: bool) {
    let font = state.font.as_font();
    let h = font.char_height() as i32;
    let x = WIDTH - CURSOR_X - h * 2;
    let line_h = font.char_height() as i32 + LINE_M;
    let y = CURSOR_X + idx * line_h - 1;
    let mut point = Point::new(x, y);
    if idx == state.cursor as i32 && (state.btns.s || state.btns.e) {
        point.x += 1;
        point.y += 1;
    }

    {
        let mut switch_point = point;
        let mut color = state.theme.secondary;
        if is_on {
            switch_point.x += h;
            color = state.theme.accent;
        }
        let style = Style::solid(color);
        draw_circle(switch_point, h, style);
    }

    let style = Style::outlined(state.theme.primary, 1);
    let corner = Size::new(h / 2, h / 2);
    draw_rounded_rect(point, Size::new(h * 2, h), corner, style);
}

/// Calculate the width in pixels of the given text.
///
/// The SDK has [`Font::line_width`] out of the box
/// but it only works with ASCII text.
fn line_width(font: &Font, t: &str) -> i32 {
    t.chars().count() as i32 * font.char_width() as i32
}

use crate::*;
use firefly_rust::*;

const PAGE_MARGIN: i32 = 20;
const CURSOR_MARGIN: i32 = 4;

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
    let mut p = Point::new(PAGE_MARGIN + CURSOR_MARGIN + 1, PAGE_MARGIN + 3);
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

    p.x += WIDTH - 2 * (PAGE_MARGIN + CURSOR_MARGIN) - 3;
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

fn draw_selections(state: &State) {
    match state.page {
        Page::Language => draw_lang_selection(state),
        Page::Timezone => {}
        Page::Time => {}
        Page::Screen => {}
        Page::Interface => {}
        Page::Misc => {}
    }
}

/// Render selection marker next to the currently active language.
fn draw_lang_selection(state: &State) {
    let idx: i32 = match state.lang {
        Language::English => 1,
        Language::Dutch => 2,
        Language::Ukrainian => 3,
        Language::Russian => 4,
        Language::TokiPona => 5,
    };

    let font = state.font.as_font();
    let x = WIDTH - PAGE_MARGIN - CURSOR_MARGIN - 7;
    let line_h = font.char_height() as i32 + CURSOR_MARGIN;
    let y = PAGE_MARGIN + CURSOR_MARGIN + idx * line_h;
    let mut point = Point::new(x, y);
    if idx == state.cursor as i32 && (state.btns.s || state.btns.e) {
        point.x += 1;
        point.y += 1;
    }

    let style = Style::solid(state.theme.accent);
    draw_circle(point, 7, style);
}

/// Calculate the width in pixels of the given text.
///
/// The SDK has [`Font::line_width`] out of the box
/// but it only works with ASCII text.
fn line_width(font: &Font, t: &str) -> i32 {
    t.chars().count() as i32 * font.char_width() as i32
}

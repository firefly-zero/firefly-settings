use crate::*;
use firefly_rust::*;

const BOX_ML: i32 = 16;
const BOX_MT: i32 = 16;
const BOX_Y: i32 = BOX_MT;

const CURSOR_ML: i32 = 4;
const LINE_M: i32 = 4;
const CURSOR_X: i32 = BOX_ML + CURSOR_ML;

const PER_PAGE: usize = 8;

pub fn render_state(state: &State) {
    let theme = cast_theme(state.theme);
    firefly_ui::draw_bg(theme);
    let font = &state.font;
    let jitter = state.input.jitter(state.hitting_wall);
    firefly_ui::draw_cursor(
        u32::from(state.cursor - state.scroll),
        theme,
        font,
        state.input.pressed(),
        jitter,
    );
    draw_title(state);
    draw_title_arrows(state);
    draw_lines(state);
    draw_selections(state);
}

fn draw_title(state: &State) {
    let text = state.translate(state.page.title());
    let font = &state.font;
    let pressed = state.cursor == 0 && state.input.pressed();
    let color = state.theme.accent;
    firefly_ui::draw_title(text, pressed, font, color);
}

fn draw_title_arrows(state: &State) {
    let style = Style::solid(state.theme.accent);
    let mut p = Point::new(CURSOR_X + 1, BOX_Y + 3);
    if state.cursor == 0 && state.input.pressed() {
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
    if state.hide_toki_pona() {
        // Hide Toki Pona if Easter Eggs are disabled.
        lines = &lines[..lines.len() - 1];
    }
    lines = &lines[state.scroll as usize..];
    let has_more = lines.len() > PER_PAGE;
    if has_more {
        lines = &lines[..PER_PAGE];
    }

    let font = &state.font;
    let line_h = font.char_height() as i32 + LINE_M;
    for (line, i) in lines.iter().zip(2..) {
        let mut point = Point::new(CURSOR_X, BOX_Y + i * line_h - LINE_M);
        if i - 1 == i32::from(state.cursor) && state.input.pressed() {
            point.x += 1;
            point.y += 1;
        }
        let line = state.translate(*line);
        draw_text(line, font, point, state.theme.primary);
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

fn draw_selections(state: &State) {
    match state.page {
        Page::Language => draw_lang_selection(state),
        Page::Interface => draw_interface_selections(state),
        Page::Misc => draw_misc_selections(state),
        Page::SystemInfo => draw_system_info_selections(state),
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
        Language::Romanian => 7,
        Language::Russian => 8,
        Language::Spanish => 9,
        Language::Swedish => 10,
        Language::Turkish => 11,
        Language::Ukrainian => 12,
        Language::TokiPona => 13,
    };
    idx -= state.scroll as i32;
    if idx < 1 || idx > PER_PAGE as i32 {
        return;
    }
    draw_marker(state, idx);
}

fn draw_interface_selections(state: &State) {
    draw_switch(state, 2, state.settings.contrast);
    draw_switch(state, 4, state.settings.reduce_flashing);
    draw_switch(state, 5, state.settings.rotate_screen);
    draw_switch(state, 6, state.settings.auto_lock != 0);
    draw_text_selection(state, 2, state.theme.name);
}

fn draw_misc_selections(state: &State) {
    draw_switch(state, 1, state.settings.gamepad_mode);
    draw_switch(state, 2, state.settings.telemetry);
    draw_switch(state, 3, state.settings.easter_eggs);
}

fn draw_system_info_selections(state: &State) {
    draw_text_selection(state, 2, &state.settings.name);
    if let Some(device) = &state.device {
        let serial = alloc::format!("{}.{:08}", device.model, device.serial);
        draw_text_selection(state, 3, &serial);

        let (v1, v2, v3) = device.main_version;
        let part = match device.main_partition {
            0 => "A",
            1 => "B",
            2 => "C",
            _ => "X",
        };
        let v = alloc::format!("v{v1}.{v2:02}.{v3:02} ({part})");
        draw_text_selection(state, 4, &v);

        let (v1, v2, v3) = device.io_version;
        let part = match device.io_partition {
            0 => "A",
            1 => "B",
            2 => "C",
            _ => "X",
        };
        let v = alloc::format!("v{v1}.{v2:02}.{v3:02} ({part})");
        draw_text_selection(state, 5, &v);
    }
}

fn draw_text_selection(state: &State, idx: i32, text: &str) {
    let font = &state.font;
    let line_h = font.char_height() as i32 + LINE_M;
    let x = WIDTH - CURSOR_X - font.line_width_utf8(text) as i32;
    let y = BOX_Y + idx * line_h - LINE_M;
    let mut point = Point::new(x, y);
    if idx - 1 == i32::from(state.cursor) && state.input.pressed() {
        point.x += 1;
        point.y += 1;
    }
    draw_text(text, font, point, state.theme.accent);
}

fn draw_marker(state: &State, idx: i32) {
    let font = &state.font;
    let h = font.char_height() as i32;
    let x = WIDTH - CURSOR_X - h;
    let line_h = font.char_height() as i32 + LINE_M;
    let y = CURSOR_X + idx * line_h - 1;
    let mut point = Point::new(x, y);
    if idx == i32::from(state.cursor) && state.input.pressed() {
        point.x += 1;
        point.y += 1;
    }
    let style = Style::solid(state.theme.accent);
    draw_circle(point, h, style);
}

fn draw_switch(state: &State, idx: i32, is_on: bool) {
    let font = &state.font;
    let pressed = idx == i32::from(state.cursor) && state.input.pressed();
    let theme = cast_theme(state.theme);
    firefly_ui::draw_switch(idx, is_on, pressed, font, theme);
}

fn cast_theme(theme: ThemeInfo) -> Theme {
    Theme {
        id: 0,
        primary: theme.primary,
        secondary: theme.secondary,
        accent: theme.accent,
        bg: theme.bg,
    }
}

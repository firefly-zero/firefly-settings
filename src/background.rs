use crate::*;
use firefly_rust::*;

pub fn draw_bg(state: &State) {
    draw_bg_grid(state);
    draw_bg_box(state);
}

fn draw_bg_grid(state: &State) {
    const CELL_SIZE: i32 = 8;

    clear_screen(state.theme.bg);
    let style = LineStyle::new(state.theme.secondary, 1);
    for x in (CELL_SIZE..WIDTH).step_by(CELL_SIZE as _) {
        draw_line(Point::new(x, 0), Point::new(x, HEIGHT), style)
    }
    for y in (CELL_SIZE..HEIGHT).step_by(CELL_SIZE as _) {
        draw_line(Point::new(0, y), Point::new(WIDTH, y), style)
    }
}

fn draw_bg_box(state: &State) {
    const MARGIN: i32 = 12;

    let size = Size::new(WIDTH - MARGIN * 2, HEIGHT - MARGIN * 2);
    draw_rounded_rect(
        Point::new(MARGIN + 1, MARGIN + 1),
        size,
        Size::new(4, 4),
        Style::solid(state.theme.primary),
    );
    draw_rounded_rect(
        Point::new(MARGIN, MARGIN),
        size,
        Size::new(4, 4),
        Style {
            fill_color: state.theme.bg,
            stroke_color: state.theme.primary,
            stroke_width: 1,
        },
    );
}

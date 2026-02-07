use crate::*;
use firefly_rust::*;

pub fn update_bg(state: &State) {
    // ...
}

pub fn draw_bg(state: &State) {
    draw_bg_grid();
    draw_bg_box();
}

fn draw_bg_grid() {
    const CELL_SIZE: i32 = 10;

    clear_screen(Color::White);
    let style = LineStyle::new(Color::LightGray, 1);
    for x in (CELL_SIZE..WIDTH).step_by(CELL_SIZE as _) {
        draw_line(Point::new(x, 0), Point::new(x, HEIGHT), style)
    }
    for y in (CELL_SIZE..HEIGHT).step_by(CELL_SIZE as _) {
        draw_line(Point::new(0, y), Point::new(WIDTH, y), style)
    }
}

fn draw_bg_box() {
    const MARGIN: i32 = 15;

    let size = Size::new(WIDTH - MARGIN * 2, HEIGHT - MARGIN * 2);
    draw_rounded_rect(
        Point::new(MARGIN + 1, MARGIN + 1),
        size,
        Size::new(4, 4),
        Style::solid(Color::Black),
    );
    draw_rounded_rect(
        Point::new(MARGIN, MARGIN),
        size,
        Size::new(4, 4),
        Style {
            fill_color: Color::White,
            stroke_color: Color::Black,
            stroke_width: 1,
        },
    );
}

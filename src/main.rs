#![no_std]
#![no_main]
mod background;
mod pages;
mod state;

use crate::background::*;
use crate::state::*;

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
}

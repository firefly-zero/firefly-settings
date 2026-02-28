#![no_std]
#![no_main]
mod pages;
mod rendering;
mod state;
mod themes;
mod translations;
mod updating;

use crate::pages::*;
use crate::rendering::render_state;
use crate::state::*;
use crate::themes::*;
use crate::translations::*;
use crate::updating::*;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    load_state();
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();
    update_state(state);
}

#[unsafe(no_mangle)]
extern "C" fn before_exit() {
    let state = get_state();
    state.save_settings();
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    render_state(state);
}

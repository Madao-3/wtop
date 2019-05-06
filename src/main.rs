#[macro_use]
extern crate clap;
extern crate pancurses;
use clap::App;
use chrono::prelude::*;
use pancurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input};

fn main() {
    App::new("WTOP").version("0.1.0");
    let window = initscr();
    // mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());
    loop {
        // let now = Utc::now();
        // window.mvprintw(2, 1, &format!("{}", now.timestamp_millis()));
        
        window.refresh();
    }
    endwin();
}

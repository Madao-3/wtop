#[macro_use]
extern crate clap;
extern crate pancurses;
use clap::App;
use chrono::prelude::*;
use pancurses::*;

mod Canvas {
    use pancurses::*;
    pub fn fill_rect(window: &Window, x: i32, y: i32, width: i16, height: i16, color: chtype) {
        let (y_max, x_max) = window.get_max_yx();
        let _color = COLOR_PAIR(color);
        for ox in 1..width {
            for oy in 1..height {
                let mut cx: i32 = x + (ox as i32) - 1;
                let mut cy: i32 = y + (oy as i32);
                if cx > x_max { continue; }
                if cy > y_max { continue; }
                window.attrset(_color);
                init_pair(color as i16, color as i16, COLOR_BLACK);
                window.mv(cy, cx);
                window.attron(A_REVERSE);
                window.addstr(" ");
            }
        }
    }

    pub fn strok_rect(window: &Window, x: i32, y: i32, width: i16, height: i16, color: chtype) {
        let (y_max, x_max) = window.get_max_yx();
        let _color = COLOR_PAIR(color);
        for ox in 1..width {
            for oy in 1..height {
                let mut cx: i32 = x + (ox as i32) - 1;
                let mut cy: i32 = y + (oy as i32) - 1;
                if cx > x_max { continue; }
                if cy > y_max { continue; }
                if !(cx == x ||  cy == y || cx == (x + width as i32 - 2) || cy == (y + height as i32 - 2)) { continue; }
                window.attrset(_color);
                init_pair(color as i16, color as i16, COLOR_BLACK);
                window.mv(cy, cx);
                window.attron(A_REVERSE);
                window.addstr(" ");
            }
        }
    }
}
fn main() {
    App::new("WTOP").version("0.1.0");
    let window = initscr();
    start_color();
    use_default_colors();
    set_blink(true);
    window.refresh();
    window.clear();
    cbreak();
    nl();
    noecho();
    curs_set(0);
    loop {
        let (y_max, x_max) = window.get_max_yx();
        Canvas::strok_rect(&window, 20, 10, 105, 10, 5);
        window.refresh();
    }
    endwin();
}



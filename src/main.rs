#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;

use canvas::Canvas;

fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);

    stdweb::event_loop();
}

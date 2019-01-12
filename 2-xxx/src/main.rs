extern crate stdweb;
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod canvas;

use canvas::Canvas;

use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget};

fn main() {
    stdweb::initialize();

    let game_width = 500;
    let game_height = 800;

    let mut x = 45;
    let mut y = 45;
    let brick_width = 40;
    let brick_height = 10;



    let canvas = Canvas::new("#canvas", game_width, game_height);

    canvas.clear_all();
    canvas.draw_rect(x, y, brick_width, brick_height, "red");

    stdweb::event_loop();
}

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

    let mut x = 45;
    let mut y = 45;

    let width = 200;
    let height = 200;

    let canvas = Canvas::new("#canvas", width, height);

    canvas.clear_all();
    canvas.draw(x, y, "red");

    stdweb::web::document().add_event_listener({
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => { x = x - 1},
                "ArrowRight" => { x = x + 1},
                "ArrowDown" => { y = y + 1 },
                "ArrowUp" => { y = y - 1 },
                _ => {}
            };

            canvas.clear_all();
            canvas.draw(x, y, "red");
        }
    });

    stdweb::event_loop();
}

#[macro_use]
extern crate stdweb;

mod canvcas;
mod direction;

fn main() {
    stdweb::initialize();


    stdweb::event_loop();
}

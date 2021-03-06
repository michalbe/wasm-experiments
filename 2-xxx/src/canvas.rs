use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        canvas.set_width(width);
        canvas.set_height(height);

        Canvas {
            canvas,
            ctx,
            width,
            height
        }
    }

    pub fn draw_rect(&self, x: u32, y: u32, width: u32, height: u32, color: &str) {
        self.ctx.set_fill_style_color(color);

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(width),
            f64::from(height),
        );

    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.width),
            f64::from(self.height)
        );
    }
}

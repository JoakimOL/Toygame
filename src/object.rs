extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
pub struct Object {
    pub x:        f64,
    pub y:        f64,
    pub selected: bool,
}
impl Object {
    pub const OBJECTSIZE: f64 = 30.0;
    const OBJECTCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const BORDER: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const BORDER_SELECTED: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        let border_rect =
            graphics::rectangle::square(self.x, self.y, Object::OBJECTSIZE);
        let object_rect = graphics::rectangle::square(
            self.x + 5.0,
            self.y + 5.0,
            Object::OBJECTSIZE - 10.0,
        );

        gl.draw(args.viewport(), |c, gl| {
            if (self.selected) {
                graphics::rectangle(
                    Object::BORDER_SELECTED,
                    border_rect,
                    c.transform,
                    gl,
                );
            }
            else {
                graphics::rectangle(
                    Object::BORDER,
                    border_rect,
                    c.transform,
                    gl,
                );
            }
            graphics::rectangle(
                Object::OBJECTCOLOR,
                object_rect,
                c.transform,
                gl,
            );
        });
    }
}

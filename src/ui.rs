extern crate find_folder;
extern crate opengl_graphics;
extern crate piston_window;

use self::piston_window::{text, TextureSettings, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;

pub(crate) struct Ui {
    pub(crate) height: u32,
    pub(super) width:  u32,
}

impl Ui {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let mut glyph_cache = GlyphCache::new(
            "./assets/FiraSans-Regular.ttf",
            (),
            TextureSettings::new(),
        ).unwrap();

        gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(
                [0.0, 0.0, 0.0, 1.0],
                [0.0, (self.height - 100) as f64, self.width as f64, 100.0],
                c.transform,
                gl,
            );
            text(
                [1.0, 1.0, 1.0, 1.0],
                32,
                "Hello world!",
                &mut glyph_cache,
                c.transform.trans(100.0, 100.0),
                gl,
            ).unwrap();
        });
    }
}

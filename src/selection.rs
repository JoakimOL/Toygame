extern crate opengl_graphics;

// use collidable::Collidable;
use object::Object;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use player::Player;

pub struct Selection {
    pub x:      f64,
    pub y:      f64,
    pub width:  f64,
    pub height: f64,
}

// impl Collidable for Selection {
//     fn check_collide(&self, other: &Collidable) -> bool {
//         let mut x = self.x;
//         let mut y = self.y;
//         let mut height = self.height;
//         let mut width = self.width;
//         if (self.width < 0.0) {
//             x = x + width;
//             width = width * -1.0;
//         }
//         if (self.height < 0.0) {
//             y = y + height;
//             height = height * -1.0;
//         }
//
//         if ((x < other.x && x + width > other.x
//             || other.x < x && other.x + Object::OBJECTSIZE > x)
//             && (y < other.y && y + height > other.y
//                 || other.y < y && other.y + Object::OBJECTSIZE > y))
//         {
//             return true;
//         }
//         return false;
//     }
// }

impl Selection {
    const SELECTION_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 0.1];

    pub fn check_collide(&self, other: &Player) -> bool {
        let mut x = self.x;
        let mut y = self.y;
        let mut height = self.height;
        let mut width = self.width;
        if (self.width < 0.0) {
            x = x + width;
            width = width * -1.0;
        }
        if (self.height < 0.0) {
            y = y + height;
            height = height * -1.0;
        }

        if ((x < other.x && x + width > other.x
            || other.x < x && other.x + Object::OBJECTSIZE > x)
            && (y < other.y && y + height > other.y
                || other.y < y && other.y + Object::OBJECTSIZE > y))
        {
            return true;
        }
        return false;
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(
                Selection::SELECTION_COLOR,
                [self.x, self.y, -self.width, -self.height],
                c.transform,
                gl,
            );
        });
    }
}

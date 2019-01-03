extern crate opengl_graphics;

use object::Object;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub struct Player {
    pub x:        f64,
    pub y:        f64,
    pub targetx:  f64,
    pub targety:  f64,
    pub moving:   bool,
    pub selected: bool,
    pub lastxy:   [f64; 2],
}

impl Player {
    pub const PLAYERCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BORDER: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const PLAYERSIZE: f64 = 30.0;

    pub fn step(&mut self) {
        if (self.moving == false) {
            self.y = self.lastxy[1];
            self.x = self.lastxy[0];
            return;
        }
        self.lastxy[0] = self.x;
        self.lastxy[1] = self.y;

        if (self.x > self.targetx) {
            self.x -= 1.0;
        }
        else if (self.x < self.targetx) {
            self.x += 1.0;
        }
        if (self.y > self.targety) {
            self.y -= 1.0;
        }
        else if (self.y < self.targety) {
            self.y += 1.0;
        }
        if (self.is_near()) {
            self.moving = false;
            return;
        }
    }

    pub fn move_player(&mut self, x: f64, y: f64) {
        self.moving = true;
        self.targetx = x;
        self.targety = y;
    }

    pub fn is_near(&self) -> bool {
        if (self.x.floor() > self.targetx.floor() - 1.0
            && self.x.floor() < self.targetx.floor() + 1.0
            && self.y.floor() > self.targety.floor() - 1.0
            && self.y.floor() < self.targety.floor() + 1.0)
        {
            return true;
        }
        return false;
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        if (self.selected) {
            let border_rect =
                graphics::rectangle::square(self.x, self.y, Player::PLAYERSIZE);
            let player_rect = graphics::rectangle::square(
                self.x + 5.0,
                self.y + 5.0,
                Player::PLAYERSIZE - 10.0,
            );

            gl.draw(args.viewport(), |c, gl| {
                graphics::rectangle(
                    Player::BORDER,
                    border_rect,
                    c.transform,
                    gl,
                );
                graphics::rectangle(
                    Player::PLAYERCOLOR,
                    player_rect,
                    c.transform,
                    gl,
                );
            });
        }
        else {
            let player_rect =
                graphics::rectangle::square(self.x, self.y, Player::PLAYERSIZE);

            gl.draw(args.viewport(), |c, gl| {
                graphics::rectangle(
                    Player::PLAYERCOLOR,
                    player_rect,
                    c.transform,
                    gl,
                );
            });
        }
    }
    pub fn check_collide(&mut self, object: &Object) -> bool {
        if ((self.x < object.x && self.x + Player::PLAYERSIZE > object.x
            || object.x < self.x && object.x + Object::OBJECTSIZE > self.x)
            && (self.y < object.y && self.y + Player::PLAYERSIZE > object.y
                || object.y < self.y && object.y + Object::OBJECTSIZE > self.y))
        {
            return true;
        }
        return false;
    }

    pub fn check_collide_player(&self, object: Player) -> bool {
        if ((self.x < object.x && self.x + Player::PLAYERSIZE > object.x
            || object.x < self.x && object.x + Player::PLAYERSIZE > self.x)
            && (self.y < object.y && self.y + Player::PLAYERSIZE > object.y
                || object.y < self.y && object.y + Player::PLAYERSIZE > self.y))
        {
            return true;
        }
        return false;
    }
}

#![allow(unused_parens)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

mod ui;
use ui::Ui;
mod object;
use object::Object;
mod selection;
use selection::Selection;
mod player;
use player::Player;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent,
    RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;

pub struct App {
    gl:           GlGraphics,
    objects:      Vec<Object>,
    players:      Vec<Player>,
    cursor:       [f64; 2],
    buttons:      [u8; 4],
    select:       Option<Selection>,
    is_selecting: bool,
    ui:           Ui,
}

impl App {
    const HEIGHT: u32 = 600;
    const WIDTH: u32 = 800;

    fn render(&mut self, args: &RenderArgs) {
        use graphics;
        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 0.5];
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BACKGROUND, gl);
        });

        self.ui.render(&mut self.gl, args);

        for object in &mut self.objects {
            object.render(&mut self.gl, args);
        }

        for player in &mut self.players {
            player.render(&mut self.gl, args);
            let line_coords = [
                player.x + Player::PLAYERSIZE / 2.0,
                player.y + Player::PLAYERSIZE / 2.0,
                self.cursor[0],
                self.cursor[1],
            ];
            self.gl.draw(args.viewport(), |c, gl| {
                graphics::line(
                    Player::PLAYERCOLOR,
                    1.0,
                    line_coords,
                    c.transform,
                    gl,
                );
            });
        }

        match &mut self.select {
            Some(sel) => sel.render(&mut self.gl, args),
            None => {}
        }
    }

    fn update_mouse_pos(&mut self, pos: [f64; 2]) {
        self.cursor = pos;
    }

    fn update(&mut self, _args: &UpdateArgs) {
        match &mut self.select {
            Some(s) => {
                s.width = s.x - self.cursor[0];
                s.height = s.y - self.cursor[1];
            }
            None => {}
        }

        for player in &mut self.players {
            if (player.moving) {
                for object in &mut self.objects {
                    if (player.check_collide(object)) {
                        player.moving = false;
                    }
                }
                player.step();
            }
        }

        // TODO
        // Collision check units
        for &mut player in &mut self.players {
            if (player.moving) {
                for &oplayer in &self.players {
                    if (player.check_collide_player(oplayer)) {
                        player.moving = false;
                    }
                }
                player.step();
            }
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {
                    self.buttons[0] = 1;
                }
                Key::A => {
                    self.buttons[1] = 1;
                }
                Key::S => {
                    self.buttons[2] = 1;
                }
                Key::D => {
                    self.buttons[3] = 1;
                }
                Key::Space => self.players.push(Player {
                    x:        self.cursor[0],
                    y:        self.cursor[1],
                    targetx:  0.0,
                    targety:  0.0,
                    moving:   false,
                    selected: false,
                    lastxy:   [self.cursor[0], self.cursor[1]],
                }),
                _ => {}
            }
        }
        if let &Button::Mouse(key) = args {
            match key {
                MouseButton::Left => {
                    self.start_box();
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {
                    self.buttons[0] = 0;
                }
                Key::A => {
                    self.buttons[1] = 0;
                }
                Key::S => {
                    self.buttons[2] = 0;
                }
                Key::D => {
                    self.buttons[3] = 0;
                }
                _ => {}
            }
        }

        if let &Button::Mouse(key) = args {
            match key {
                MouseButton::Right => {
                    for player in &mut self.players {
                        if (player.selected) {
                            player.move_player(
                                (self.cursor[0] - Player::PLAYERSIZE / 2.0)
                                    .floor(),
                                (self.cursor[1] - Player::PLAYERSIZE / 2.0)
                                    .floor(),
                            );
                        }
                    }
                }
                MouseButton::Left => {
                    // self.objects.push(Object {
                    //     x: self.cursor[0] - Object::OBJECTSIZE / 2.0,
                    //     y: self.cursor[1] - Object::OBJECTSIZE / 2.0,
                    // });
                    self.end_box();
                }
                _ => {}
            }
        }
    }

    fn start_box(&mut self) {
        self.is_selecting = true;
        self.select = Some(Selection {
            x:      self.cursor[0],
            y:      self.cursor[1],
            height: 15.0,
            width:  15.0,
        });
    }

    fn end_box(&mut self) {
        match &mut self.select {
            Some(s) => {
                s.width = -(s.x - self.cursor[0]);
                s.height = -(s.y - self.cursor[1]);
                for player in &mut self.players {
                    if (s.check_collide(player)) {
                        player.selected = true;
                    }
                    else {
                        player.selected = false;
                    }
                }
            }
            None => {}
        }
        self.is_selecting = false;

        self.select = None;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow =
        WindowSettings::new("test", [App::HEIGHT, App::WIDTH])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = App {
        gl:           GlGraphics::new(opengl),
        players:      vec![],
        objects:      vec![],
        cursor:       [0.0, 0.0],
        buttons:      [0, 0, 0, 0],
        select:       None,
        is_selecting: false,
        ui:           Ui {
            height: App::HEIGHT,
            width:  App::WIDTH,
        },
    };

    for i in 1..10 {
        app.objects.push(Object {
            x:        50.0 + (i as f64 * 30.0),
            y:        300.0,
            selected: false,
        });
    }

    let mut eventsettings = EventSettings::new();
    eventsettings.ups = 60;
    eventsettings.max_fps = 60;
    let mut events = Events::new(eventsettings);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(e) = e.mouse_cursor_args() {
            app.update_mouse_pos(e);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }

        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
}

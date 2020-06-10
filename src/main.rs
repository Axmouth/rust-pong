extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

#[derive(Clone, PartialEq, Copy)]
enum Winner {
    None,
    Right,
    Left,
}

pub struct App {
    gl_graphics: GlGraphics,
    left_score: i32,
    left_pos: f32,
    left_vel: i32,
    right_score: i32,
    right_pos: f32,
    right_vel: i32,
    ball_x: f32,
    ball_y: f32,
    vel_x: i32,
    vel_y: i32,
    arena_width: f32,
    arena_height: f32,
    speed_factor: f32,
    winner: Winner,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        use opengl_graphics::*;
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const VICTORY_BACKGROUND: [f32; 4] = [0.0, 0.2, 0.0, 1.0];
        const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const MIDDLE_LINE: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const SCORE: [f32; 4] = [7.0, 7.0, 7.0, 1.0];
        self.arena_height = args.window_size[1] as f32;
        self.arena_width = args.window_size[0] as f32;

        let left = rectangle::square(0.0, 0.0, 50.0);
        let left_pos = self.left_pos as f64;
        let right = rectangle::square(0.0, 0.0, 50.0);
        let right_pos = self.right_pos as f64;
        let ball = rectangle::square(0.0, 0.0, 10.0);
        let ball_x = self.ball_x as f64;
        let ball_y = self.ball_y as f64;
        let left_score = &(self.left_score.to_string())[..];
        let right_score = &(self.right_score.to_string())[..];
        let winner = self.winner;

        let mut glyph_cache =
            GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();

        self.gl_graphics.draw(args.viewport(), |c, gl| {
            if winner != Winner::None {
                clear(VICTORY_BACKGROUND, gl);
                if winner == Winner::Left {
                    text::Text::new_color(SCORE, 32)
                        .draw(
                            "Left wins!",
                            &mut glyph_cache,
                            &DrawState::default(),
                            c.transform
                                .trans(0.3 * args.window_size[0], 0.3 * args.window_size[1]),
                            gl,
                        )
                        .unwrap();
                } else {
                    text::Text::new_color(SCORE, 32)
                        .draw(
                            "Right wins!",
                            &mut glyph_cache,
                            &DrawState::default(),
                            c.transform
                                .trans(0.3 * args.window_size[0], 0.3 * args.window_size[1]),
                            gl,
                        )
                        .unwrap();
                }

                text::Text::new_color(SCORE, 32)
                    .draw(
                        "Press Esc to quit, R to restart",
                        &mut glyph_cache,
                        &DrawState::default(),
                        c.transform
                            .trans(0.2 * args.window_size[0], 0.7 * args.window_size[1]),
                        gl,
                    )
                    .unwrap();
                return;
            }
            clear(BACKGROUND, gl);

            rectangle(FOREGROUND, left, c.transform.trans(-40.0, left_pos), gl);
            rectangle(
                FOREGROUND,
                right,
                c.transform
                    .trans(args.window_size[0] as f64 - 10.0, right_pos),
                gl,
            );
            line_from_to(
                MIDDLE_LINE,
                1.0,
                [0.0, 0.0],
                [0.0, args.window_size[1] as f64],
                c.transform.trans((args.window_size[0] / 2.0) as f64, 0.0),
                gl,
            );
            text::Text::new_color(SCORE, 32)
                .draw(
                    left_score,
                    &mut glyph_cache,
                    &DrawState::default(),
                    c.transform
                        .trans(0.4 * args.window_size[0], 0.3 * args.window_size[1]),
                    gl,
                )
                .unwrap();
            text::Text::new_color(SCORE, 32)
                .draw(
                    right_score,
                    &mut glyph_cache,
                    &DrawState::default(),
                    c.transform
                        .trans(0.6 * args.window_size[0], 0.3 * args.window_size[1]),
                    gl,
                )
                .unwrap();
            rectangle(FOREGROUND, ball, c.transform.trans(ball_x, ball_y), gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.winner != Winner::None {
            return;
        }
        if (self.left_vel == 1 && self.left_pos < self.arena_height - 50.0)
            || (self.left_vel == -1 && self.left_pos >= 1.0)
        {
            self.left_pos += self.left_vel as f32 * self.speed_factor;
        }
        if (self.right_vel == 1 && self.right_pos < self.arena_height - 50.0)
            || (self.right_vel == -1 && self.right_pos >= 1.0)
        {
            self.right_pos += self.right_vel as f32 * self.speed_factor;
        }
        self.ball_x += self.vel_x as f32 * self.speed_factor;
        if self.ball_x > self.arena_width - 10.0 {
            self.vel_x = -self.vel_x;
            if self.ball_y < self.right_pos || self.ball_y > self.right_pos + 50.0 {
                self.left_score += 1;
                if self.left_score >= 5 {
                    println!("Left wins!");
                    self.reset_state();
                    self.winner = Winner::Left;
                }
                self.ball_x = self.arena_width / 2.0;
                self.ball_y = self.arena_height / 2.0;
            }
        }
        if self.ball_x < 1.0 {
            self.vel_x = -self.vel_x;
            if self.ball_y < self.left_pos || self.ball_y > self.left_pos + 50.0 {
                self.right_score += 1;
                if self.right_score >= 5 {
                    println!("Right wins!");
                    self.reset_state();
                    self.winner = Winner::Right;
                }
                self.ball_x = self.arena_width / 2.0;
                self.ball_y = self.arena_height / 2.0;
            }
        }

        self.ball_y += self.vel_y as f32 * self.speed_factor;
        if self.ball_y > self.arena_height - 10.0 || self.ball_y < 1.0 {
            self.vel_y = -self.vel_y;
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_vel = -1;
                }
                Key::Down => {
                    self.right_vel = 1;
                }
                Key::W => {
                    self.left_vel = -1;
                }
                Key::S => {
                    self.left_vel = 1;
                }
                Key::R => {
                    if self.winner != Winner::None {
                        self.reset_state();
                    }
                }
                Key::D1 | Key::NumPad1 => {
                    self.speed_factor = 1.0;
                }
                Key::D2 | Key::NumPad2 => {
                    self.speed_factor = 2.0;
                }
                Key::D3 | Key::NumPad3 => {
                    self.speed_factor = 3.0;
                }
                Key::D4 | Key::NumPad4 => {
                    self.speed_factor = 4.0;
                }
                Key::D5 | Key::NumPad5 => {
                    self.speed_factor = 5.0;
                }
                Key::D6 | Key::NumPad6 => {
                    self.speed_factor = 6.0;
                }
                Key::D7 | Key::NumPad7 => {
                    self.speed_factor = 7.0;
                }
                Key::D8 | Key::NumPad8 => {
                    self.speed_factor = 8.0;
                }
                Key::D9 | Key::NumPad9 => {
                    self.speed_factor = 9.0;
                }
                Key::D0 | Key::NumPad0 => {
                    self.speed_factor = 0.0;
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_vel = 0;
                }
                Key::Down => {
                    self.right_vel = 0;
                }
                Key::W => {
                    self.left_vel = 0;
                }
                Key::S => {
                    self.left_vel = 0;
                }
                _ => {}
            }
        }
    }

    fn reset_state(&mut self) {
        self.left_score = 0;
        self.left_pos = 1.0;
        self.left_vel = 0;
        self.right_score = 0;
        self.right_pos = 1.0;
        self.right_vel = 0;
        self.ball_x = 0.0;
        self.ball_y = 0.0;
        self.vel_x = 1;
        self.vel_y = 1;
        // self.speed_factor = 2.0;
        self.winner = Winner::None;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Rusty Pong", [512, 342])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl_graphics: GlGraphics::new(opengl),
        left_score: 0,
        left_pos: 1.0,
        left_vel: 0,
        right_score: 0,
        right_pos: 1.0,
        right_vel: 0,
        ball_x: 0.0,
        ball_y: 0.0,
        vel_x: 1,
        vel_y: 1,
        speed_factor: 2.0,
        arena_height: 0.0,
        arena_width: 0.0,
        winner: Winner::None,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
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

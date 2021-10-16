use std::{collections::LinkedList, iter::FromIterator};

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, ButtonEvent, ButtonState, EventLoop, EventSettings, Events, Key, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings};

mod snake;
mod direction;

use snake::Snake;
use direction::Direction;

pub struct App {
    gl: GlGraphics,
    snake: snake::Snake,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0., 1., 0., 1.];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.snake.update();
    }

    fn key_pressed(&mut self, button: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match button {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &_ => last_direction
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake { body: LinkedList::from_iter([(0, 0), (0, 1)]) ,direction: Direction::Right},
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                app.key_pressed(&k.button);
            }
        }
    }
}

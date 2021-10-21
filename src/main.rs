mod constants;
mod direction;
mod food;
mod snake;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    Button, ButtonEvent, ButtonState, EventLoop, EventSettings, Events, Key, RenderArgs,
    RenderEvent, UpdateArgs, UpdateEvent, Window, WindowSettings,
};

use direction::Direction;
use food::food_spawner::FoodSpawner;
use snake::Snake;

pub struct App {
    gl: GlGraphics,
    snake: Snake,
    food_spawner: FoodSpawner,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0., 1., 0., 1.];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.food_spawner.render(&mut self.gl, args);
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.snake.update();

        let snake_head = self.snake.get_head().unwrap();

        // Detect eaten foods
        let eaten_food_i = self
            .food_spawner
            .foods
            .iter()
            .position(|food| food.x == snake_head.0 && food.y == snake_head.1);

        match eaten_food_i {
            Some(i) => {
                self.food_spawner.foods.remove(i);
                self.snake.grow();
            }
            None => (),
        }

        // Respawn foods
        if self.food_spawner.foods.len() == 0 {
            self.food_spawner.spawn_to_regular();
        }
    }

    fn key_pressed(&mut self, button: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match button {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &_ => last_direction,
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [200, 200])
        .resizable(false)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::default(),
        food_spawner: FoodSpawner::new(2, window.size()),
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

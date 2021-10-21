use std::ops::Range;

use opengl_graphics::GlGraphics;
use piston::{RenderArgs, Size};
use rand::Rng;

use crate::constants::SQUARE_SIZE;
use crate::food::Food;

pub struct FoodSpawner {
    pub foods: Vec<Food>,
    pub regular_n: i32,
    window_size: Size,
}

impl FoodSpawner {
    /// Returns a new FoodSpawner with `regular_n` foods spawned in random positions.
    ///
    /// # Arguments
    ///
    /// * `regular_n` - Initial number of foods.
    /// * `window_size` - Size of the window, used to calculate spawn positions.
    pub fn new(regular_n: i32, window_size: Size) -> FoodSpawner {
        let mut food_spawner = FoodSpawner { foods: vec![], regular_n, window_size };

        food_spawner.spawn_to_regular();

        return food_spawner;
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLUE: [f32; 4] = [0., 0., 1., 1.];

        let squares: Vec<graphics::types::Rectangle> = self
            .foods
            .iter()
            .map(|food| {
                graphics::rectangle::square(
                    food.x as f64 * SQUARE_SIZE,
                    food.y as f64 * SQUARE_SIZE,
                    SQUARE_SIZE,
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            for square in squares {
                graphics::rectangle(BLUE, square, c.transform, gl);
            }
        })
    }

    /// Spawns new foods until `regular_n` is reached.
    pub fn spawn_to_regular(&mut self) {
        // TODO: avoid foods from spawning on top of snake.
        let mut rng = rand::thread_rng();

        for _ in 0..(self.regular_n - self.foods.len() as i32) {
            let x = rng.gen_range(Range {
                start: 0,
                end: (self.window_size.width / SQUARE_SIZE) as i32,
            });

            let y = rng.gen_range(Range {
                start: 0,
                end: (self.window_size.height / SQUARE_SIZE) as i32,
            });

            self.foods.push(Food { x, y });
        }
    }
}

use std::ops::Range;

use opengl_graphics::GlGraphics;
use piston::{RenderArgs, Size};
use rand::Rng;

use crate::constants::SQUARE_SIZE;
use crate::food::Food;

pub struct FoodSpawner {
    pub foods: Vec<Food>,
}

impl FoodSpawner {
    /// Returns a new FoodSpawner with `n` initial foods in random positions.
    ///
    /// # Arguments
    ///
    /// * `n` - Initial number of foods.
    /// * `window_size` - Size of the window, used to calculate spawn positions.
    pub fn new(n: i32, window_size: Size) -> FoodSpawner {
        let mut foods: Vec<Food> = vec![];

        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x = rng.gen_range(Range {
                start: 0,
                end: (window_size.width / SQUARE_SIZE) as i32,
            });

            let y = rng.gen_range(Range {
                start: 0,
                end: (window_size.height / SQUARE_SIZE) as i32,
            });

            foods.push(Food { x, y });
        }

        FoodSpawner { foods }
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
}

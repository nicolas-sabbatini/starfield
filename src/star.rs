use rand::prelude::random;

use super::{draw_circle, HEIGHT, WHITE, WIDTH};

#[derive(Debug)]
pub struct Star {
    x: f32,
    y: f32,
    size: f32,
    z: f32,
    cx: f32,
    cy: f32,
    csize: f32,
}

impl Star {
    pub fn new_rand() -> Self {
        Star {
            x: (random::<f32>() - 0.49) * WIDTH as f32,
            y: (random::<f32>() - 0.49) * HEIGHT as f32,
            size: (random::<f32>() * 2.0) + 1.0,
            z: WIDTH as f32,
            cx: 0.0,
            cy: 0.0,
            csize: 0.0,
        }
    }

    fn restart(&mut self) {
        self.x = (random::<f32>() - 0.49) * WIDTH as f32;
        self.y = (random::<f32>() - 0.49) * HEIGHT as f32;
        self.size = (random::<f32>() * 2.0) + 1.0;
        self.z = WIDTH as f32;
    }

    pub fn draw(&self) {
        draw_circle(
            self.cx + (WIDTH as f32 / 2.0),
            self.cy + (HEIGHT as f32 / 2.0),
            self.size * self.csize,
            WHITE,
        );
    }

    pub fn update(&mut self) {
        self.cx = map_range(
            self.x / self.z,
            -0.5,
            0.5,
            -WIDTH as f32 / 2.0,
            WIDTH as f32 / 2.0,
        );
        self.cy = map_range(
            self.y / self.z,
            -0.5,
            0.5,
            -HEIGHT as f32 / 2.0,
            HEIGHT as f32 / 2.0,
        );
        self.csize = map_range(self.z, WIDTH as f32, 0.0, 0.0, 2.0);
        self.z -= 8.0;

        if self.cx < WIDTH as f32 / -2.0
            || self.cx > WIDTH as f32 / 2.0
            || self.cy < HEIGHT as f32 / -2.0
            || self.cy > HEIGHT as f32 / 2.0
        {
            self.restart();
        }
    }
}

fn map_range(input: f32, in_min_range: f32, in_max_range: f32, out_min: f32, out_max: f32) -> f32 {
    (input - in_min_range) * ((out_max - out_min) / (in_max_range - in_min_range)) + out_min
}

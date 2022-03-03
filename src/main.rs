use macroquad::prelude::*;
use macroquad_canvas_2d::*;
use star::Star;

mod star;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn win_config() -> Conf {
    Conf {
        window_title: "Starfield".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(win_config)]
async fn main() {
    let canvas = Canvas2D::new(WIDTH as f32, HEIGHT as f32);
    let mut star_vec: Vec<Star> = Vec::new();
    for _i in 0..800 {
        let new_s = Star::new_rand();
        star_vec.push(new_s)
    }
    canvas.set_camera();
    clear_background(BLUE);
    loop {
        canvas.set_camera();
        {
            //        clear_background(BLACK);
            draw_rectangle(
                0.0,
                0.0,
                WIDTH as f32,
                HEIGHT as f32,
                Color::new(0.0, 0.0, 0.3, 0.2),
            );
            for s in star_vec.iter_mut() {
                s.update();
                s.draw();
            }
        }
        set_default_camera();
        // Craw canvas to screen
        clear_background(BLACK);
        canvas.draw_to_screen();
        next_frame().await
    }
}

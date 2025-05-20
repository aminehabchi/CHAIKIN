use minifb::{Key, MouseButton, Window, WindowOptions};
use std::thread;
use std::time::Duration;

mod geometrical_shapes;
use geometrical_shapes::*;

mod helpers;
use helpers::*;

fn main() {
    let mut buffer = vec![0_u32; WIDTH * HEIGHT];
    let mut window = Window::new("Moving Pixel", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut arr: Vec<Point> = Vec::new();

    let mut last_x: f32 = -1.0;
    let mut last_y: f32 = -1.0;
    #[allow(unused_variables)]
    let mut step: u8 = 1;
    let mut points: Vec<Point> = Vec::new();
    let mut animate = false;
    let mut count = 0;
    let mut go = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // clear window if click on space
        thread::sleep(Duration::from_millis(16));

        if window.is_key_down(Key::Space) {
            clear_window(&mut buffer);
            arr = Vec::new();
            points = Vec::new();
            step = 0;
            go = true;
            count = 0;
            animate = false;
        }
        if count == 20 {
            count = 0;
            go = true;
        }
        if !go {
            count += 1;
            continue;
        }

        //
        if window.is_key_down(Key::Enter) && !animate && arr.len() > 0 {
            points = arr.clone();
            animate = true;
            draw_lines(&points, &mut buffer);
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            go =false;
            continue;
        }

        if animate {
            go = false;
            step += 1;
            if step == 8 {
                step = 0;
                points = arr.clone();
            } else {
                points = chakin(points);
            }
            clear_window(&mut buffer);
            draw_lines(&points, &mut buffer);
            draw_points(&arr, &mut buffer);
        }

        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                if !animate && !(last_x == x && last_y == y) {
                    arr.push(Point { x, y });
                    last_x = x;
                    last_y = y;

                    if (x as usize) < WIDTH && (y as usize) < HEIGHT {
                        Circle::new(Point { x, y }, 3).draw(&mut buffer, WIDTH, HEIGHT, 0xFFFFFF);
                    }
                }
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

use minifb::{Key, MouseButton, Window, WindowOptions};
use std::thread;
use std::time::Duration;

mod geometrical_shapes;
use geometrical_shapes::*;

fn get_25(p1: Point, p2: Point) -> Point {
    let t = 0.25;
    Point {
        x: p1.x + t * (p2.x - p1.x),
        y: p1.y + t * (p2.y - p1.y),
    }
}
fn clear_window(buffer: &mut [u32]) {
    for pixel in buffer.iter_mut() {
        *pixel = 0x000000;
    }
}

fn main() {
    let width: usize = 1000;
    let height: usize = 1000;

    let mut buffer = vec![0_u32; width * height];
    let mut window = Window::new("Moving Pixel", width, height, WindowOptions::default()).unwrap();

    let mut arr: Vec<Point> = Vec::new();

    let mut last_x: f32 = -1.0;
    let mut last_y: f32 = -1.0;

    let mut step: u8 = 0;
    let mut points: Vec<Point> = Vec::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // clear window if click on space
        if window.is_key_down(Key::Space) {
            clear_window(&mut buffer);
            arr = Vec::new();
            points = Vec::new();
            step = 0;
        }

        if arr.len() == 4 && step < 8 {
            let mut new_points: Vec<Point> = Vec::new();

            new_points.push(points[0].clone());
            for i in 1..points.len() {
                let q = get_25(points[i - 1].clone(), points[i].clone());
                let r = get_25(points[i].clone(), points[i - 1].clone());

                new_points.push(q);
                new_points.push(r);
            }
            new_points.push(points[points.len() - 1].clone());

            points = new_points.clone();
            step += 1;

            //////////////////
            clear_window(&mut buffer);
            for i in 1..points.len() {
                Line::new(points[i - 1].clone(), points[i].clone()).draw(
                    &mut buffer,
                    width,
                    height,
                    0xFFFFFF,
                )
            }
        }

        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                if arr.len() < 4 && !(last_x == x && last_y == y) {
                    arr.push(Point { x, y });
                    last_x = x;
                    last_y = y;

                    if (x as usize) < width && (y as usize) < height {
                        Circle::new(Point { x, y }, 3).draw(&mut buffer, width, height, 0xFFFFFF);
                    }

                    if arr.len() == 4 {
                        Line::new(arr[0].clone(), arr[1].clone()).draw(
                            &mut buffer,
                            width,
                            height,
                            0xFFFFFF,
                        );
                        Line::new(arr[1].clone(), arr[2].clone()).draw(
                            &mut buffer,
                            width,
                            height,
                            0xFFFFFF,
                        );
                         Line::new(arr[2].clone(), arr[3].clone()).draw(
                            &mut buffer,
                            width,
                            height,
                            0xFFFFFF,
                        );
                        points = arr.clone();
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
        thread::sleep(Duration::from_millis(32));
    }
}

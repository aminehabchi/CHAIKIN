use minifb::{Window, WindowOptions, Key, MouseMode}; // <-- Added MouseMode
use std::time::Duration;
use std::thread;

struct Point{
    x :f32,
    y :f32,
}

fn next_move(p1,p2:)

fn main() {
    let width = 1000;
    let height = 1000;

    let mut buffer = vec![0u32; width * height];
    let mut window = Window::new("Moving Pixel", width, height, WindowOptions::default()).unwrap();
    
    let mut mouse_point: Point = Point{
        x:-1.0,
        y:-1.0,
    };

    // let mut point:Vec<point>=Vec::new();

    let mut head:Point=Point{
        x:(width/2) as f32,
        y:(height/2) as f32,
    };
    buffer[(head.y as usize)*width+(head.x as usize)]=0xFFFFFF;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            mouse_point.x=x;
            mouse_point.y=y;
        }



        window.update_with_buffer(&buffer, width, height).unwrap();
        thread::sleep(Duration::from_millis(8));
    }
}

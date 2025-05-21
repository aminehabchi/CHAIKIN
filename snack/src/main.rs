use minifb::{Window, WindowOptions, Key, MouseMode};
use std::time::Duration;
use std::thread;
mod helpers;
use helpers::*;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut window = Window::new("Moving Pixel", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    
    let mut mouse_point: Point = Point {
        x: -1.0,
        y: -1.0,
    };
    
    let mut head: Point = Point {
        x: (WIDTH/2) as f32,
        y: (HEIGHT/2) as f32,
    };
    
    let mut all_points: Vec<Point> = Vec::new();
    
    let mut last_direction: Vec<Direction> = Vec::new();
    let mut frame = 0f32;
    
    // Snake thickness configuration
    let snake_thickness = 2; // Adjust this value as needed: 1 = 3x3 points, 2 = 5x5 points, etc.
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            mouse_point.x = x;
            mouse_point.y = y;
        }
        
        next_move(&mut last_direction, &mouse_point, &mut head);
        
        if all_points.len() == 0 || head.x != all_points[all_points.len()-1].x || head.y != all_points[all_points.len()-1].y {
            all_points.push(head.clone());
            
            if all_points.len() == 200 {
                buffer.fill(0);
                all_points.remove(0);
                
                // Draw the tear and ribs
                draw_tear(&mut buffer, head, last_direction[last_direction.len()-1]);
                draw_ribs_from_spine(&all_points, &mut buffer, 0xFFFFFF, frame);
                
                // Draw the snake with thickness
                draw_thick_snake(&mut buffer, &all_points, snake_thickness, 0xFFFFFF);
            } else {
                // Draw the snake with thickness
                draw_thick_snake(&mut buffer, &all_points, snake_thickness, 0xFFFFFF);
            }
            
        }
        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        frame += 1.;
        thread::sleep(Duration::from_millis(16));
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    LeftUp,
    LeftDown,
    Up,
    Down,
    Right,
    RightUp,
    RightDown,
    SamePlace,
}

fn is_sharp_turn(d1: Direction, d2: Direction) -> bool {
    matches!(
        (d1, d2),
        // 180° turns (reversal)
        (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::LeftUp, Direction::RightDown)
            | (Direction::RightDown, Direction::LeftUp)
            | (Direction::LeftDown, Direction::RightUp)
            | (Direction::RightUp, Direction::LeftDown)

        // ±90° orthogonal turns
            | (Direction::Left, Direction::Up)
            | (Direction::Left, Direction::Down)
            | (Direction::Right, Direction::Up)
            | (Direction::Right, Direction::Down)
            | (Direction::Up, Direction::Left)
            | (Direction::Up, Direction::Right)
            | (Direction::Down, Direction::Left)
            | (Direction::Down, Direction::Right)

        // Diagonal ⇄ straight
            | (Direction::LeftUp, Direction::Right)
            | (Direction::LeftUp, Direction::Down)
            | (Direction::RightUp, Direction::Left)
            | (Direction::RightUp, Direction::Down)
            | (Direction::LeftDown, Direction::Right)
            | (Direction::LeftDown, Direction::Up)
            | (Direction::RightDown, Direction::Left)
            | (Direction::RightDown, Direction::Up)

        // Straight ⇄ diagonal
            | (Direction::Left, Direction::RightUp)
            | (Direction::Left, Direction::RightDown)
            | (Direction::Right, Direction::LeftUp)
            | (Direction::Right, Direction::LeftDown)
            | (Direction::Up, Direction::LeftDown)
            | (Direction::Up, Direction::RightDown)
            | (Direction::Down, Direction::LeftUp)
            | (Direction::Down, Direction::RightUp)
    )
}

pub fn next_move(last_direction: &mut Vec<Direction>, mouse_point: &Point, head: &mut Point) {
    let mut all_directions: Vec<(Direction, f32, f32, f32)> = vec![
        (Direction::Left, -1.0, 0.0, 0.0),
        (Direction::LeftUp, -1.0, -1.0, 0.0),
        (Direction::LeftDown, -1.0, 1.0, 0.0),
        (Direction::Up, 0.0, -1.0, 0.0),
        (Direction::Down, 0.0, 1.0, 0.0),
        (Direction::Right, 1.0, 0.0, 0.0),
        (Direction::RightUp, 1.0, -1.0, 0.0),
        (Direction::RightDown, 1.0, 1.0, 0.0),
        (Direction::SamePlace, 0.0, 0.0, 0.0),
    ];

    let last = last_direction.last().copied();

    for dir in &mut all_directions {
        let target_x = head.x + dir.1;
        let target_y = head.y + dir.2;

        let mut d = distance((target_x, target_y), (mouse_point.x, mouse_point.y));

        // Penalize opposite direction
        if let Some(last_dir) = last {
            if is_sharp_turn(last_dir, dir.0) {
                d += 100.0; // discourage sudden reversal
            }
        }

        dir.3 = d;
    }

    // Sort by biased distance
    all_directions.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

    for (direction, dx, dy, _) in all_directions {
        let next_x = head.x + dx;
        let next_y = head.y + dy;

        // Bounds check
        if next_x >= WIDTH as f32 || next_x < 0.0 || next_y >= HEIGHT as f32 || next_y < 0.0 {
            continue;
        }

        // Make the move
        head.x = next_x;
        head.y = next_y;
        last_direction.push(direction);
        break;
    }
}

fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}


pub fn draw_tear(buffer: &mut [u32], point: Point, direction: Direction) {
    let radius :f32= 5.0;          // Radius of the tear head circle
    let taper_length = 15;     // Length of the taper tail
    let color = 0xFFFFFF;      // White color for border

    let (dx, dy) = direction_vector(direction);

    // Draw circular border of the head (using midpoint circle algorithm approximation)
    let radius_i = radius as i32;
    for y in -radius_i..=radius_i {
        for x in -radius_i..=radius_i {
            let dist_sq = (x * x + y * y) as f32;
            if dist_sq >= (radius - 1.0).powi(2) && dist_sq <= radius.powi(2) {
                let px = point.x + x as f32;
                let py = point.y + y as f32;
                set_pixel(buffer, px as i32, py as i32, color);
            }
        }
    }
}
pub fn direction_vector(direction: Direction) -> (f32, f32) {
    match direction {
        Direction::Left => (-1.0, 0.0),
        Direction::LeftUp => (-1.0, -1.0),
        Direction::LeftDown => (-1.0, 1.0),
        Direction::Up => (0.0, -1.0),
        Direction::Down => (0.0, 1.0),
        Direction::Right => (1.0, 0.0),
        Direction::RightUp => (1.0, -1.0),
        Direction::RightDown => (1.0, 1.0),
        Direction::SamePlace => (0.0, 0.0),
    }
}

pub fn set_pixel(buffer: &mut [u32], x: i32, y: i32, color: u32) {
    if x >= 0 && y >= 0 && (x as usize) < WIDTH && (y as usize) < HEIGHT {
        buffer[(y as usize) * WIDTH + (x as usize)] = color;
    }
}


fn draw_line_f32(
    start: (f32, f32),
    end: (f32, f32),
    buffer: &mut Vec<u32>,
    color: u32,
) {
    let (mut x0, mut y0) = start;
    let (x1, y1) = end;

    let dx = x1 - x0;
    let dy = y1 - y0;
    let steps = dx.abs().max(dy.abs()) as usize;

    if steps == 0 { return; }

    let step_x = dx / steps as f32;
    let step_y = dy / steps as f32;

    for _ in 0..=steps {
        let xi = x0.round() as isize;
        let yi = y0.round() as isize;
        if xi >= 0 && yi >= 0 && xi < WIDTH as isize && yi < HEIGHT as isize {
            buffer[(yi as usize) * WIDTH + (xi as usize)] = color;
        }
        x0 += step_x;
        y0 += step_y;
    }
}



use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
pub fn draw_ribs_from_spine(all_points: &Vec<Point>, buffer: &mut Vec<u32>, color: u32, frame: f32) {
    let spacing = 10;
    let mut rng = SmallRng::seed_from_u64(42); // consistent randomness

    for (i, idx) in (1..all_points.len()).step_by(spacing).enumerate() {
        if idx > 190 { break; }

        let p0 = all_points[idx - 1];
        let p1 = all_points[idx];

        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 { continue; }

        let dir_x = dx / len;
        let dir_y = dy / len;

        let base_perp = (-dir_y, dir_x);

        // Phase offset that changes per rib for nice wave delay
        let wave_phase = frame * 0.3 + i as f32 * 0.8;  // Increase 0.8 for bigger delay between ribs

        // Add some small random jitter to wave_phase per rib if you want more natural randomness
        // let random_phase = rng.gen_range(0.0..std::f32::consts::TAU);
        // let wave_phase = wave_phase + random_phase;

        let wave_amplitude = 5.0;

        let perp = (
            base_perp.0 + wave_amplitude * wave_phase.sin() * dir_x * 0.1,
            base_perp.1 + wave_amplitude * wave_phase.sin() * dir_y * 0.1,
        );

        let initial_rib = make_initial_rib(p1, perp, (dir_x, dir_y), 40.0);
        let curved_rib = chaikin_curve(&initial_rib, 3);

        draw_polyline(&curved_rib, buffer, color);
    }
}





fn chaikin_curve(points: &Vec<Point>, iterations: usize) -> Vec<Point> {
    if iterations == 0 || points.len() < 2 {
        return points.clone();
    }

    let mut result = points.clone();

    for _ in 0..iterations {
        let mut new_points = Vec::with_capacity(result.len() * 2);

        for i in 0..result.len() - 1 {
            let p0 = result[i];
            let p1 = result[i + 1];

            // Q point: 3/4 of p0 and 1/4 of p1
            let q = Point {
                x: 0.75 * p0.x + 0.25 * p1.x,
                y: 0.75 * p0.y + 0.25 * p1.y,
            };

            // R point: 1/4 of p0 and 3/4 of p1
            let r = Point {
                x: 0.25 * p0.x + 0.75 * p1.x,
                y: 0.25 * p0.y + 0.75 * p1.y,
            };

            new_points.push(q);
            new_points.push(r);
        }

        result = new_points;
    }

    result
}

fn make_initial_rib(center: Point, perp: (f32, f32), spine_dir: (f32, f32), length: f32) -> Vec<Point> {
    let left = Point {
        x: center.x - perp.0 * length / 2.0,
        y: center.y - perp.1 * length / 2.0,
    };

    let control = Point {
        x: center.x + spine_dir.0 * length / 3.0,
        y: center.y + spine_dir.1 * length / 3.0,
    };

    let right = Point {
        x: center.x + perp.0 * length / 2.0,
        y: center.y + perp.1 * length / 2.0,
    };

    vec![left, control, right]
}

fn draw_polyline(points: &Vec<Point>, buffer: &mut Vec<u32>, color: u32) {
    for i in 0..points.len() - 1 {
        draw_line_f32((points[i].x, points[i].y), (points[i+1].x, points[i+1].y), buffer, color);
    }
}

// Add this function to your helpers.rs file
pub fn draw_thick_point(buffer: &mut [u32], x: f32, y: f32, thickness: i32, color: u32) {
    let x_i = x as i32;
    let y_i = y as i32;
    
    // Draw a square of the specified thickness around the point
    for dy in -thickness..=thickness {
        for dx in -thickness..=thickness {
            let nx = x_i + dx;
            let ny = y_i + dy;
            
            // Check if within bounds
            if nx >= 0 && ny >= 0 && nx < WIDTH as i32 && ny < HEIGHT as i32 {
                buffer[(ny as usize) * WIDTH + (nx as usize)] = color;
            }
        }
    }
}

// Add this function to draw the entire snake with thick points
pub fn draw_thick_snake(buffer: &mut [u32], points: &Vec<Point>, thickness: i32, color: u32) {
    for point in points {
        draw_thick_point(buffer, point.x, point.y, thickness, color);
    }
}
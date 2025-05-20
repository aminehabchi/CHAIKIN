use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
use std::thread;

fn draw_circle(
    cx: isize,
    cy: isize,
    r: isize,
    buffer: &mut [u32],
    width: usize,
    height: usize,
    color: u32,
) {
    let mut x = 0;
    let mut y = r;
    let mut d = 1 - r;

    plot_circle_points(cx, cy, x, y, buffer, width, height, color);

    while x < y {
        x += 1;
        if d < 0 {
            d += 2 * x + 1;
        } else {
            y -= 1;
            d += 2 * (x - y) + 1;
        }
        plot_circle_points(cx, cy, x, y, buffer, width, height, color);
    }
}

fn plot_circle_points(
    cx: isize,
    cy: isize,
    x: isize,
    y: isize,
    buffer: &mut [u32],
    width: usize,
    height: usize,
    color: u32,
) {
    let points = [
        (cx + x, cy + y),
        (cx - x, cy + y),
        (cx + x, cy - y),
        (cx - x, cy - y),
        (cx + y, cy + x),
        (cx - y, cy + x),
        (cx + y, cy - x),
        (cx - y, cy - x),
    ];

    for &(px, py) in &points {
        if px >= 0 && py >= 0 && (px as usize) < width && (py as usize) < height {
            let idx = py as usize * width + px as usize;
            if idx < buffer.len() {
                buffer[idx] = color;
            }
        }
    }
}



fn draw_line(
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    buffer: &mut [u32],
    width: usize,
    height: usize,
    color: u32,
) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy; // error value

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as usize) < width && (y0 as usize) < height {
            buffer[y0 as usize * width + x0 as usize] = color;
        }
        
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}


fn next_move(angle_deg: f64, x: f64, y: f64, cx: f64, cy: f64) -> (isize, isize) {
    let angle_rad = angle_deg.to_radians(); // Convert degrees to radians
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    let x_rotated = cos_theta * (x - cx) - sin_theta * (y - cy) + cx;
    let y_rotated = sin_theta * (x - cx) + cos_theta * (y - cy) + cy;

    (x_rotated.round() as isize, y_rotated.round() as isize)
}



fn main() {
const COLOR_FADE: [u32; 40] = [
0xFF000000, // dark green
0xFF003900,
0xFF004000,
0xFF004600,
0xFF004C00,
0xFF005200,
0xFF005800,
0xFF005E00,
0xFF006400,
0xFF006A00,
0xFF007000,
0xFF007600,
0xFF007C00,
0xFF008200,
0xFF008800,
0xFF008E00,
0xFF009400,
0xFF009A00,
0xFF00A000,
0xFF00A600,
0xFF00AC00,
0xFF00B200,
0xFF00B800,
0xFF00BE00,
0xFF00C400,
0xFF00CA00,
0xFF00D000,
0xFF00D600,
0xFF00DC00,
0xFF00E200,
0xFF00E800,
0xFF00EE00,
0xFF00F400,
0xFF00FA00,
0xFF00FF00,
0xFF00FA00,
0xFF00FA00,
0xFF00FA00,
0xFF00FA00,
0xFF00FA00,
];


    let width = 1000;
    let height = 1000;

    let mut buffer = vec![0u32; width * height];
    let mut window = Window::new("Moving Pixel", width, height, WindowOptions::default()).unwrap();
    

    let cx:isize=(width/2) as isize;
    let cy:isize=(height/2) as isize;
    let radius:isize=400;

    let mut x:isize=cx;
    let mut y:isize=cy-radius;

    let mut arr:Vec<(isize,isize)>=Vec::new();

    
    

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        (x,y)=next_move(0.5,x as f64,y as f64,cx as f64,cy as f64);

        arr.push((x,y));

        if arr.len()>=40{
            for i in 0..arr.len(){
                draw_line(cx, cy, arr[i].0, arr[i].1, &mut buffer, width, height, COLOR_FADE[i]);
            }
            arr.remove(0);
        }

        // display circles
        draw_circle(cx,cy,radius,&mut buffer, width, height, COLOR_FADE[39]);
        draw_circle(cx,cy,radius/4,&mut buffer, width, height, COLOR_FADE[39]);
        draw_circle(cx,cy,2*radius/4,&mut buffer, width, height, COLOR_FADE[39]);
        draw_circle(cx,cy,3*radius/4,&mut buffer, width, height, COLOR_FADE[39]);

        // display +
        draw_line(cx, cy-radius, cx, cy+radius, &mut buffer, width, height, COLOR_FADE[39]);
        draw_line(cx-radius, cy, cx+radius, cy, &mut buffer, width, height, COLOR_FADE[39]);
        
        // display X
        let diag = (radius as f32 * 0.7071).round() as isize;
        draw_line(cx - diag, cy - diag, cx + diag, cy + diag, &mut buffer, width, height, COLOR_FADE[39]);
        draw_line(cx - diag, cy + diag, cx + diag, cy - diag, &mut buffer, width, height, COLOR_FADE[39]);

        window.update_with_buffer(&buffer, width, height).unwrap();
        thread::sleep(Duration::from_millis(16));
    }
}
// --------------------------------------- Point impl and Drawable -------------------------------
#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

// --------------------------------------- Line impl and Drawable -------------------------------
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
    pub fn draw(&self, buffer: &mut [u32], width: usize, height: usize, color: u32) {
        let mut x0 = self.p1.x as isize;
        let mut y0 = self.p1.y as isize;
        let x1 = self.p2.x as isize;
        let y1 = self.p2.y as isize;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        while x0 != x1 || y0 != y1 {
            if x0 >= 0 && y0 >= 0 && (x0 as usize) < width && (y0 as usize) < height {
                buffer[y0 as usize * width + x0 as usize] = color;
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

        // Final point
        if x1 >= 0 && y1 >= 0 && (x1 as usize) < width && (y1 as usize) < height {
            buffer[y1 as usize * width + x1 as usize] = color;
        }
    }
}

// --------------------------------------- Circle impl and Drawable -------------------------------
pub struct Circle {
    pub center: Point,
    pub radius: usize,
}

impl Circle {
    pub fn new(center: Point, radius: usize) -> Self {
        Circle { center, radius }
    }

    pub fn draw(&self, buffer: &mut [u32], width: usize, height: usize, color: u32) {
        let cx = self.center.x as isize;
        let cy = self.center.y as isize;
        let mut x = self.radius as isize;
        let mut y = 0;
        let mut d = 1 - x;

        fn put_pixel(
            buffer: &mut [u32],
            width: usize,
            height: usize,
            x: isize,
            y: isize,
            color: u32,
        ) {
            if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
                buffer[y as usize * width + x as usize] = color;
            }
        }

        while x >= y {
            // Octant symmetry
            put_pixel(buffer, width, height, cx + x, cy + y, color);
            put_pixel(buffer, width, height, cx + y, cy + x, color);
            put_pixel(buffer, width, height, cx - y, cy + x, color);
            put_pixel(buffer, width, height, cx - x, cy + y, color);
            put_pixel(buffer, width, height, cx - x, cy - y, color);
            put_pixel(buffer, width, height, cx - y, cy - x, color);
            put_pixel(buffer, width, height, cx + y, cy - x, color);
            put_pixel(buffer, width, height, cx + x, cy - y, color);

            y += 1;
            if d < 0 {
                d += 2 * y + 1;
            } else {
                x -= 1;
                d += 2 * (y - x) + 1;
            }
        }
    }
}

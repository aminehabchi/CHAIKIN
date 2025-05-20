use super::geometrical_shapes::*;

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 1000;

pub fn get_25(p1: Point, p2: Point) -> Point {
    let t = 0.25;
    Point {
        x: p1.x + t * (p2.x - p1.x),
        y: p1.y + t * (p2.y - p1.y),
    }
}

pub fn clear_window(buffer: &mut [u32]) {
    for pixel in buffer.iter_mut() {
        *pixel = 0x000000;
    }
}

pub fn chakin(points: Vec<Point>) -> Vec<Point> {
    let mut new_points: Vec<Point> = Vec::new();
    new_points.push(points[0].clone());
    for i in 1..points.len() {
        let q = get_25(points[i - 1].clone(), points[i].clone());
        let r = get_25(points[i].clone(), points[i - 1].clone());

        new_points.push(q);
        new_points.push(r);
    }
    new_points.push(points[points.len() - 1].clone());

    new_points.clone()
}

pub fn draw_lines(points: &Vec<Point>, buffer: &mut [u32]) {
    for i in 1..points.len() {
        Line::new(points[i - 1].clone(), points[i].clone()).draw(buffer, WIDTH, HEIGHT, 0xFFFFFF)
    }
}
pub fn draw_points(points: &Vec<Point>, buffer: &mut [u32]) {
    for i in 0..points.len()  {
        Circle::new(points[i].clone(),3).draw(buffer, WIDTH, HEIGHT, 0xFFFFFF);
    }
}

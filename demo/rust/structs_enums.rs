struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        let x_squared = (self.x * self.x) as f64;
        let y_squared = (self.y * self.y) as f64;
        (x_squared + y_squared).sqrt()
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let point = Point::new(3, 4);
    let distance = point.distance_from_origin();
    println!("The distance from the origin is {}", distance);

    let color = Color::Green;
    match color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
    }
}

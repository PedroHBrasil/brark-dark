trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

struct Square {
    side_length: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side length {}", self.side_length);
    }
}

fn draw_all_shapes<T: Drawable>(shapes: &[T]) {
    for shape in shapes {
        shape.draw();
    }
}

fn main() {
    let shapes: [Box<dyn Drawable>; 2] = [
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side_length: 3.0 }),
    ];
    draw_all_shapes(&shapes);
}

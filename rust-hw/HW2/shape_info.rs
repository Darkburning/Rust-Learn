use std::f64;
use std::f64::consts::PI;
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {
    fn new(l: f64, w: f64) -> Self {
        Self {
            length: l,
            width: w,
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(r: f64) -> Self {
        Self { radius: r }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}
#[derive(Debug)]
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // 海伦公式
        let p = self.perimeter() / 2.0;
        f64::sqrt(p * (p - self.a) * (p - self.b) * (p - self.c))
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn print_shape_info<T: Shape + std::fmt::Debug>(shape: T) {
    println!(
        "Shape: {:?}, Area: {}, Perimeter: {}",
        shape,
        shape.area(),
        shape.perimeter()
    );
}

fn main() {
    let t = Triangle::new(2.0, 2.0, 2.0);
    let c = Circle::new(2.0);
    let r = Rectangle::new(2.0, 2.0);
    print_shape_info(t);
    print_shape_info(c);
    print_shape_info(r);
}

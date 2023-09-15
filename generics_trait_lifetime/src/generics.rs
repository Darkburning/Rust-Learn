use std::fmt;

struct OneTypePoint<T> {
    x: T,
    y: T,
}
struct TwoTypePoint<T, U> {
    x: T,
    y: U,
}

impl TwoTypePoint<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum MyEnum<T> {
    VariantA(T),
    VariantB(T, T),
    VariantC,
}

impl<T: fmt::Display> fmt::Display for OneTypePoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: fmt::Display, U: fmt::Display> fmt::Display for TwoTypePoint<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn generics_demo() {
    let p1 = OneTypePoint { x: 1, y: 2 };
    let p2 = TwoTypePoint { x: 1.1, y: 2.123 };
    let p3 = TwoTypePoint { x: "hello", y: 2.1 };
    println!("OneTypePoint: {}, TwoTypePoint: {}", p1, p2);
    println!("distance from origin: {}", p2.distance_from_origin());
    // no method named `distance_from_origin` found for struct `TwoTypePoint<&str, {float}>` in the current scope
    //  the method was found for `TwoTypePoint<f64, f64>`
    // println!("distance from origin: {}", p3.distance_from_origin());
}

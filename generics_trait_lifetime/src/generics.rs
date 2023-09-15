use std::fmt;

struct OneTypePoint<T> {
    x: T,
    y: T,
}

impl<T: fmt::Display> fmt::Display for OneTypePoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct TwoTypePoint<T, U> {
    x: T,
    y: U,
}

impl<T: fmt::Display, U: fmt::Display> fmt::Display for TwoTypePoint<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn generics_demo() {
    let p1 = OneTypePoint { x: 1, y: 2 };
    let p2 = TwoTypePoint { x: 1, y: 2.123 };
    println!("OneTypePoint: {}, TwoTypePoint: {}", p1, p2);
}

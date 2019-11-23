mod fake_module {
    use pub_impl::pub_impl;

    pub struct Point{
        x: f32,
        y: f32,
    }

    #[pub_impl]
    impl Point {
        pub fn new(x: f32, y: f32) -> Self {
            Point { x, y }
        }
    }
}

use fake_module::Point;

fn main() {
    let point = Point::new(3.0, 4.0);
}

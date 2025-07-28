#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center_x: f64, center_y: f64, _radius: f64) -> Self {
        Self {
            center: Point(center_x, center_y),
            radius: _radius,
        }
    }
    pub fn intersect(&self, other: Circle) -> bool {
        let dx = self.center.0 - other.center.0;
        let dy = self.center.1 - other.center.1;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < self.radius + other.radius
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let x = self.0 - other.0;
        let y = self.1 - other.1;
        (x * x + y * y).sqrt()
    }
}

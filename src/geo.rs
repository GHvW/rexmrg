

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}


pub struct Feature {
    point: Point,
    value: f64
}

impl Feature {

    pub fn new(point: Point, value: f64) -> Self {
        Feature { point, value }
    }

    pub fn csv_row(&self) -> String {
        // long lat value
        String::from(format!("{},{},{}", self.point.x, self.point.y, self.value))
    }
}

use trap::Vector2;

pub struct Rectangle {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}


impl Rectangle {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Rectangle {
        Rectangle {
            left: if left < right {left} else {right},
            right: if left < right {right} else {left},
            top: if top > bottom {top} else {bottom},
            bottom: if top > bottom {bottom} else {top},
        }
    }


    /// Creates a rectangle from two corners
    pub fn from_points(a: Vector2, b: Vector2) -> Rectangle {
        let (min_x, max_x) = if a.x < b.x {(a.x, b.x)} else {(b.x, a.x)};
        let (min_y, max_y) = if a.y < b.y {(a.y, b.y)} else {(b.y, a.y)};

        Rectangle {
            left: min_x,
            right: max_x,
            top: max_y,
            bottom: min_y
        }
    }


    /// Returns true if this rectangle contains a point
    pub fn contains(&self, point: Vector2) -> bool {
        self.left <= point.x && point.x <= self.right &&
            self.bottom <= point.y && point.y <= self.top
    }


    /// Adds a margin to the rectangle
    pub fn add_margin(&mut self, margin: f64) {
        self.left -= margin;
        self.right += margin;
        self.top += margin;
        self.bottom -= margin;
    }
}
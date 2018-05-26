
use trap::Vector2;
use renderer::Triangulate;
use renderer::Triangles;
use collision::Collide;

#[derive(Copy, Clone)]
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


    /// Returns true of two rectangles occupy the same space
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.left < other.right && other.left < self.right &&
            self.bottom < other.top && other.bottom < self.top
    }
}



impl super::Shape for Rectangle {
    fn translate(&mut self, amount: Vector2) {
        self.left += amount.x;
        self.right += amount.x;
        self.top += amount.y;
        self.bottom += amount.y;
    }

    fn center(&self) -> Vector2 {
        Vector2 {
            x: (self.left + self.right) * 0.5,
            y: (self.top + self.bottom) * 0.5,
        }
    }
}


impl Triangulate for Rectangle {
    fn get_triangles(&self) -> Triangles {
        Triangles::IndexedTriangles(
            vec![
                Vector2::new(self.left, self.top),
                Vector2::new(self.right, self.top),
                Vector2::new(self.right, self.bottom),
                Vector2::new(self.left, self.bottom)
            ],
            vec![
                0, 1, 2,
                2, 3, 0
            ]
        )
    }
}


impl Collide for Rectangle {
    fn get_farthest_point(&self, axis: Vector2) -> Vector2 {
        if self.bottom < self.top {
            if axis.x > 0.0 {
                if axis.y > 0.0 {
                    Vector2::new(self.right, self.top)
                } else {
                    Vector2::new(self.right, self.bottom)
                }
            } else {
                if axis.y > 0.0 {
                    Vector2::new(self.left, self.top)
                } else {
                    Vector2::new(self.left, self.bottom)
                }
            }
        } else {
            unimplemented!()
        }
    }

    fn get_bounding_box(&self) -> Rectangle {
        self.clone()
    }
}

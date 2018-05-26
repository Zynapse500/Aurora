use trap::Vector2;
use renderer::Triangulate;
use renderer::Triangles;
use collision::Collide;
use shapes::Rectangle;


#[derive(Copy, Clone)]
pub struct Circle {
    pub center: Vector2,
    pub radius: f64
}

impl Circle {
    pub fn new(center: Vector2, radius: f64) -> Circle {
        Circle { center, radius }
    }
}


impl super::Shape for Circle {
    fn translate(&mut self, amount: Vector2) {
        self.center += amount;
    }

    fn center(&self) -> Vector2 {
        self.center
    }
}



impl Triangulate for Circle {
    fn get_triangles(&self) -> Triangles {
        use std::f64::consts::PI;
        const SEGMENTS: u32 = 64;

        let mut points = Vec::new();
        let mut indices = Vec::new();

        points.push(self.center);

        for i in 0..SEGMENTS {
            let (dy, dx) = (i as f64 / SEGMENTS as f64 * PI * 2.0).sin_cos();

            points.push(self.center + self.radius * Vector2::new(dx, dy));

            indices.push(0);
            indices.push(i + 1);
            indices.push((i + 1) % SEGMENTS + 1);
        }

        Triangles::IndexedTriangles(
            points,
            indices
        )
    }
}


impl Collide for Circle {
    fn get_farthest_point(&self, axis: Vector2) -> Vector2 {
        self.center + self.radius * axis.normal()
    }

    fn get_bounding_box(&self) -> Rectangle {
        Rectangle {
            left: self.center.x - self.radius,
            right: self.center.x + self.radius,
            top: self.center.y + self.radius,
            bottom: self.center.y - self.radius,
        }
    }
}

use trap::Vector2;

pub struct ConvexHull {
    pub points: Vec<Vector2>
}

impl ConvexHull {
    pub fn new(points: Vec<Vector2>) -> ConvexHull {
        ConvexHull {
            points
        }
    }


    pub fn average(&self) -> Vector2 {
        let mut sum = Vector2::new(0.0, 0.0);

        for point in self.points.iter() {
            sum += *point;
        }

        sum / self.points.len() as f64
    }


    pub fn translate(&mut self, amount: Vector2) {
        for point in self.points.iter_mut() {
            *point += amount;
        }
    }
}

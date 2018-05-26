use trap::Vector2;
use renderer::Triangulate;
use renderer::Triangles;
use collision::Collide;
use shapes::Rectangle;


#[derive(Clone)]
pub struct ConvexHull {
    pub points: Vec<Vector2>
}

impl ConvexHull {
    pub fn new(points: Vec<Vector2>) -> ConvexHull {
        ConvexHull {
            points
        }
    }
}


impl super::Shape for ConvexHull {
    fn translate(&mut self, amount: Vector2) {
        for point in self.points.iter_mut() {
            *point += amount;
        }
    }


    fn center(&self) -> Vector2 {
        let mut sum = Vector2::new(0.0, 0.0);

        for point in self.points.iter() {
            sum += *point;
        }

        sum / self.points.len() as f64
    }
}



impl Triangulate for ConvexHull {
    fn get_triangles(&self) -> Triangles {
        Triangles::IndexedTriangles(
            self.points.clone(),
            {
                let mut indices = Vec::new();
                indices.reserve((self.points.len() - 2) * 3);
                for i in 1..self.points.len()-1 {
                    indices.push(0);
                    indices.push(i as u32);
                    indices.push(i as u32 + 1);
                }

                indices
            }
        )
    }
}


impl Collide for ConvexHull {
    fn get_farthest_point(&self, axis: Vector2) -> Vector2 {
        let mut index = 0;
        let mut max_projection = axis.dot(self.points[0]);

        for i in 1..self.points.len() {
            let projection = axis.dot(self.points[i]);
            if projection > max_projection {
                index = i;
                max_projection = projection;
            }
        }

        self.points[index]
    }

    fn get_bounding_box(&self) -> Rectangle {
        use std::f64::INFINITY;

        let mut min = Vector2::new(INFINITY, INFINITY);
        let mut max = Vector2::new(-INFINITY, -INFINITY);

        for point in self.points.iter() {
            if point.x > max.x { max.x = point.x }
            if point.x < min.x { min.x = point.x }

            if point.y > max.y { max.y = point.y }
            if point.y < min.y { min.y = point.y }
        }

        return Rectangle {
            left: min.x,
            right: max.x,
            top: max.y,
            bottom: min.y,
        }
    }
}

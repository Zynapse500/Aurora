use Vector2;
use Rectangle;

pub trait Collide: AsCollide {
    /// Return the point the furthest along an axis
    fn get_farthest_point(&self, axis: Vector2) -> Vector2;


    /// Return a bounding box
    fn get_bounding_box(&self) -> Rectangle;
}


pub trait AsCollide {
    fn as_collide(&self) -> &Collide;
}

impl<T: Collide> AsCollide for T {
    fn as_collide(&self) -> &Collide {
        self
    }
}



/// Returns true if two convex objects are intersecting
pub fn intersect(a: &Collide, b: &Collide) -> bool {
    gjk_simplex(a, b).is_some()
}


/// Returns the minimum translation vector to move two objects out of an overlap
pub fn overlap(a: &Collide, b: &Collide) -> Option<Vector2> {
    const TOLERANCE: f64 = 1e-7;

    if let Some(mut simplex) = gjk_simplex(a, b).take() {
//        println!("EPA: {:?}", simplex);

        if simplex[0].x.is_nan() {
            return Some(Vector2::new(0.0, 0.0));
        }

        loop {
            use std::f64::INFINITY;
            // Find the edge closest to the origin
            let mut normal = Vector2::new(0.0, 0.0);
            let mut min_distance = INFINITY;
            let mut index = 0;

            for i in 0..simplex.len() {
                let a = simplex[i];
                let b = simplex[(i + 1) % simplex.len()];

                let n = -toward_origin(a, b).normal();
                let distance = n.dot(a);
                if distance < min_distance {
                    normal = n;
                    min_distance = distance;
                    index = i;
                }
            }


            let p = support(a, b, normal);
            let delta = min_distance - normal.dot(p);

//            println!("Delta: {}", delta);

            if delta.abs() < TOLERANCE {
                return Some(normal * min_distance);
            } else {
                simplex.insert(index + 1, p);
            }
        }
    }

    None
}


/// Returns the final simplex between two convex objects, if there is one
fn gjk_simplex(a: &Collide, b: &Collide) -> Option<Vec<Vector2>> {
    let mut simplex = Vec::new();
    let mut direction = Vector2{ x: 1.0, y: 0.0 };

    simplex.push(support(a, b, direction));

    direction = -direction;

    loop {
        let last = support(a, b, direction);
        simplex.push(last);

        if direction.dot(last) <= 0.0 {
            return None;
        } else {
            // Determine if the simplex contains the origin
            if {
                let a = simplex[0];
                let b = simplex[1];

                if simplex.len() == 3 {
                    let a_perp = toward_origin(a, last);
                    let b_perp = toward_origin(b, last);

                    if a_perp.dot(b) < 0.0 {
                        simplex.remove(1);
                        direction = a_perp;
                        false
                    } else if b_perp.dot(a) < 0.0 {
                        simplex.remove(0);
                        direction = b_perp;
                        false
                    } else {
                        true
                    }
                } else {
                    direction = toward_origin(a, b);
                    false
                }
            } {
                return Some(simplex);
            }
        }
    }
}


/// Return the point farthest along an axis on the minkowski difference
fn support(a: &Collide, b: &Collide, direction: Vector2) -> Vector2 {
    let p1 = a.get_farthest_point(direction);
    let p2 = b.get_farthest_point(-direction);

    p1 - p2
}


/// Return a direction vector perpendicular to a line toward the origin
fn toward_origin(a: Vector2, b: Vector2) -> Vector2 {
    let direction = Vector2::new(b.y - a.y, a.x - b.x);

    if direction.dot(b) > 0.0 {
        -direction
    } else {
        direction
    }
}
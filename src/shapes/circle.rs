use trap::Vector2;


#[derive(Copy, Clone)]
pub struct Circle {
    pub center: Vector2,
    pub radius: f64
}

impl Circle {
    pub fn new(center: Vector2, radius: f64) -> Circle {
        Circle { center, radius }
    }


    pub fn translate(&mut self, amount: Vector2) {
        self.center += amount;
    }
}

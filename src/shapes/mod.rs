

mod rectangle;
mod circle;
mod convex_hull;

pub use self::rectangle::Rectangle;
pub use self::circle::Circle;
pub use self::convex_hull::ConvexHull;


use Vector2;
use renderer::Triangulate;
use collision::Collide;

pub trait Shape: Triangulate + Collide {
    /// Translates the shape
    fn translate(&mut self, amount: Vector2);

    /// Returns the center of the shape
    fn center(&self) -> Vector2;
}


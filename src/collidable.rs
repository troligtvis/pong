use crate::na;

pub trait Collidable {
    fn get_position(&self) -> na::Point2<f32>;
    fn get_size(&self) -> (f32, f32);

    fn check_collision(&self, other: &dyn Collidable) -> bool {
        let r1x = self.get_position().x;
        let r1y = self.get_position().y;
        let (r1w, r1h) = self.get_size();

        let r2x = other.get_position().x;
        let r2y = other.get_position().y;
        let (r2w, r2h) = self.get_size();

        let result = r1x + r1w >= r2x && r1x <= r2x + r2w && r1y + r1h >= r2y && r1y <= r2y + r2h;

        result
    }
}

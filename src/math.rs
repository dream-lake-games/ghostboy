use crate::prelude::*;

impl Pos {
    pub fn to_ivec(&self) -> IVec2 {
        IVec2::new(self.x.round() as i32, self.y.round() as i32)
    }
}
impl std::ops::Add<Vec2> for Pos {
    type Output = Self;

    fn add(mut self, rhs: Vec2) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

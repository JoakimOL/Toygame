pub trait Collidable {
    fn check_collide(&self, other: &Collidable) -> bool;
}

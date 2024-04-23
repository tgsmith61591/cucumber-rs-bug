#[derive(Clone, Copy, cucumber::World, Debug, Default)]
pub struct World {
    pub sleeper: u64,
    pub refreshed: bool,
}

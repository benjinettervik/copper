#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderLayer {
    Background,
    Terrain,
    Objects,
    Sprite,
    Effects
}
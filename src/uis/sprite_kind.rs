#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpriteKind {
    Gravel,
    Road,
    Lava,
    Water,
    Empty,
    WallBrown,
}

impl SpriteKind {
    pub fn get_index(&self) -> usize {
        match self {
            SpriteKind::Gravel => 0,
            SpriteKind::Road => 1,
            SpriteKind::Lava => 2,
            SpriteKind::Water => 3,
            SpriteKind::Empty => 4,
            SpriteKind::WallBrown => 5,
        }
    } 
}

struct Wall {
    kind: WallKind,
}

impl Wall {
    // Constants
	// Constructors
    pub fn from_kind(kind: WallKind) -> Self {
        Self { kind }
    }
	// Public functions
	// Private functions
}

#[derive(Copy, Clone)]
pub enum WallKind {
    Dirt,
    Rock,
}
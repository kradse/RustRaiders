struct Floor {
    kind: FloorKind,
}

impl Floor {
    // Constants
	// Constructors
    pub fn from_kind(kind: FloorKind) -> Self {
        Self { kind }
    }
	// Public functions
	// Private functions
}

#[derive(Copy, Clone)]
pub enum FloorKind {
    Dirt,
    Gravel,
}
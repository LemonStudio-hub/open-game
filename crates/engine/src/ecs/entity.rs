use generational_arena::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub(crate) index: Index,
}

impl Entity {
    pub(crate) fn new(index: Index) -> Self {
        Self { index }
    }

    pub fn id(&self) -> usize {
        self.index.into_raw_parts().0
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({})", self.id())
    }
}

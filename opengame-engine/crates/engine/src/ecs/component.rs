use std::any::Any;

pub trait Component: Any + 'static {}
impl<T: Any + 'static> Component for T {}

pub(crate) trait ComponentStorage: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, index: generational_arena::Index);
}

pub(crate) struct TypedStorage<T: 'static> {
    pub data: std::collections::HashMap<generational_arena::Index, T>,
}

impl<T: 'static> TypedStorage<T> {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, index: generational_arena::Index, component: T) {
        self.data.insert(index, component);
    }

    pub fn get(&self, index: generational_arena::Index) -> Option<&T> {
        self.data.get(&index)
    }

    pub fn get_mut(&mut self, index: generational_arena::Index) -> Option<&mut T> {
        self.data.get_mut(&index)
    }

    pub fn remove_entry(&mut self, index: generational_arena::Index) -> Option<T> {
        self.data.remove(&index)
    }
}

impl<T: 'static> ComponentStorage for TypedStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, index: generational_arena::Index) {
        self.data.remove(&index);
    }
}

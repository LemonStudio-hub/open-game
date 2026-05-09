use super::entity::Entity;
use super::world::World;
use super::component::TypedStorage;
use generational_arena::Index;

pub struct QuerySingle<'a, T: 'static> {
    storage: &'a TypedStorage<T>,
}

impl<'a, T: 'static> QuerySingle<'a, T> {
    pub fn new(world: &'a World) -> Option<Self> {
        let storage = world.get_storage::<T>()?;
        Some(Self { storage })
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.storage.get(entity.index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.storage.data.iter().map(|(idx, comp)| (Entity::new(*idx), comp))
    }

    pub fn len(&self) -> usize {
        self.storage.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.data.is_empty()
    }
}

pub struct QuerySingleMut<'a, T: 'static> {
    storage: &'a mut TypedStorage<T>,
}

impl<'a, T: 'static> QuerySingleMut<'a, T> {
    pub fn new(world: &'a mut World) -> Option<Self> {
        let storage = world.get_storage_mut::<T>()?;
        Some(Self { storage })
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.storage.get(entity.index)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.storage.get_mut(entity.index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.storage.data.iter().map(|(idx, comp)| (Entity::new(*idx), comp))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.storage.data.iter_mut().map(|(idx, comp)| (Entity::new(*idx), comp))
    }

    pub fn len(&self) -> usize {
        self.storage.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.data.is_empty()
    }
}

pub struct QueryDouble<'a, A: 'static, B: 'static> {
    storage_a: &'a TypedStorage<A>,
    storage_b: &'a TypedStorage<B>,
}

impl<'a, A: 'static, B: 'static> QueryDouble<'a, A, B> {
    pub fn new(world: &'a World) -> Option<Self> {
        let storage_a = world.get_storage::<A>()?;
        let storage_b = world.get_storage::<B>()?;
        Some(Self { storage_a, storage_b })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &A, &B)> {
        self.storage_a.data.iter().filter_map(|(idx, a)| {
            let b = self.storage_b.get(*idx)?;
            Some((Entity::new(*idx), a, b))
        })
    }
}

pub struct QueryDoubleMut<'a, A: 'static, B: 'static> {
    storage_a: *mut TypedStorage<A>,
    storage_b: *mut TypedStorage<B>,
    _marker: std::marker::PhantomData<&'a (A, B)>,
}

impl<'a, A: 'static, B: 'static> QueryDoubleMut<'a, A, B> {
    pub fn new(world: &'a mut World) -> Option<Self> {
        if std::any::TypeId::of::<A>() == std::any::TypeId::of::<B>() {
            return None;
        }

        let storage_a = world.get_storage_mut::<A>()? as *mut TypedStorage<A>;
        let storage_b = world.get_storage_mut::<B>()? as *mut TypedStorage<B>;

        Some(Self {
            storage_a,
            storage_b,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn iter(&mut self) -> Vec<(Entity, &mut A, &mut B)> {
        let storage_a = unsafe { &mut *self.storage_a };
        let storage_b = unsafe { &mut *self.storage_b };

        let mut ptrs: Vec<(Index, *mut A, *mut B)> = Vec::new();
        for (idx, a) in storage_a.data.iter_mut() {
            if let Some(b) = storage_b.data.get_mut(idx) {
                ptrs.push((*idx, a as *mut A, b as *mut B));
            }
        }

        ptrs.into_iter().map(|(idx, a_ptr, b_ptr)| {
            (Entity::new(idx), unsafe { &mut *a_ptr }, unsafe { &mut *b_ptr })
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_single() {
        let mut world = World::new();
        let e1 = world.spawn_empty();
        let e2 = world.spawn_empty();
        world.insert_component(e1, 10_i32);
        world.insert_component(e2, 20_i32);

        let query = QuerySingle::<i32>::new(&world).unwrap();
        assert_eq!(query.len(), 2);
        assert!(!query.is_empty());
        assert_eq!(query.get(e1), Some(&10));
        assert_eq!(query.get(e2), Some(&20));
    }

    #[test]
    fn test_query_single_iter() {
        let mut world = World::new();
        let e1 = world.spawn_empty();
        let e2 = world.spawn_empty();
        world.insert_component(e1, 10_i32);
        world.insert_component(e2, 20_i32);

        let query = QuerySingle::<i32>::new(&world).unwrap();
        let mut sum = 0;
        for (_entity, val) in query.iter() {
            sum += *val;
        }
        assert_eq!(sum, 30);
    }

    #[test]
    fn test_query_single_empty() {
        let world = World::new();
        let query = QuerySingle::<i32>::new(&world);
        assert!(query.is_none());
    }

    #[test]
    fn test_query_single_mut() {
        let mut world = World::new();
        let e = world.spawn_empty();
        world.insert_component(e, 10_i32);

        let mut query = QuerySingleMut::<i32>::new(&mut world).unwrap();
        if let Some(val) = query.get_mut(e) {
            *val = 99;
        }

        drop(query);
        assert_eq!(world.get_component::<i32>(e), Some(&99));
    }

    #[test]
    fn test_query_single_mut_iter() {
        let mut world = World::new();
        let e1 = world.spawn_empty();
        let e2 = world.spawn_empty();
        world.insert_component(e1, 1_i32);
        world.insert_component(e2, 2_i32);

        let mut query = QuerySingleMut::<i32>::new(&mut world).unwrap();
        for (_entity, val) in query.iter_mut() {
            *val *= 10;
        }

        drop(query);
        assert_eq!(world.get_component::<i32>(e1), Some(&10));
        assert_eq!(world.get_component::<i32>(e2), Some(&20));
    }

    #[test]
    fn test_query_double() {
        let mut world = World::new();
        let e1 = world.spawn_empty();
        let e2 = world.spawn_empty();
        world.insert_component(e1, 1_i32);
        world.insert_component(e1, 10.0_f64);
        world.insert_component(e2, 2_i32);

        let query = QueryDouble::<i32, f64>::new(&world).unwrap();
        let results: Vec<_> = query.iter().collect();
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0].1, 1);
        assert_eq!(*results[0].2, 10.0);
    }

    #[test]
    fn test_query_double_same_type_returns_none() {
        let mut world = World::new();
        let _e = world.spawn_empty();
        let query = QueryDoubleMut::<i32, i32>::new(&mut world);
        assert!(query.is_none());
    }

    #[test]
    fn test_query_double_mut() {
        let mut world = World::new();
        let e = world.spawn_empty();
        world.insert_component(e, 1_i32);
        world.insert_component(e, 10.0_f64);

        let mut query = QueryDoubleMut::<i32, f64>::new(&mut world).unwrap();
        let results = query.iter();
        assert_eq!(results.len(), 1);

        drop(query);
        assert_eq!(world.get_component::<i32>(e), Some(&1));
        assert_eq!(world.get_component::<f64>(e), Some(&10.0));
    }
}

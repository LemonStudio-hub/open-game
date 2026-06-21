use super::builder::EntityBuilder;
use super::component::{ComponentStorage, TypedStorage};
use super::entity::Entity;
use generational_arena::Arena;
use std::any::{Any, TypeId};
use std::collections::HashMap;

struct EntityData {
    alive: bool,
}

pub struct World {
    entities: Arena<EntityData>,
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Arena::new(),
            storages: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    pub fn spawn_empty(&mut self) -> Entity {
        let index = self.entities.insert(EntityData { alive: true });
        Entity::new(index)
    }

    pub fn spawn(&mut self) -> EntityBuilder<'_> {
        let entity = self.spawn_empty();
        EntityBuilder::new(self, entity)
    }

    pub fn spawn_components(&mut self) -> EntityBuilder<'_> {
        self.spawn()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if let Some(data) = self.entities.get_mut(entity.index) {
            data.alive = false;
            for storage in self.storages.values_mut() {
                storage.remove(entity.index);
            }
            self.entities.remove(entity.index);
            true
        } else {
            false
        }
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.get(entity.index).is_some_and(|d| d.alive)
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn insert_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let storage = self
            .storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(TypedStorage::<T>::new()));

        storage
            .as_any_mut()
            .downcast_mut::<TypedStorage<T>>()
            .unwrap()
            .insert(entity.index, component);
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) -> Option<T> {
        let storage = self.storages.get_mut(&TypeId::of::<T>())?;
        storage
            .as_any_mut()
            .downcast_mut::<TypedStorage<T>>()?
            .remove_entry(entity.index)
    }

    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        if let Some(storage) = self.storages.get(&TypeId::of::<T>()) {
            storage
                .as_any()
                .downcast_ref::<TypedStorage<T>>()
                .is_some_and(|s| s.get(entity.index).is_some())
        } else {
            false
        }
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let storage = self.storages.get(&TypeId::of::<T>())?;
        storage
            .as_any()
            .downcast_ref::<TypedStorage<T>>()?
            .get(entity.index)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let storage = self.storages.get_mut(&TypeId::of::<T>())?;
        storage
            .as_any_mut()
            .downcast_mut::<TypedStorage<T>>()?
            .get_mut(entity.index)
    }

    pub(crate) fn get_storage<T: 'static>(&self) -> Option<&TypedStorage<T>> {
        let storage = self.storages.get(&TypeId::of::<T>())?;
        storage.as_any().downcast_ref::<TypedStorage<T>>()
    }

    pub(crate) fn get_storage_mut<T: 'static>(&mut self) -> Option<&mut TypedStorage<T>> {
        let storage = self.storages.get_mut(&TypeId::of::<T>())?;
        storage.as_any_mut().downcast_mut::<TypedStorage<T>>()
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.resources
            .remove(&TypeId::of::<T>())?
            .downcast::<T>()
            .ok()
            .map(|boxed| *boxed)
    }

    pub fn has_resource<T: 'static>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    pub fn entities(&self) -> Vec<Entity> {
        self.entities
            .iter()
            .map(|(idx, _)| Entity::new(idx))
            .collect()
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.storages.clear();
        self.resources.clear();
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_despawn() {
        let mut world = World::new();
        let e = world.spawn_empty();
        assert!(world.is_alive(e));
        assert_eq!(world.entity_count(), 1);

        assert!(world.despawn(e));
        assert!(!world.is_alive(e));
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn test_spawn_multiple() {
        let mut world = World::new();
        let e1 = world.spawn_empty();
        let e2 = world.spawn_empty();
        let e3 = world.spawn_empty();
        assert_eq!(world.entity_count(), 3);
        assert_ne!(e1, e2);
        assert_ne!(e2, e3);
    }

    #[test]
    fn test_insert_and_get_component() {
        let mut world = World::new();
        let e = world.spawn_empty();

        world.insert_component(e, 42_i32);
        assert_eq!(world.get_component::<i32>(e), Some(&42));
    }

    #[test]
    fn test_get_component_mut() {
        let mut world = World::new();
        let e = world.spawn_empty();

        world.insert_component(e, 10_i32);
        if let Some(val) = world.get_component_mut::<i32>(e) {
            *val = 20;
        }
        assert_eq!(world.get_component::<i32>(e), Some(&20));
    }

    #[test]
    fn test_remove_component() {
        let mut world = World::new();
        let e = world.spawn_empty();

        world.insert_component(e, 42_i32);
        let removed = world.remove_component::<i32>(e);
        assert_eq!(removed, Some(42));
        assert_eq!(world.get_component::<i32>(e), None);
    }

    #[test]
    fn test_has_component() {
        let mut world = World::new();
        let e = world.spawn_empty();

        assert!(!world.has_component::<i32>(e));
        world.insert_component(e, 42_i32);
        assert!(world.has_component::<i32>(e));
    }

    #[test]
    fn test_multiple_component_types() {
        let mut world = World::new();
        let e = world.spawn_empty();

        world.insert_component(e, 42_i32);
        world.insert_component(e, "hello".to_string());

        assert_eq!(world.get_component::<i32>(e), Some(&42));
        assert_eq!(world.get_component::<String>(e), Some(&"hello".to_string()));
    }

    #[test]
    fn test_component_on_different_entities() {
        let mut world = World::new();
        let e1 = world.spawn_empty();
        let e2 = world.spawn_empty();

        world.insert_component(e1, 1_i32);
        world.insert_component(e2, 2_i32);

        assert_eq!(world.get_component::<i32>(e1), Some(&1));
        assert_eq!(world.get_component::<i32>(e2), Some(&2));
    }

    #[test]
    fn test_resource_insert_and_get() {
        let mut world = World::new();
        world.insert_resource(100_u64);

        assert!(world.has_resource::<u64>());
        assert_eq!(world.get_resource::<u64>(), Some(&100));
    }

    #[test]
    fn test_resource_get_mut() {
        let mut world = World::new();
        world.insert_resource(100_u64);

        if let Some(val) = world.get_resource_mut::<u64>() {
            *val = 200;
        }
        assert_eq!(world.get_resource::<u64>(), Some(&200));
    }

    #[test]
    fn test_resource_remove() {
        let mut world = World::new();
        world.insert_resource(42_i32);

        let removed = world.remove_resource::<i32>();
        assert_eq!(removed, Some(42));
        assert!(!world.has_resource::<i32>());
    }

    #[test]
    fn test_entities_list() {
        let mut world = World::new();
        let _e1 = world.spawn_empty();
        let _e2 = world.spawn_empty();
        let entities = world.entities();
        assert_eq!(entities.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut world = World::new();
        let e = world.spawn_empty();
        world.insert_component(e, 42_i32);
        world.insert_resource(100_u64);

        world.clear();
        assert_eq!(world.entity_count(), 0);
        assert!(!world.has_resource::<i32>());
    }

    #[test]
    fn test_despawn_removes_components() {
        let mut world = World::new();
        let e = world.spawn_empty();
        world.insert_component(e, 42_i32);

        world.despawn(e);
        assert_eq!(world.get_component::<i32>(e), None);
    }

    #[test]
    fn test_spawn_builder() {
        let mut world = World::new();
        let e = world
            .spawn()
            .with(100_i32)
            .with(std::f64::consts::PI)
            .build();

        assert_eq!(world.get_component::<i32>(e), Some(&100));
        assert_eq!(world.get_component::<f64>(e), Some(&std::f64::consts::PI));
    }
}

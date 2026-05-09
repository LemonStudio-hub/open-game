use super::entity::Entity;
use super::world::World;

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: Entity,
}

impl<'a> EntityBuilder<'a> {
    pub(crate) fn new(world: &'a mut World, entity: Entity) -> Self {
        Self { world, entity }
    }

    pub fn with<T: 'static>(self, component: T) -> Self {
        self.world.insert_component(self.entity, component);
        self
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}

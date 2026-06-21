use crate::ecs::entity::Entity;
use crate::math::Vec2;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CellCoord(i32, i32);

pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<CellCoord, Vec<Entity>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        // Retain HashMap allocations, just clear each cell's Vec
        for entities in self.cells.values_mut() {
            entities.clear();
        }
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2, half_extent: Vec2) {
        let min_x = ((position.x - half_extent.x) / self.cell_size).floor() as i32;
        let max_x = ((position.x + half_extent.x) / self.cell_size).floor() as i32;
        let min_y = ((position.y - half_extent.y) / self.cell_size).floor() as i32;
        let max_y = ((position.y + half_extent.y) / self.cell_size).floor() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                self.cells.entry(CellCoord(x, y)).or_default().push(entity);
            }
        }
    }

    pub fn query(&self, position: Vec2, half_extent: Vec2) -> Vec<Entity> {
        let min_x = ((position.x - half_extent.x) / self.cell_size).floor() as i32;
        let max_x = ((position.x + half_extent.x) / self.cell_size).floor() as i32;
        let min_y = ((position.y - half_extent.y) / self.cell_size).floor() as i32;
        let max_y = ((position.y + half_extent.y) / self.cell_size).floor() as i32;

        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(entities) = self.cells.get(&CellCoord(x, y)) {
                    for &entity in entities {
                        if seen.insert(entity) {
                            result.push(entity);
                        }
                    }
                }
            }
        }
        result
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self::new(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use generational_arena::Index;

    fn make_entity(id: usize) -> Entity {
        Entity::new(Index::from_raw_parts(id, 0))
    }

    #[test]
    fn test_new_grid() {
        let grid = SpatialGrid::new(50.0);
        assert_eq!(grid.cell_size(), 50.0);
    }

    #[test]
    fn test_default_grid() {
        let grid = SpatialGrid::default();
        assert_eq!(grid.cell_size(), 100.0);
    }

    #[test]
    fn test_insert_and_query() {
        let mut grid = SpatialGrid::new(100.0);
        let e = make_entity(1);
        grid.insert(e, Vec2::new(50.0, 50.0), Vec2::new(10.0, 10.0));

        let results = grid.query(Vec2::new(50.0, 50.0), Vec2::new(50.0, 50.0));
        assert!(results.contains(&e));
    }

    #[test]
    fn test_query_empty() {
        let grid = SpatialGrid::new(100.0);
        let results = grid.query(Vec2::ZERO, Vec2::new(100.0, 100.0));
        assert!(results.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut grid = SpatialGrid::new(100.0);
        let e = make_entity(1);
        grid.insert(e, Vec2::ZERO, Vec2::new(10.0, 10.0));
        grid.clear();

        let results = grid.query(Vec2::ZERO, Vec2::new(100.0, 100.0));
        assert!(results.is_empty());
    }

    #[test]
    fn test_multiple_entities_same_cell() {
        let mut grid = SpatialGrid::new(100.0);
        let e1 = make_entity(1);
        let e2 = make_entity(2);
        grid.insert(e1, Vec2::new(10.0, 10.0), Vec2::new(5.0, 5.0));
        grid.insert(e2, Vec2::new(20.0, 20.0), Vec2::new(5.0, 5.0));

        let results = grid.query(Vec2::new(15.0, 15.0), Vec2::new(50.0, 50.0));
        assert!(results.contains(&e1));
        assert!(results.contains(&e2));
    }

    #[test]
    fn test_entities_in_different_cells() {
        let mut grid = SpatialGrid::new(100.0);
        let e1 = make_entity(1);
        let e2 = make_entity(2);
        grid.insert(e1, Vec2::new(10.0, 10.0), Vec2::new(5.0, 5.0));
        grid.insert(e2, Vec2::new(1000.0, 1000.0), Vec2::new(5.0, 5.0));

        let results = grid.query(Vec2::new(10.0, 10.0), Vec2::new(15.0, 15.0));
        assert!(results.contains(&e1));
        assert!(!results.contains(&e2));
    }

    #[test]
    fn test_no_duplicate_entities() {
        let mut grid = SpatialGrid::new(100.0);
        let e = make_entity(1);
        grid.insert(e, Vec2::ZERO, Vec2::new(200.0, 200.0));

        let results = grid.query(Vec2::ZERO, Vec2::new(200.0, 200.0));
        let count = results.iter().filter(|&&ent| ent == e).count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_clear_retains_capacity() {
        let mut grid = SpatialGrid::new(100.0);
        let e = make_entity(1);
        grid.insert(e, Vec2::ZERO, Vec2::new(10.0, 10.0));
        let cell_count_before = grid.cells.len();
        grid.clear();
        // Cells should still exist (just empty), so HashMap is not deallocated
        assert_eq!(grid.cells.len(), cell_count_before);
    }
}

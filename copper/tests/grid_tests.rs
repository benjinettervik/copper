#[cfg(test)]
mod grid_tests{
    // import test components
    use::copper::*;
    use copper::engine::Engine;
    use copper::engine::world::*;
    use copper::grid::{Grid,GridPosition};
    // use crate::copper::engine::*;
    // use crate::grid::*;    

    #[test]
    pub fn make_a_grid_insert_entity(){
        // basics for engine
        let mut engine = Engine::new();
        let mut world = World::new();
        let ent_id = world.spawn();
        // grid 
        let mut grid = Grid::new(32,32,16.0);
        // ent id in grid 
        grid.insert(ent_id, 10.0 ,10.0);
        // hur skulle man querya detta? 
        // ett förslag
        let grid_pos = GridPosition{x:10.0,y:10.0};
    }
    #[test]
    pub fn insert_sprites_with_different_textures_on_grid(){
        
    }
}
The grid works in 3 steps each tick
1. first clear the grid
"grid.clear()"
2. second insert all entities into the grid, something similar to:
for(entity) in entities.iter().enumerate(){
    grid.insert(entity.id, entity.x, entity.y);
}
3. third, query, entities that needs to use pathfinding will make use of the grid, compare mouse click coordinates with objects in the same grid pos, other actions related to input that need to be translated 
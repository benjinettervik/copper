use copper::grid::{Grid, GridPosition};

#[test]
fn grid_position_new() {
    let position = GridPosition::new(3, 2);

    assert_eq!(position, GridPosition { x: 3, y: 2 });
}

#[test]
fn contains_position_inside() {
    let grid = Grid::new(10, 10, 16.0);

    assert!(grid.contains_position(GridPosition::new(3, 2)));
}

#[test]
fn contains_position_outside() {
    let grid = Grid::new(10, 10, 16.0);

    assert!(!grid.contains_position(GridPosition::new(10, 2)));
}

#[test]
fn insert_grid_entity() {
    let mut grid = Grid::new(10, 10, 16.0);

    let inserted = grid.insert_grid(7, GridPosition::new(3, 2));

    assert!(inserted);
    assert_eq!(grid.query_grid(GridPosition::new(3, 2)), &[7]);
}

#[test]
fn insert_grid_outside() {
    let mut grid = Grid::new(10, 10, 16.0);

    let inserted = grid.insert_grid(7, GridPosition::new(10, 2));

    assert!(!inserted);
}

#[test]
fn world_insert_query() {
    let mut grid = Grid::new(10, 10, 16.0);

    grid.insert(7, 50.0, 35.0);

    assert_eq!(grid.query_grid(GridPosition::new(3, 2)), &[7]);
    assert_eq!(grid.query(50.0, 35.0), &[7]);
}

#[test]
fn clear_grid() {
    let mut grid = Grid::new(10, 10, 16.0);

    grid.insert_grid(7, GridPosition::new(3, 2));
    grid.clear();

    assert!(grid.query_grid(GridPosition::new(3, 2)).is_empty());
}

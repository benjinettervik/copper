use crate::Component;

pub struct Grid{
    cells:Vec<Vec<Vec<usize>>>, // [Rows][Cols][obj]
    width:usize,
    height:usize,
    cell_size:f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridPosition{
    pub x:usize,
    pub y:usize,
}

impl GridPosition {
    pub fn new(x:usize, y:usize) -> Self {
        Self { x, y }
    }
}

impl Component for GridPosition {
    fn name(&self) -> &str {
        "GridPosition"
    }
}

impl Grid{
    pub fn new(width:usize, height:usize, cell_size:f32) -> Self{
        let cells = (0..height)
            .map(|_| (0..width).map(|_| Vec::new()).collect())
            .collect();
        Self{
            cells,
            width,
            height,
            cell_size,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }


    //wipes all cells, once per frame
    pub fn clear(&mut self){
        for row  in &mut self.cells{
            for cell in row{
                cell.clear();
            }
        }
    }

    //coordinate math
    pub fn world_to_grid(&self, x:f32, y:f32) -> Option<(usize,usize)> {
        let gx = (x/self.cell_size) as i32;
        let gy = (y/self.cell_size) as i32;
        if gx>= 0 && gy>= 0 &&(gx as usize) < self.width && (gy as usize) < self.height {
            Some((gx as usize,gy as usize))
        }
        else{
            None //not in grid
        }
    }

    pub fn grid_to_world(&self, gx:usize, gy:usize) -> Option<(f32,f32)> {
        if gx < self.width && gy < self.height {
            Some((gx as f32 * self.cell_size, gy as f32 * self.cell_size))
        } else {
            None
        }
    }

    pub fn contains_position(&self, position: GridPosition) -> bool {
        position.x < self.width && position.y < self.height
    }

    pub fn insert_grid(&mut self, entity_id:usize, position: GridPosition) -> bool {
        if self.contains_position(position) {
            self.cells[position.y][position.x].push(entity_id);
            true
        } else {
            false
        }
    }

    pub fn query_grid(&self, position: GridPosition) -> &[usize] {
        if self.contains_position(position) {
            &self.cells[position.y][position.x]
        } else {
            &[]
        }
    }

    //mutation
    pub fn insert(&mut self, entity_id:usize, x:f32, y:f32) -> bool{
        if let Some((gx,gy)) = self.world_to_grid(x,y) {
            self.insert_grid(entity_id, GridPosition::new(gx, gy))
        } else {
            false
        }
    }

    pub fn query(&self, x:f32, y:f32) -> &[usize]{
        if let Some((gx,gy)) = self.world_to_grid(x,y) {
            self.query_grid(GridPosition::new(gx, gy))
        } else {
            &[]
        }
    }

    //just an eg of what you might want to implement
    pub fn query_adjacent(&self, x:f32, y:f32, radius: usize) -> Vec<usize>{
        if let Some((gx,gy)) = self.world_to_grid(x, y){
        let x_min = gx.saturating_sub(radius);
        let y_min = gy.saturating_sub(radius);
        let x_max = (gx + radius).min(self.width - 1);
        let y_max = (gy + radius).min(self.height - 1);

        let mut results = Vec::new();
        for row in &self.cells[y_min..=y_max] {
            for cell in &row[x_min..=x_max] {
                results.extend_from_slice(cell);
            }
        }
        results
        } else {
            Vec::new()
        }
    }

    //TODO
    //pathfinding{
    //}

    //TODO
    //collision_detection{
    //}
}




//eg use of query
//if mouse_clicked{
//    let hits = grid.query(mouse_x, mouse_y);
//    for id in hits{
//        //might need to compare entity pos with mouse_x, mouse_y; make sure its not only in the same grid but acctually was clicked if its size is smaller than the grid
//        //handle clicks
//    }
//}

use crate::assets::texture_map::TM_Handle;
use crate::grid::{Grid, GridPosition};
use crate::renderer::render_grid::RenderGrid;
use crate::renderer::render_layer::RenderLayer;
use crate::renderer::render_map::RenderMap;
use serde_json::Value;
use std::fs;

pub struct JSON_Meta {
    pub height: u32,
    pub width: u32,
    pub layers: Vec<Vec<u32>>,
}

///Extracting layer data using serde_json dependency
pub fn extract_layer_data2(path: &str) -> Result<JSON_Meta, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let layers = json["layers"].as_array().ok_or("Missing layers array")?;

    let mut result = Vec::new();
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    // let mut result = JSON_Meta{};
    for layer in layers {
        let data = layer["data"].as_array().ok_or("Missing data array")?;

        width = layer["width"].as_u64().unwrap() as u32;
        height = layer["height"].as_u64().unwrap() as u32;

        println!("Found data array with {} entries", data.len());
        let numbers: Vec<u32> = data
            .iter()
            .filter_map(|v| v.as_u64())
            .map(|n| n as u32)
            .collect();

        result.push(numbers);
    }
    Ok(JSON_Meta {
        height: height,
        width: width,
        layers: result,
    })
}

pub fn json_to_rendermap(path: &str, pix_dim: f32, handle: TM_Handle) -> Result<RenderMap, String> {
    let json_read = extract_layer_data2(path).unwrap();
    let layer_amount = json_read.layers.len();
    // let result = RenderMap::new();

    // basic implemntation do not allow for more than 3 layers
    if layer_amount > 3 {
        return Err("Too many layers in map".to_string());
    }
    let mut render_grids = Vec::<RenderGrid>::new();

    //
    let layers = json_read.layers;
    let width = json_read.width;
    let height = json_read.height;

    for i in 0..layer_amount {
        let layer = &layers[i];
        let mut grid: Grid = Grid::new(width as usize, height as usize, pix_dim);
        match i {
            0 => {
                println!("Layer 1");
                let mut count = 0;
                for y in 0..height {
                    for x in 0..width {
                        let tile = layer[count];

                        if tile != 0 {
                            grid.insert_grid(
                                (tile - 1) as usize,
                                GridPosition {
                                    x: x as usize,
                                    y: y as usize,
                                },
                            );
                        }
                        count += 1;
                    }
                }
                render_grids.push(RenderGrid {
                    layer: RenderLayer::Background,
                    grid: grid,
                    handle: handle.clone(),
                    tile_size: pix_dim,
                })
            }

            1 => {
                println!("Layer 2");
                let mut count = 0;
                for y in 0..height {
                    for x in 0..width {
                        let tile = layer[count];

                        if tile != 0 {
                            grid.insert_grid(
                                (tile - 1) as usize,
                                GridPosition {
                                    x: x as usize,
                                    y: y as usize,
                                },
                            );
                        }
                        count += 1;
                    }
                }
                render_grids.push(RenderGrid {
                    layer: RenderLayer::Terrain,
                    grid: grid,
                    handle: handle.clone(),
                    tile_size: pix_dim,
                })
            }

            2 => {
                println!("Layer 2");
                let mut count = 0;
                for y in 0..height {
                    for x in 0..width {
                        let tile = layer[count];

                        if tile != 0 {
                            grid.insert_grid(
                                (tile - 1) as usize,
                                GridPosition {
                                    x: x as usize,
                                    y: y as usize,
                                },
                            );
                        }
                        count += 1;
                    }
                }
                render_grids.push(RenderGrid {
                    layer: RenderLayer::Objects,
                    grid: grid,
                    handle: handle.clone(),
                    tile_size: pix_dim,
                })
            }

            _ => {}
        }
    }

    // println!("Making a rendermap");

    Ok(RenderMap {
        grids: render_grids,
    })
}

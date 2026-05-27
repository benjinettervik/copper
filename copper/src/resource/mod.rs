// use copper::renderer::test_components_renderer::*;
pub mod camera;
use crate::renderer::test_components_renderer::*;
use crate::grid::GridPosition;
use crate::resource::camera::*;
use crate::renderer::render_sys::*;
use crate::renderer::render_sys::NewRenderSys;
use crate::input::Input;
use std::collections::HashMap;
use std::any::Any;
use std::any::TypeId;
use crate::grid::Grid;

// 
use std::fs;
use serde_json::Value;


// pub struct Resources {
//     pub texture_hash: TextureAsset,
//     pub render_queue: RenderQueue,
//     pub Camera2D: Camera2D,
//     pub input: Input,
// }

pub struct Resources{
    // key becomes the type of the resource that has been saved , and the box contains on 
    // heap the accessible data associated.
    // Box<T> is a smart pointer -- stores variable on heap instead of directly on stack 
    // dyn Any , unknown size at compile time 
    resources: HashMap<TypeId, Box<dyn Any>>,

}

impl Resources{
    pub fn new() -> Self {

        let mut resources = Self {
            resources: HashMap::new(),
        };

        // basic tools for resources, that can be ignored if user want to
        // can also become a system
        resources.init_basic_kit();
        resources
    }

    pub fn insert<T:Any>(&mut self, value: T){
        // typeid
        // type fingerprint
        self.resources.insert(TypeId::of::<T>(),Box::new(value));
    }

    pub fn get<T: Any>(&self) -> Option<&T>
    {
        self.resources
            .get(&TypeId::of::<T>()) //Hashmap get with key -> typeid of T -- downcast ref to check if the value inside the dyn Any is actually type T
            .and_then(|v| v.downcast_ref::<T>())
    }

    // same but mutable
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T>
    {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }

    pub fn init_basic_kit(&mut self){
        self.insert(RenderQueue{commands: Vec::new(),is_grid:None,t_map:None});
        self.insert(TextureAsset{textures: HashMap::new(),});
        self.insert(Camera2D::new());
        self.insert(Input::new());
        self.insert(RenderMap::new());
        self.insert(TextureMap::new());
        // self.resources.insert(Grid::new(32,32,16.0));
    }
}







#[derive(Debug,Clone,Ord,Eq,PartialEq,PartialOrd)]
pub enum RenderLayer {
    Background,
    Terrain,
    Objects,
    Sprite,
    Effects
}

#[derive(Debug,Clone)]
pub struct RenderCommand {
    pub texture: TextureHandle,
    pub layer: RenderLayer,
    pub x: f32,
    pub y: f32,
    pub texture_map_handle: Option<TM_Handle>, 
    //what texture, what layer it is in, where to place it, where to get it
}
#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub struct TM_Handle
{
    pub id:String,
}

pub struct TextureMap{
    pub textures: HashMap<TM_Handle,TextureAsset>,
}

impl TextureMap{
    pub fn new() -> Self{
        Self{
            textures:HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct RenderGrid{
    pub layer: RenderLayer,
    pub grid: Grid,
    pub handle: TM_Handle,
    pub tile_size: f32
}

pub struct RenderMap{
    pub grids: Vec<RenderGrid>,
}
impl RenderMap{


    pub fn new() -> Self {
        Self { grids: Vec::new(), }
    }
}

#[derive(Debug,Clone)]
pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
    pub is_grid: Option<GridHandle>,
    pub t_map: Option<TMapHandle>,
}


pub struct RenderData {
    pub sprites: Option<MockSprite>,
    pub render_maps: Option<RenderMap>
}








///Converting a .png file to a texture struct through img dependency, basically a decoder.
pub fn convert_texture(path: &str) -> Result<Texture,String> {
        

        let img = image::open(path)
            .map_err(|e| format!("Failed to load image: {}", e))?
            .to_rgba8();
        
        let (width, height) = img.dimensions();
        // println!("{:?}",(width,height));
        let pixel_data = img.into_raw();
        Ok(Texture {
            width,
            height,
            pixel_data,
        })
    }

pub fn extract_tileset(tile_h: u32, tile_w: u32, texture: &Texture,) -> Vec<Texture> {
    // rgba size of 4 
    let byte_per_pixel: usize = 4;

    // width and height of texture in u32
    let width_u32 = texture.width;
    let height_u32 = texture.height;
    let width = width_u32 as usize;
    let height = height_u32 as usize;
    // tile width and height as usize
    let tile_w_usize = tile_w as usize;
    let tile_h_usize = tile_h as usize;

    // assert its even
    assert!(width % tile_w_usize == 0);
    assert!(height % tile_h_usize == 0);

    let tiles_x = width / tile_w_usize;
    let tiles_y = height / tile_h_usize;

    let row_stride = width * byte_per_pixel;
    let tile_row_bytes = tile_w_usize * byte_per_pixel;

    let mut tiles = Vec::with_capacity(tiles_x * tiles_y);

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let mut tile = vec![0u8; tile_w_usize * tile_h_usize * byte_per_pixel];

            for row in 0..tile_h_usize {
                let src_y = ty * tile_h_usize + row;
                let src_x = tx * tile_w_usize;
                let src_start = src_y * row_stride + src_x * byte_per_pixel
        ;
                let dst_start = row * tile_row_bytes;
                
                // slice 
                tile[dst_start..dst_start + tile_row_bytes]
                    .copy_from_slice(
                        &texture.pixel_data
                            [src_start..src_start + tile_row_bytes],
                    );
            }

            tiles.push(Texture {
                height: tile_h, // keep u32
                width: tile_w,  // keep u32
                pixel_data: tile,
            });
        }
    }

    tiles
}


pub struct JSON_Meta{
    pub height:u32,
    pub width:u32,
    pub layers: Vec<Vec<u32>>, 
}

///Extracting layer data using serde_json dependency

pub fn extract_layer_data(path: &str) -> Result<Vec<Vec<u32>>, String> {

    let text = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_str(&text)
        .map_err(|e| e.to_string())?;

    let layers = json["layers"]
        .as_array()
        .ok_or("Missing layers array")?;

    let mut result = Vec::new();
    for layer in layers {
        let data = layer["data"]
            .as_array()
            .ok_or("Missing data array")?;

        println!("Found data array with {} entries", data.len());
        let numbers: Vec<u32> = data
            .iter()
            .filter_map(|v| v.as_u64())
            .map(|n| n as u32)
            .collect();

        result.push(numbers);
    }
    Ok(result)
}
pub fn extract_layer_data2(path: &str) -> Result<JSON_Meta, String> {

    let text = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_str(&text)
        .map_err(|e| e.to_string())?;

    let layers = json["layers"]
        .as_array()
        .ok_or("Missing layers array")?;

    let mut result = Vec::new();
    let mut width:u32 = 0;
    let mut height:u32 = 0;
    // let mut result = JSON_Meta{};
    for layer in layers {
        let data = layer["data"]
            .as_array()
            .ok_or("Missing data array")?;

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
    Ok(JSON_Meta{height:height,width:width,layers:result})
}

pub fn json_to_rendermap(path:&str,pix_dim:f32, handle: TM_Handle) -> Result<RenderMap,String>{

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

    for i in 0..layer_amount{
        let layer = &layers[i];
        let mut grid: Grid = Grid::new(width as usize,height as usize,pix_dim);
        match i {
                0 => {
                    println!("Layer 1");
                    let mut count = 0;
                    for y in 0..height{
                        for x in 0..width{
                            let tile = layer[count];

                            if tile != 0 {
                                grid.insert_grid(
                                    (tile - 1) as usize,
                                    GridPosition {
                                        x: x as usize,
                                        y: y as usize
                                    }
                                );
                            }
                            count+=1;
                        }
                    }
                    render_grids.push(RenderGrid{
                        layer: RenderLayer::Background, 
                        grid: grid,
                        handle:handle.clone(),
                        tile_size: pix_dim})
                }

                1 => {
                    println!("Layer 2");
                    let mut count = 0;
                    for y in 0..height{
                        for x in 0..width{
                            let tile = layer[count];

                            if tile != 0 {
                                grid.insert_grid(
                                    (tile - 1) as usize,
                                    GridPosition {
                                        x: x as usize,
                                        y: y as usize
                                    }
                                );
                            }
                            count+=1;
                        }
                    }
                    render_grids.push(RenderGrid{
                        layer: RenderLayer::Terrain, 
                        grid: grid,
                        handle: handle.clone(),
                        tile_size: pix_dim})
                }

                2 => {
                    println!("Layer 2");
                    let mut count = 0;
                    for y in 0..height{
                        for x in 0..width{
                            let tile = layer[count];

                            if tile != 0 {
                                grid.insert_grid(
                                    (tile - 1) as usize,
                                    GridPosition {
                                        x: x as usize,
                                        y: y as usize
                                    }
                                );
                            }
                            count+=1;
                        }
                    }
                    render_grids.push(RenderGrid{
                        layer: RenderLayer::Objects, 
                        grid: grid,
                        handle: handle.clone(),
                        tile_size: pix_dim})
                }

                _ => {}
        }
}
        
        
        println!("Making a rendermap");
        
    Ok(RenderMap{grids:render_grids})

}
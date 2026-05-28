// use crate::assets::texture_map::TM_Handle;
use std::collections::HashMap;
use crate::assets::texture_asset::TextureAsset;
// assets. texture_map.rs
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

// PNG converter
use crate::renderer::texture::Texture;
pub fn convert_texture(path: &str) -> Result<Texture, String> {
    let img = image::open(path)
        .map_err(|e| format!("Failed to load image: {}", e))?
        .to_rgba8();

    let (width, height) = img.dimensions();

    let raw = img.into_raw();

    let mut pixel_data: Vec<u32> =
        Vec::with_capacity((width * height) as usize);

    for rgba in raw.chunks_exact(4) {

        let r = rgba[0] as u32;
        let g = rgba[1] as u32;
        let b = rgba[2] as u32;
        let a = rgba[3] as u32;

        let packed =
            (a << 24) |
            (b << 16) |
            (g << 8)  |
            r;

        pixel_data.push(packed);
    }
    Ok(Texture {
        width,
        height,
        pixel_data,
    })
}

pub fn extract_tileset(
    tile_h: u32,
    tile_w: u32,
    texture: &Texture,
) -> Vec<Texture> {

    let width = texture.width as usize;
    let height = texture.height as usize;

    let tile_w_usize = tile_w as usize;
    let tile_h_usize = tile_h as usize;

    assert!(width % tile_w_usize == 0);
    assert!(height % tile_h_usize == 0);

    let tiles_x = width / tile_w_usize;
    let tiles_y = height / tile_h_usize;

    let mut tiles = Vec::with_capacity(tiles_x * tiles_y);

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {

            let mut tile: Vec<u32> =
                Vec::with_capacity(tile_w_usize * tile_h_usize);

            for row in 0..tile_h_usize {

                let src_y = ty * tile_h_usize + row;
                let src_x = tx * tile_w_usize;

                let src_start = src_y * width + src_x;

                let src_end = src_start + tile_w_usize;

                tile.extend_from_slice(
                    &texture.pixel_data[src_start..src_end]
                );
            }

            tiles.push(Texture {
                height: tile_h,
                width: tile_w,
                pixel_data: tile,
            });
        }
    }

    tiles
}
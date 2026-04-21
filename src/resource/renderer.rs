#![allow(unused)]
use crate::resource::asset_manager::*;

use std::collections::HashMap;
use std::default::Default;
use wgpu::util::DeviceExt;
use crate::engine::system::*;

// window
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
    
};
use winit::window::Window;
use std::sync::Arc;

// pollster
use pollster::block_on;


pub struct Renderer{
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: Arc<Window>,
    pub texture_cache: GPUTextureCache,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline: wgpu::RenderPipeline,
}



impl Renderer {

    pub async fn new(window: Arc<Window>) -> Self {
        // instance
        
        let instance = wgpu::Instance::default();

        // surface
        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        // adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Failed to find an adapter");

        // device and queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("Failed to create device");

        // window
        let size = window.inner_size();

        // surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);
        
        // texture bind group layout
        let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(
                    wgpu::SamplerBindingType::Filtering
                ),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            },
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("render pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            },
        );

            Self {
                surface,
                device,
                queue,
                config,
                window,
                texture_cache: GPUTextureCache::new(), // ✅ now stored
                texture_bind_group_layout,
                pipeline
            }
    }

fn convert_texture(
    &mut self,
    handle: &TextureHandle,
    tex: &Texture,
) -> GpuTextureComp {
        let size = wgpu::Extent3d {
            width: tex.width,
            height: tex.height,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("diffuse_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &tex.data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * tex.width),
                rows_per_image: Some(tex.height),
            },
            size,
        );
        let view = texture.create_view(&Default::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
        });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });
        let mut gpu_tex  = GpuTextureComp{
            texture, 
            view,
            bind_group,
        };

        gpu_tex



    }
   
   
   
//    later later
   
pub fn draw(
        &mut self,
        render_data: GridRenderData,asset_manager: &AssetManager,
    ) 
    
    { 
        // Cacheing doesnt work post refactor of load_texture etc
        // Check if its in cache
        // if not, it simply uploads it to cache
    for cmd in &render_data.tiles {
    let handle = cmd.texture;

    if !self.texture_cache.textures.contains_key(&handle) {
        let tex = asset_manager
            .get(handle); // <-- your AssetManager::get(TextureHandle)

        let gpu_tex = self.convert_texture(&handle, tex);
        self.texture_cache.textures.insert(handle, gpu_tex);
    }
}

        let mut vertices: Vec<Vertex> = Vec::new();

        let tile_size = 0.1; // temp size in NDC
        let window_aspect = self.config.width as f32 / self.config.height as f32;
        let camera = render_data.camera_position;
        let zoom = render_data.camera_zoom;
        let x_scale = tile_size * zoom;
        let y_scale = tile_size * window_aspect * zoom;

        for cmd in &render_data.tiles {
            let tex = asset_manager.get(cmd.texture);
            let x = (cmd.position.0 - camera.0) * x_scale;
            let y = (cmd.position.1 - camera.1) * y_scale;
            let width = x_scale;
            let height = y_scale * tex.height as f32 / tex.width as f32;

            vertices.extend_from_slice(&[
                // triangle 1
                Vertex { position: [x, y], uv: [0.0, 1.0] },
                Vertex { position: [x + width, y], uv: [1.0, 1.0] },
                Vertex { position: [x + width, y + height], uv: [1.0, 0.0] },

                // triangle 2
                Vertex { position: [x, y], uv: [0.0, 1.0] },
                Vertex { position: [x + width, y + height], uv: [1.0, 0.0] },
                Vertex { position: [x, y + height], uv: [0.0, 0.0] },
            ]);
        }

        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );



        // 
        // 
        // 




        let output = self.surface
            .get_current_texture()
            .expect("Failed to get surface texture");

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        // for cmd in &render_data.tiles {
        //     let gpu_tex = self.texture_cache.textures.get(&cmd.texture).unwrap();

        //     render_pass.set_bind_group(0, &gpu_tex.bind_group, &[]);

        //     // draw call (temporary)
        //     render_pass.draw(0..6, 0..1);
        // }


        let mut vertex_offset = 0;

        for cmd in &render_data.tiles {
            let gpu_tex = self.texture_cache.textures.get(&cmd.texture).unwrap();

            render_pass.set_bind_group(0, &gpu_tex.bind_group, &[]);

            render_pass.draw(vertex_offset..vertex_offset + 6, 0..1);

            vertex_offset += 6;
        }
        // issue GPU draw commands
        // issue GPU draw commands
        // issue GPU draw commands

        // println!("\n\nWork breakpoint:");
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        output.present();


        // println!("\n\n\nEnd of work breakpoint:");
        
        
    }


    fn tile_to_ndc(x: f32, y: f32, tile_size: f32) -> [f32; 2] {
        [
            x * tile_size,
            y * tile_size,
        ]
    }
}



// 
pub struct GPUTextureCache {
    pub textures: HashMap<TextureHandle, GpuTextureComp>,
}
impl GPUTextureCache{
    pub fn new() -> Self {
        Self{ textures: HashMap::new()}
    }
}

// GPU Textures
pub struct GpuTextureComp {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
}

// Gpu renders triangles, 
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}
impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
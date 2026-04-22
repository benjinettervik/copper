#![allow(unused)]
pub mod system;
pub mod world;
pub mod scheduler;
pub mod camera;
pub mod meta;

pub use camera::*;


use system::*;
use world::*;
use winit::event::{WindowEvent,Event};
use winit::window::{WindowBuilder};
use winit::event_loop::EventLoop;
use crate::resource::renderer::Renderer;
use crate::resource::resources::Resources;
use crate::engine::scheduler::Scheduler;
use std::sync::Arc;
use crate::input::*;
use std::any::TypeId;

pub struct Engine {
    pub world: World,
    pub scheduler: Scheduler,
    pub resources: Resources,
}

impl Engine {
    pub fn new() -> Self {
        let mut resources = Resources::new();
        resources.insert(Camera2D::new());

        Self {
            world: World::new(),
            // systems: vec![], // perhaps add default systems here, rendering, etc
            resources,
            scheduler: Scheduler::new(),
        }
        
    }

    pub fn register_system<T: System + 'static>(&mut self, system: T) {
        // temporary 
        self.scheduler.add_update_system(system);
    }
    

    // for testing
   pub fn no_ev_loop_run(&mut self, cycles: usize) {
        self.scheduler.run_startup(&mut self.world, &mut self.resources);

        for _ in 0..cycles {
            self.scheduler.run_update(&mut self.world, &mut self.resources);
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(
            WindowBuilder::new()
                .build(&event_loop)
                .unwrap()
        );
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        
        self.resources.insert(renderer);
        self.scheduler.run_startup(&mut self.world, &mut self.resources);
        self.resources.insert( Input::new());

        event_loop.run(move |event, elwt| {
            match event {

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { event, .. } => {

                        if let Some(input) = self.resources.get_mut::<Input>() {
                            input.handle_input(event);
                        }
                    }

                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }

                    _ => {}
                },

                Event::AboutToWait => {
                    self.scheduler.run_update(&mut self.world, &mut self.resources);
                    if let Some(input) = self.resources.get_mut::<Input>() {
                            input.clear_frame();
                        }
                    window.request_redraw();
                }

                _ => {}
            }
        }).unwrap();
}
}

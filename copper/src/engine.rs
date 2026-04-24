// pub mod query;
pub mod scheduler;
pub mod system;
pub mod world;

use scheduler::*;
use system::*;
use world::*;

use crate::renderer::render_sys::*;
use crate::renderer::test_components_renderer::*;
use pixels::{Pixels, SurfaceTexture};
use std::any::TypeId;
use std::cell::*;
use std::sync::Arc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;
// use crate::renderer::render_sys::*;
use crate::Component;
use crate::renderer::Renderer;
use crate::renderer::render_sys::RenderSys;
use crate::resource::Resources;
type EntityId = usize;

pub struct Engine {
    pub world: World,
    scheduler: Scheduler,
    // added more to engine
    pub resources: Resources,
    pub renderer: Option<Renderer>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),

            // resources + renderer added
            resources: Resources::new(),
            renderer: None,
        }
    }

    pub fn add_system<T1, T2>(&mut self, system_routine: T1, system: T2) -> &mut Self
    where
        T1: SystemRoutine + 'static,
        T2: System + 'static,
    {
        self.scheduler.add_system(system_routine, system);
        self
    }

    pub fn run(&mut self) {
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        loop {
            self.scheduler
                .run_update(&mut self.world, &mut self.resources);
        }
    }

    pub fn query(
        &self,
        components_read: &Vec<TypeId>,
        components_write: &Vec<TypeId>,
        components_with: &Vec<TypeId>,
        components_without: &Vec<TypeId>,
    ) -> Vec<EntityId> {
        self.world.query(
            components_read,
            components_write,
            components_with,
            components_without,
        )
    }

    pub fn run_cycles(&mut self, cycles: usize) {
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        for _ in 0..cycles {
            self.scheduler
                .run_update(&mut self.world, &mut self.resources);
        }
    }

    //
    // ###
    // ### Fixed a test run for being able to see the window -- now run update systems in scheduler
    // ### the rendering requires RenderSys to be added as update
    // ###
    //

    pub fn test_run(&mut self) {
        println!("In test run!");

        // run startup sys
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        // winit eventloop time
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();

        event_loop
            .run(move |event, elwt| {
                match event {
                    Event::Resumed => {
                        let window = elwt.create_window(Window::default_attributes()).unwrap();

                        // renderer owns window
                        self.renderer = Some(Renderer::new(window));
                    }

                    // Update sys
                    Event::AboutToWait => {
                        // Update ECS
                        self.scheduler
                            .run_update(&mut self.world, &mut self.resources);

                        // Queue redraw event since renderer own window
                        if let Some(renderer) = &self.renderer {
                            renderer.request_redraw();
                        }
                    }

                    // Window event -> renderer
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::RedrawRequested => {
                            if let Some(renderer) = &mut self.renderer {
                                // render draw
                                renderer.draw(&mut self.resources);
                            }
                        }

                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }

                        _ => {}
                    },

                    _ => {}
                }
            })
            .unwrap();
    }
}

pub trait SystemRoutine {}

pub struct Startup;
impl SystemRoutine for Startup {}

pub struct Update;
impl SystemRoutine for Update {}

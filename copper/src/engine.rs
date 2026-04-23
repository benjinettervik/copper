pub mod scheduler;
pub mod system;
pub mod world;


use scheduler::*;
use system::*;
use world::*;

use std::sync::Arc;
use winit::window::Window;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use pixels::{Pixels, SurfaceTexture};
use crate::renderer::test_components_renderer::*;
use crate::renderer::render_sys::*;
// use crate::renderer::render_sys::*;
use crate::resource::Resources;
use crate::renderer::render_sys::RenderSys;
use crate::renderer::Renderer;

pub struct Engine {
    pub world: World,
    scheduler: Scheduler,
    pub resources: Resources,
    pub renderer: Renderer,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),
            resources: Resources::new(),
            renderer: Renderer::new(),
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

    pub fn run_cycles(&mut self, cycles: usize) {
        self.scheduler.run_startup(&mut self.world);

        for _ in 0..cycles {
            self.scheduler.run_update(&mut self.world);
        }
    }

    pub fn test_run(&mut self)
    {
        println!("In test run!");
        self.scheduler.run_startup(&mut self.world);
        
        // window
        let mut window: Option<Window> = None;
        
        // eventloop
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        event_loop.run(move |event, elwt| {
            match event {
                Event::Resumed => {
                    window = Some(
                        elwt.create_window(Window::default_attributes())
                            .unwrap()
                    );
                }
                

                Event::AboutToWait => {
                // update
                self.scheduler.run_update(&mut self.world);
                
                // rendersys can provide
                self.renderer.draw();
                // 
                
            }   

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }

                    _ => {}
                },

                _ => {}
            }
        }).unwrap();            
    }


    }



// }

pub trait SystemRoutine {}

pub struct Startup;
impl SystemRoutine for Startup {}

pub struct Update;
impl SystemRoutine for Update {}


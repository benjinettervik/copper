//! This module is the main module to use when developing in Copper! ut contains all necessary functions to create a simple game.

pub mod scheduler;
pub mod system;
pub mod world;

use scheduler::*;
use system::*;
use world::*;

use crate::input::Input;
use crate::renderer::Renderer;
use crate::resource::Resources;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

use std::thread;
use std::time::{Duration, SystemTime};

/// The 'Engine' struct represents the engine itself. It contains the necessary functions used to manipulate the game engine.
///
/// Example of use:
/// ```ignore
/// let mut engine = Engine::new().
/// add_system(Startup, ExampleSystem).
/// run();
/// ```

pub struct Engine {
    pub world: World,
    scheduler: Scheduler,
    pub resources: Resources,
    pub renderer: Option<Renderer>,
}

impl Engine {
    /// Initializes the engine and returns a new 'Engine' struct.
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),

            // resources + renderer added
            resources: Resources::new(),
            renderer: None,
        }
    }

    /// Adds a system into the engine. Set a system routine (for example Startup or Update) to when it will run.
    pub fn add_system<T1, T2>(&mut self, system_routine: T1, system: T2) -> &mut Self
    where
        T1: SystemRoutine + 'static,
        T2: System + 'static,
    {
        self.scheduler.add_system(system_routine, system);
        self
    }

    /// Runs all systems that have been added to an 'Engine'. Does not terminate naturally.
    pub fn run(&mut self) -> &mut Self {
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        loop {
            self.scheduler
                .run_update(&mut self.world, &mut self.resources);
        }
    }

    /// Runs all systems that have been added to an 'Engine' a set number of times. Terminates after all cycles have run.
    pub fn run_cycles_concurrent(&mut self, cycles: usize) -> &mut Self {
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        let dep = self.scheduler.make_dep_graph();
        let sorted = self.scheduler.sort_systems(dep);

        for _ in 0..cycles {
            self.scheduler
                .run_prio_update(&mut self.world, &mut self.resources, &sorted);
        }

        self
    }

    pub fn run_cycles(&mut self, cycles: usize) -> &mut Self {
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        for _ in 0..cycles {
            self.scheduler
                .run_update(&mut self.world, &mut self.resources);
        }

        self
    }

    //
    // ###
    // ### Fixed a test run for being able to see the window -- now run update systems in scheduler
    // ### the rendering requires RenderSys to be added as update
    // ###
    //

    pub fn test_run(&mut self, time: u64) {
        println!("In test run!");

        // run startup sys
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        let dep = self.scheduler.make_dep_graph();
        let sorted = self.scheduler.sort_systems(dep);
        let d_graph = self.scheduler.make_dep_graph();

        // winit eventloop time
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();

        event_loop
            .run(move |event, elwt| {
                match event {
                    Event::Resumed => {
                        let window = elwt
                            .create_window(
                                Window::default_attributes()
                                    .with_title("Copper")
                                    .with_inner_size(LogicalSize::new(1280.0, 720.0)),
                            )
                            .unwrap();

                        // renderer owns window
                        self.renderer = Some(Renderer::new(window));
                    }

                    // Update sys
                    Event::AboutToWait => {
                        println!("In AboutToWait");
                        // Update ECS
                        // self.scheduler.sort_systems(d_graph);
                        // self.scheduler
                        //     .run_update(&mut self.world, &mut self.resources);

                        let time = SystemTime::now();
                        self.scheduler.run_prio_update(
                            &mut self.world,
                            &mut self.resources,
                            &sorted,
                        );
                        println!("Update systems time: ")
                        //Då detta är en test run så borde detta flyttas till tick systemet senare
                        // self.resources.input.input_polling();
                        self.resources.get_mut::<Input>().unwrap().input_polling();

                        // Queue redraw event since renderer own window
                        if let Some(renderer) = &self.renderer {
                            renderer.request_redraw();
                        }
                    }

                    // Window event -> renderer
                    Event::WindowEvent { event, .. } => match event {
                        //cursor, keys, mouse också flyttas till tick systemet senare
                        WindowEvent::KeyboardInput { event, .. } => {
                            // self.resources.input.handle_keys(event);
                            self.resources
                                .get_mut::<Input>()
                                .unwrap()
                                .handle_keys(event);
                        }

                        WindowEvent::MouseInput { state, button, .. } => {
                            self.resources
                                .get_mut::<Input>()
                                .unwrap()
                                .handle_mouse_button(button, state);
                        }

                        WindowEvent::CursorMoved { position, .. } => {
                            self.resources
                                .get_mut::<Input>()
                                .unwrap()
                                .handle_mouse_movement(position.x, position.y);
                        }

                        WindowEvent::RedrawRequested => {
                            if let Some(renderer) = &mut self.renderer {
                                // render draw
                                renderer.draw(&mut self.resources);
                                thread::sleep(Duration::from_millis(time));
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

/// A system routine for systems which only run at startup.
pub struct Startup;
impl SystemRoutine for Startup {}

/// A system routine for systems which run every engine tick.
pub struct Update;
impl SystemRoutine for Update {}

//! This module is the main module to use when developing in Copper! ut contains all necessary functions to create a simple game.
use std::cell::Cell;
use std::rc::Rc;
use std::time::SystemTime;

use crate::ecs::scheduler::*;
use crate::ecs::system::*;
use crate::ecs::world::*;

use crate::input::Input;
use crate::resource::config::CopperConfig;
use crate::renderer::Renderer;
use crate::resource::Resources;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::resource::time::Time;
use std::time::Instant;
use std::thread;
use std::time::Duration;

// use std::thread;
// use std::time::{Duration, SystemTime};

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

    //
    // ###
    // ### Fixed a test run for being able to see the window -- now run update systems in scheduler
    // ### the rendering requires RenderSys to be added as update
    // ###
    //

    pub fn run_render(&mut self, concurrent: bool) {
        let start_time = SystemTime::now();
        let start_time_res = Instant::now();
        let mut last_frame = Instant::now();

        // to count frames for performance stats, need reference counting and cell since winit
        // moves
        let total_frames = Rc::new(Cell::new(0usize));
        let total_frames_closure = Rc::clone(&total_frames);
        // run startup sys
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        // create dependencies for scheduler
        let dep = self.scheduler.make_dep_graph();
        let guard_rails = self.resources.get::<CopperConfig>().unwrap().scheduler_guard_rails;
        // println!("{:?}",guard_rails);
        let sorted = self.scheduler.sort_systems(dep,guard_rails);

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
                        total_frames_closure.set(total_frames_closure.get() + 1);

                        // 
                        // #####
                        // ##### Added so that Time resource can be used.
                        // #####
                        //
                        let now = Instant::now();
                        let delta = now - last_frame;
                        // println!("{:?}",delta);
                        last_frame = now;
                        let delta_seconds = delta.as_secs_f32();
                        let time = self.resources.get_mut::<Time>().unwrap();
                        time.register_delta(delta_seconds);

                        // #####
                        // #####
                        // #####
                        //
                        
                        if concurrent {
                            self.scheduler.run_update(
                                &mut self.world,
                                &mut self.resources,
                                &sorted,
                            );
                        } else {
                            self.scheduler.run_update_non_concurrent(
                                &mut self.world,
                                &mut self.resources,
                                &sorted,
                            );
                        }
                        self.resources.get_mut::<Input>().unwrap().input_polling();

                        // queue redraw event since renderer own window
                        if let Some(renderer) = &self.renderer {
                            renderer.request_redraw();
                        }

                        // 
                        // Sleep?
                        // Uncomment if you want 20+ ms speed
                        // thread::sleep(Duration::from_millis(20));
                        // 
                        // 
                        // 
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

        let time_ran = start_time.elapsed().unwrap().as_secs() as usize;

        let to_print;
        if concurrent {
            to_print = "concurrently";
        } else {
            to_print = "non-concurrently"
        }
        println!(
            "============================================================================================="
        );
        println!(
            "Engine ran {} for {} seconds with an average FPS of {}!",
            to_print,
            time_ran,
            (total_frames.get() / time_ran)
        );
        println!(
            "============================================================================================="
        );
    }

    pub fn run_without_render(&mut self, concurrent: bool, seconds: u64) {
        let start_time = SystemTime::now();

        // to count frames for performance stats, need reference counting and cell since winit
        // moves
        let mut total_frames: usize = 0;
        // run startup sys
        self.scheduler
            .run_startup(&mut self.world, &mut self.resources);

        // create dependencies for scheduler
        let dep = self.scheduler.make_dep_graph();
        let sorted = self.scheduler.sort_systems(dep,false);

        let start_time = SystemTime::now();

        while start_time.elapsed().unwrap().as_secs() < seconds {
            total_frames += 1;
            if concurrent {
                self.scheduler
                    .run_update(&mut self.world, &mut self.resources, &sorted);
            } else {
                self.scheduler.run_update_non_concurrent(
                    &mut self.world,
                    &mut self.resources,
                    &sorted,
                );
            }
        }

        let time_ran = start_time.elapsed().unwrap();

        let to_print;
        if concurrent {
            to_print = "concurrently";
        } else {
            to_print = "non-concurrently"
        }

        println!(
            "============================================================================================="
        );
        println!(
            "Engine ran {} for around {} second(s) with an average FPS of {}!",
            to_print,
            start_time.elapsed().unwrap().as_secs(),
            (total_frames as f64 / (start_time.elapsed().unwrap().as_millis() as f64 / 1000.0))
                .round()
        );
        println!(
            "============================================================================================="
        );
    }
}

pub trait SystemRoutine {}

/// A system routine for systems which only run at startup.
pub struct Startup;
impl SystemRoutine for Startup {}

/// A system routine for systems which run every engine tick.
pub struct Update;
impl SystemRoutine for Update {}

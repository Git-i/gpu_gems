use std::{collections::HashMap, sync::Arc};

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window};

use crate::context::Context;

fn default_run(_ctx: &Context) {
    println!("update");
}
pub struct Application
{
    windows: HashMap<winit::window::WindowId, Arc<Window>>,
    pub frame_fn: fn(&Context),
    event_loop: EventLoop<()>
}

impl Application
{
    pub fn new() -> Application {
        Application {
            windows: HashMap::new(),
            frame_fn: default_run,
            event_loop: EventLoop::new().expect("Failed to make event loop")
        }
    }
    pub fn add_window(&mut self, window: Arc<Window>) {
        self.windows.insert(window.id(), window);
    }
    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}

struct ApplicationInternal
{
    windows: HashMap<winit::window::WindowId, Arc<Window>>,
    pub frame_fn: fn(&Context),
    ctx: Context
}
//runs an app and returns a context without the potentially destoryed window and swapcahin
pub fn run_app(app: Application, ctx: Context) -> Context {
    let mut internal = ApplicationInternal {
        windows: app.windows, 
        frame_fn: app.frame_fn,
        ctx
    };
    let _ = app.event_loop.run_app(&mut internal);
    internal.ctx.invalidate_application();
    internal.ctx
}
impl ApplicationHandler for ApplicationInternal
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let window = self.windows.get(&window_id).expect("Window was not registered");
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                (self.frame_fn)(&self.ctx);
                window.request_redraw();
            }
            _ => ()
        }
    }
}
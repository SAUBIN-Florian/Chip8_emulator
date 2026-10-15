#![allow(dead_code, unused_variables)]

mod chip8;
//mod graphics;

use std::sync::Arc;

use winit::dpi::LogicalSize;
use winit::window::Window;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;

use chip8::Chip8;
//use graphics::GraphicsState;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 256;

struct App {
    window: Arc<Window>,
    //g_state: GraphicsState,
    chip8: Chip8
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Arc::new(event_loop.create_window(
            Window::default_attributes()
                .with_title("Chip8 Emulator")
                .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        ).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            // TODO: implement an keyboard input handler
            WindowEvent::RedrawRequested => {
                // Drawing goes brrrr
                // TODO: implement primitive with wgpu for renderering a buffer of pixels
                // TODO: cadence the game loop at a decent framerate

                self.window.as_ref().request_redraw();
            },
            _ => ()
        }
    } 
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    //let mut app: App;
    //let _ = event_loop.run_app(&mut app);
}

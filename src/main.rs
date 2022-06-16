mod engine;

use pollster::FutureExt;
use wgpu::SurfaceError;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use crate::engine::RenderContext;

fn create_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Hello World")
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_visible(false)
        .build(&event_loop)
        .expect("Could not create Window");
    (event_loop, window)
}

fn on_window_event(ctx: &mut RenderContext, event: &WindowEvent, flow: &mut ControlFlow)  {
    match event {
        WindowEvent::CloseRequested => {
            *flow = ControlFlow::Exit
        },
        WindowEvent::KeyboardInput { input, .. } => on_keyboard_input(input, flow),
        WindowEvent::Resized(size) => ctx.resize(*size),
        _ => ()
    }
}

fn on_keyboard_input(event: &KeyboardInput, flow: &mut ControlFlow) {
    if let Some(key) = &event.virtual_keycode {
        match event.state {
            ElementState::Pressed => {
                println!("Key pressed: {:?}", &key)
            },
            ElementState::Released => {
                match key {
                    VirtualKeyCode::Escape => *flow = ControlFlow::Exit,
                    _ => ()
                }
            }
        }
    }
}

fn main() {

    let (event_loop, window) = create_window();

    let mut ctx = RenderContext::new(&window).block_on();

    window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: win_event, window_id } if window_id == window.id() => {
                on_window_event(&mut ctx, &win_event, control_flow);
            },
            Event::MainEventsCleared => {
                // Update and draw
                match ctx.render() {
                    Ok(_) => (),
                    Err(SurfaceError::Lost) => ctx.resize(ctx.size),
                    Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("Render error: {:?}", e),
                }
            },
            _ => ()
        }
    });

}

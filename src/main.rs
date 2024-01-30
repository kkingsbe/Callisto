mod shader;
mod shaderprogram;
mod renderer;
mod uniform;
mod particle;
mod simulation;

use glutin::{Api, ContextBuilder, GlRequest};
use glutin::event::{Event, WindowEvent};
use glutin::event::WindowEvent::MouseInput;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;

use crate::renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Callisto").with_inner_size(glutin::dpi::LogicalSize::new(800.0, 800.0));

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Cannot make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let mut renderer = Renderer::new().expect("Cannot create renderer");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    println!("Window resized");
                    gl_context.resize(physical_size)
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => {
                    renderer.set_mouse_position(position.x as f32, position.y as f32);
                },
                MouseInput { button, .. } => {
                    match button {
                        glutin::event::MouseButton::Left => {
                            renderer.on_mouse_click();
                        },
                        _ => ()
                    }
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glutin::event::ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(glutin::event::VirtualKeyCode::LControl) => {
                                renderer.on_keypress(renderer::KEY::LCTRL)
                            },
                            _ => (),
                        }
                    }
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.draw();
                gl_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => gl_context.window().request_redraw(), //Trigger drawing the next frame
            _ => (),
        }
    });
}
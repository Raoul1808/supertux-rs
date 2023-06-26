mod assets;
mod graphics;

use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

struct State {
    size: PhysicalSize<u32>,
    window: Window,
    renderer: graphics::render::RenderContext,
    mouse_x: f64,
    mouse_y: f64,
    sprite: Sprite,
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let renderer = graphics::render::RenderContext::new(&window, size)
            .await;

        let sprite = Sprite::new_from_x_y(100.0, 100.0, 100.0, 100.0);

        Self {
            renderer,
            size,
            window,
            mouse_x: 0.0,
            mouse_y: 0.0,
            sprite,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.renderer.resize(new_size);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_x = position.x;
                self.mouse_y = position.y;
                true
            }
            _ => false
        }
    }

    fn update(&mut self) {
        self.sprite.set_position(
            self.mouse_x as f32,
            self.mouse_y as f32,
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let vertices = self.sprite.get_vertex_data();
        println!("{:?}", vertices);
        self.renderer.fill_buffers(&vertices, &vec![0, 1, 2, 2, 3, 0]);
        self.renderer.render()
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use crate::graphics::sprite::Sprite;
use crate::graphics::Vertex;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger.");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.body()?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => if !state.input(event) {
            match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                },
                _ => {}
            }
        },
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Event::MainEventsCleared => {
            state.window().request_redraw();
        },
        _ => {}
    })
}

#![allow(unused_imports)]
#![allow(non_snake_case)]


extern crate conrod;
extern crate renert;

pub use conrod::{widget, color, Colorable, Borderable, Sizeable, Positionable, Labelable, Widget};
pub use conrod::backend::glium::glium;
pub use conrod::backend::glium::glium::{DisplayBuild, Surface};
pub use renert::*;

#[macro_export]
macro_rules! gen_ids {
    ($($id:tt),*) => {
        widget_ids!(struct Ids { $($id),* });
    };
    ($($id:tt),*,) => {
        widget_ids!(struct Ids { $($id),* });
    };
}

#[macro_export]
macro_rules! is_inputed {
    ($k: tt) => {
        glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::$k))
    }
}

#[macro_export]
macro_rules! is_closed {
    () => {
        glium::glutin::Event::Closed
    }
}

pub fn gen_display(title: &str, width: u32, height: u32) -> conrod::glium::Display {
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(width, height)
        .with_title(title)
        //.with_multisampling(4)
        .build_glium()
        .unwrap();
    display
}

pub fn gen_ui(width: f64, height: f64) -> conrod::Ui {
    let ui = conrod::UiBuilder::new([width, height]).build();
    ui
}

pub fn gen_renderer(display: &conrod::glium::Display) -> Result<conrod::backend::glium::Renderer, conrod::backend::glium::RendererCreationError> {
    let renderer = conrod::backend::glium::Renderer::new(display);
    renderer
}

pub fn gen_imageMap() -> conrod::image::Map<glium::texture::Texture2d> {
    let imageMap = conrod::image::Map::<glium::texture::Texture2d>::new();
    imageMap
}

pub fn gen_eventLoop() -> EventLoop {
    let eventLoop = EventLoop::new();
    eventLoop
}

pub fn handle_event(ui: &mut conrod::Ui, eventLoop: &mut EventLoop, event: &glium::glutin::Event, display: &conrod::glium::Display) {
    if let Some(event) = conrod::backend::winit::convert(event.clone(), display) {
        ui.handle_event(event);
        eventLoop.needs_update();
    }
}

pub fn draw(ui: &mut conrod::Ui, renderer: &mut conrod::backend::glium::Renderer, display: &mut conrod::glium::Display, imageMap: &mut conrod::image::Map::<glium::texture::Texture2d>) {
    if let Some(primitives) = ui.draw_if_changed() {
        renderer.fill(display, primitives, imageMap);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(display, &mut target, imageMap).unwrap();
        target.finish().unwrap();
    }
}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events.extend(display.poll_events());

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

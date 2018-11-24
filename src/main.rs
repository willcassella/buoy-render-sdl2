extern crate buoy;
use buoy::{Window, WidgetId, IntoObj};
use buoy::layout::{Point, Area, Region};
use buoy::commands::CommandList;

mod ui;
use ui::{TestGenerator};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 1920, 1080)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Build the UI
        let window_size = canvas.output_size().unwrap();
        let ui_commands = build_ui(window_size.0 as f32, window_size.1 as f32);

        // Render the UI
        canvas.clear();
        render_ui(&mut canvas, &ui_commands);
        canvas.present();
    }
}

fn render_ui(canvas: &mut WindowCanvas, commands: &CommandList) {
    for quad in &commands.colored_quads {
        canvas.set_draw_color(Color::RGBA(quad.color.red(), quad.color.green(), quad.color.blue(), quad.color.alpha()));
        let rect = Rect::new(quad.quad.x as i32, quad.quad.y as i32, quad.quad.width as u32, quad.quad.height as u32);
        canvas.fill_rect(rect).unwrap();
    }
}

fn build_ui(window_width: f32, window_height: f32) -> CommandList {
    let window_region = Region {
        pos: Point::zero(),
        area: Area {
            width: window_width,
            height: window_height,
        },
    };

    // Build UI
    let mut ctx = Window::default();
    let elem_obj = ctx.run(window_region.area, TestGenerator.into_obj(WidgetId::str("root")).erase()).expect("Failed to build UI");

    // Render UI
    let mut commands = CommandList::default();
    elem_obj.element.render(window_region, &mut commands);
    return commands;
}
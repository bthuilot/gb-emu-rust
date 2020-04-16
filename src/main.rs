use std::time::Duration;
use std::f64::INFINITY;
use crate::gameboy::{Gameboy, Options};
use crate::display::{SCREEN_WIDTH, SCREEN_HEIGHT};
use ticker::Ticker;
use crate::runner::Runner;
use winit::event_loop::{ControlFlow, EventLoop};
use pixels::{Pixels, SurfaceTexture};
use winit::event::Event;
use std::error::Error;
use pixel_canvas::{input::MouseState, Canvas, Color};

mod cpu;
mod memory;
mod ops;
mod bit_functions;
mod gameboy;
mod cart;
mod input;
mod speed;
mod cb_ops;
mod display;
mod palette;
mod runner;

fn main() -> () {

//    let mut runner = Runner::new("/Users/bryce/PokemonRed.gb");
//    runner.run();
    let mut gb = Gameboy::new("/Users/bryce/Downloads/cpu_instrs/cpu_instrs.gb", Options {
        sound: false,
        cgb: false
    });

    let canvas = Canvas::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)
        .title("Tile")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    // The canvas will render for you at up to 60fps.
    canvas.render( move |mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let color = gb.screen_data[x][y];
                *pixel = Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                }
            }
        }
        gb.update();
    });



//    let event_loop = EventLoop::new();
//    let title = gb.memory.cart.title.as_str();
//    let (window, surface, width, height, mut hidpi_factor) = Runner::create_window(title, &event_loop);
//    let surface_texture = SurfaceTexture::new(width, height, surface);
//    let mut pixels = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).expect("Penis");
//    event_loop.run(move |event, _, control_flow| {
//        // The one and only event that winit_input_helper doesn't have for us...
//        if let Event::RedrawRequested(_) = event {
//            let mut frame = pixels.get_frame();
//            for x in 0..gb.prepared_screen.len() {
//                for y in 0..gb.prepared_screen[x].len() {
//                    let color = gb.prepared_screen[x][y];
//                    let index = 4 * (x + (y * (SCREEN_WIDTH as usize)));
//                    frame[index] = 255;
//                    frame[index + 1] = 255;
//                    frame[index + 2] = 255;
//                    frame[index + 3] = 0xFF;
//                }
//            }
//            if pixels.render().is_err() {
//                *control_flow = ControlFlow::Exit;
//                println!("ERROR>?");
//                return;
//            }
//        }
//        gb.update();
//        println!("here");
//        window.request_redraw();
//    });

    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
}

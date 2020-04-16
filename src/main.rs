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
    let mut gb = Gameboy::new("/Users/bryce/PokemonRed.gb", Options {
        sound: false,
        cgb: false
    });

    let event_loop = EventLoop::new();
    let title = gb.memory.cart.title.as_str();
    let (window, surface, width, height, mut hidpi_factor) = Runner::create_window(title, &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).expect("Penis");
    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            let mut frame = pixels.get_frame();
            for y in 0..gb.prepared_screen.len() {
                for x in 0..gb.prepared_screen[y].len() {
                    let color = gb.prepared_screen[y][x];
                    let index = 4 * (x + (y * (SCREEN_WIDTH as usize)));
                    if index < frame.len() {
                        frame[index] = color.r;
                        frame[index + 1] = color.g;
                        frame[index + 2] = color.b;
                        frame[index + 3] = 0xFF;
                    }
                }
            }
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        gb.update();
        window.request_redraw();
    });


    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
}

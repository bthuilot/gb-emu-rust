use std::time::Duration;
use std::f64::INFINITY;
use crate::gameboy::{Gameboy, Options};
use crate::display::{SCREEN_WIDTH, SCREEN_HEIGHT};
use winit_input_helper::WinitInputHelper;
use winit::event_loop::{ControlFlow, EventLoop};
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use std::error::Error;
use crate::pixels_helper::create_window;
extern crate nfd;

use nfd::Response;
use std::process::exit;


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
mod pixels_helper;

fn main() -> () {

    let mut correct_file: bool = false;
    let mut file = String::new();
    while !correct_file {
        let result = nfd::open_file_dialog(Some(".gb"), None).unwrap_or_else(|e| {
            panic!(e);
        });
        match result {
            Response::Okay(file_path) => {
                file = file_path;
                correct_file = true;
            }
            Response::OkayMultiple(files) => {
                println!("Please only select one file")
            }
            Response::Cancel => {
                println!("User canceled");
                exit(0);
            }
        }
    }

    let mut gb = Gameboy::new(file.as_str(), Options {
        sound: false,
        cgb: false
    });

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let title = gb.memory.cart.title.as_str();
    let (window, surface, width, height, mut hidpi_factor) = create_window(title, &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).expect("Penis");
    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            let mut frame = pixels.get_frame();
            for x in 0..gb.prepared_screen.len() {
                for y in 0..gb.prepared_screen[x].len() {
                    let color = gb.prepared_screen[x][y];
                    let index = 4 * (x + (y * (SCREEN_WIDTH as usize)));
                    frame[index] = color.r;
                    frame[index + 1] = color.g;
                    frame[index + 2] = color.b;
                    frame[index + 3] = 0xFF;
                }
            }
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                println!("ERROR>?");
                return;
            }
        }
        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            gb.update();
            window.request_redraw();
        }
    });
}

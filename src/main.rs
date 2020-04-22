mod bit_functions;
mod cart;
mod cpu;
mod gameboy;
mod input;
mod graphics;
mod memory;
mod pixels_helper;

extern crate nfd;
use crate::gameboy::Gameboy;
use crate::pixels_helper::{create_window, get_keymap};
use crate::input::Button;
use crate::graphics::{SCREEN_WIDTH, SCREEN_HEIGHT};
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use nfd::Response;
use std::process::exit;
use std::collections::HashMap;

fn main() -> () {
    let mut correct_file: bool = false;
    let mut file = String::new();
    while !correct_file {
        let result = nfd::dialog().filter("gb").filter("gbc").open().unwrap_or_else(|e| {
            panic!(e);
        });
        match result {
            Response::Okay(file_path) => {
                file = file_path;
                correct_file = true;
            }
            Response::OkayMultiple(_files) => println!("Please only select one file"),
            Response::Cancel => {
                println!("User canceled");
                exit(0);
            }
        }
    }

    let mut gb = Gameboy::new(
        file.as_str(),
    );

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let title = gb.memory.cart.title.as_str();
    let (window, surface, width, height, mut hidpi_factor) = create_window(title, &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels =
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).expect("Unable to open screen");
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            for x in 0..gb.rendered_screen.len() {
                for y in 0..gb.rendered_screen[x].len() {
                    let color = gb.rendered_screen[x][y];
                    let index = 4 * (x + (y * (SCREEN_WIDTH as usize)));
                    frame[index] = color.r;
                    frame[index + 1] = color.g;
                    frame[index + 2] = color.b;
                    frame[index + 3] = 0xFF;
                }
            }
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                println!("Unable to render pixels");
                return;
            }
        }
        // Handle input events
        if input.update(event) {
            // Set key bindings
            let keymap: HashMap<VirtualKeyCode, Button> = get_keymap();
            for (keycode, button) in keymap.iter() {
                if input.key_pressed(*keycode) {
                    gb.press_button(*button);
                }
                if input.key_released(*keycode) {
                    gb.release_button(*button);
                }
            }

            // Change Palette
            if input.key_pressed(VirtualKeyCode::P) {
                gb.swap_palette();
            }

            // Set speed up
            if input.key_pressed(VirtualKeyCode::Space) {
                gb.toggle_speed(true);
            }

            if input.key_released(VirtualKeyCode::Space) {
                gb.toggle_speed(false);
            }

            // Close events
            if input.quit() {
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

            // Update Gameboy
            gb.update();
            // Redraw the window
            window.request_redraw();
        }
    });
}

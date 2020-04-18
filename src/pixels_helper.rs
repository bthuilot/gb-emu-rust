use crate::gameboy::Gameboy;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use pixels::{Pixels, SurfaceTexture};
use std::error::Error;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

// Sample code from pixels examples
pub(crate) fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, pixels::wgpu::Surface, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        let size = window.current_monitor().size();
        (
            size.width as f64 / hidpi_factor,
            size.height as f64 / hidpi_factor,
        )
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round();

    // Resize, center, and display the window
    let min_size = PhysicalSize::new(width, height).to_logical::<f64>(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let surface = pixels::wgpu::Surface::create(&window);
    let size = default_size.to_physical::<f64>(hidpi_factor);
    (
        window,
        surface,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

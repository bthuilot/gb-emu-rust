use pixel_canvas::{input::MouseState, Canvas, Color};

fn main() {
    let r1: u8 = 250;
    let r2: u8 = 23;
    println!("{}", r1 + r2);

    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(512, 512)
        .title("Gameboy")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    // The canvas will render for you at up to 60fps.
    canvas.render(|mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - mouse.x;
                let dy = y as i32 - mouse.y;
                let dist = dx * dx + dy * dy;
                *pixel = Color {
                    r: if dist < 128 * 128 { dy as u8 } else { 0 },
                    g: if dist < 128 * 128 { dx as u8 } else { 0 },
                    b: (x * y) as u8,
                }
            }
        }
    });
}

use GameboyEmulator::z80::Z80;

fn main() {
    let r1: u8 = 250;
    let r2: u8 = 23;
    println!("{}", r1.wrapping_add(r2));
}

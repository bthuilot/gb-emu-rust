extern crate cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::intrinsics::ceilf64;

const SAMPLE_RATE: usize = 44100;
const SAMPLE_PERIOD: f64 = 1_f64 / (SAMPLE_RATE as f64);


struct SoundCard {
    memory: [u8; 0x34],
    c_1: Channel,
    c_2: Channel,
    c_3: Channel,
    c_4: Channel,

    left_volume: f64,
    right_volume: f64,

    // TODO: Waveform
    waveform: [u8; 0x20]
}


impl SoundCard {
    pub fn new() -> SoundCard {
        let mut sc = SoundCard {
            memory: [0; 0x34],
            c_1: Channel::new(),
            c_2: Channel::new(),
            c_3: Channel::new(),
            c_4: Channel::new(),
            left_volume: 0.0,
            right_volume: 0.0,
            waveform: [0; 0x20],
        };

        sc.init();
        return sc;
    }

    fn init(&mut self) {
        for index in 0_usize..0x20 {
            if index % 1 == 0 {
                self.memory[index] = 0xFF
            }
        }
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find a default output device");
        let config = device.default_output_config()?;
        let stream = device.build_output_stream(
            config,
            move |data: &mut [T]| write_data(data, 4, &mut self.calculate_next_value),
            err_fn,
        )?;
        stream.play()?;

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    fn calculate_next_value(&mut self) {
        let samples: isize = 0;


    }
}


trait WaveGenerator {
    fn generate(t: f64) -> u8;
}

struct Channel {
    freq: f64,
    generator: f64,
    time: f64,
    amplitude: f64,

    duration: isize,

    envelope_time: isize,
    envelope_steps: isize,
    envelope_steps_init: isize,
    envelope_samples: isize,
    envelope_increasing: bool,

    sweep_time: f64,
    sweep_step_len: u8,
    sweep_steps: u8,
    sweep_step: u8,
    sweep_increase: bool,

    on: bool
}

impl Channel {
    pub fn new() -> Channel {
        return Channel {
            freq: 0.0,
            generator: 0.0,
            time: 0.0,
            amplitude: 0.0,
            duration: 0,
            envelope_time: 0,
            envelope_steps: 0,
            envelope_steps_init: 0,
            envelope_samples: 0,
            envelope_increasing: false,
            sweep_time: 0.0,
            sweep_step_len: 0,
            sweep_steps: 0,
            sweep_step: 0,
            sweep_increase: false,
            on: false
        }
    }
}
extern crate soundio;
use soundio::Context;
use std::process::exit;
use self::soundio::InStream;


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

    input_stream: InStream,

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
            input_stream: InStream{ userdata: Box::new(InStreamUserData {}), phantom: Default::default() },
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

        let mut ctx = soundio::Context::new();
        ctx.set_app_name("Gameboy Emulator");
        let result = ctx.connect();
        if result.is_err() {
            println!("Unable to open channel");
            exit(1);
        }

        let dev = ctx.default_input_device().expect("No input device");

        self.input_stream = dev.open_instream(
            SAMPLE_RATE as i32,
            soundio::Format::Float64BE,
            soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Stereo),
            2.0,
            read_callback,
            None::<fn()>,
            None::<fn(soundio::Error)>,
        )?;
    }

    fn read_callback(stream: &mut soundio::InStreamReader) {
        let frame_count_max = stream.frame_count_max();
        if let Err(e) = stream.begin_read(frame_count_max) {
            println!("Error reading from stream: {}", e);
            return;
        }

        for f in 0..stream.frame_count() {
            for c in 0..stream.channel_count() {
                do_something_with(stream.sample::<i16>(c, f));
            }
        }
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
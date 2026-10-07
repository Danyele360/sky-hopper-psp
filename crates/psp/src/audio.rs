use core::sync::atomic::{AtomicU32, Ordering};
use psp::audio::{AudioChannel, AudioFormat};

static EVENTS: AtomicU32 = AtomicU32::new(0);
static PLAYING: AtomicU32 = AtomicU32::new(0);
pub fn update(events: u32, playing: bool) {
    EVENTS.fetch_or(events, Ordering::Relaxed);
    PLAYING.store(playing as u32, Ordering::Relaxed);
}

#[derive(Clone, Copy)]
struct Voice {
    phase: f32,
    frequency: f32,
    slide: f32,
    life: f32,
    duration: f32,
    volume: f32,
}
impl Voice {
    const SILENT: Self = Self {
        phase: 0.0,
        frequency: 0.0,
        slide: 0.0,
        life: 0.0,
        duration: 1.0,
        volume: 0.0,
    };
    fn start(&mut self, f: f32, slide: f32, duration: f32, volume: f32) {
        *self = Self {
            phase: 0.0,
            frequency: f,
            slide,
            life: duration,
            duration,
            volume,
        };
    }
    fn sample(&mut self) -> f32 {
        if self.life <= 0.0 {
            return 0.0;
        }
        self.life -= 1.0 / 44100.0;
        self.frequency += self.slide / 44100.0;
        self.phase += self.frequency / 44100.0;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        let triangle = 1.0 - 4.0 * (self.phase - 0.5).abs();
        triangle * self.volume * (self.life / self.duration).max(0.0)
    }
}
pub fn run() -> i32 {
    let channel = match AudioChannel::reserve(512, AudioFormat::Mono) {
        Ok(c) => c,
        Err(_) => return 1,
    };
    let mut samples = [0i16; 512];
    let mut voices = [Voice::SILENT; 6];
    let notes = [
        261.63, 329.63, 392.0, 523.25, 392.0, 329.63, 293.66, 392.0, 246.94, 329.63, 392.0, 493.88,
        392.0, 329.63, 293.66, 246.94,
    ];
    let mut note = 0;
    let mut note_left = 0u32;
    loop {
        let events = EVENTS.swap(0, Ordering::Relaxed);
        if events & 1 != 0 {
            voices[0].start(360.0, 2700.0, 0.13, 1700.0);
        }
        if events & 2 != 0 {
            voices[1].start(1100.0, 3500.0, 0.07, 2300.0);
        }
        if events & 4 != 0 {
            voices[2].start(520.0, 2800.0, 0.25, 2400.0);
        }
        if events & 8 != 0 {
            voices[3].start(170.0, -430.0, 0.16, 2100.0);
        }
        if events & 16 != 0 {
            voices[4].start(340.0, -675.0, 0.4, 3200.0);
        }
        let playing = PLAYING.load(Ordering::Relaxed) != 0;
        for sample in &mut samples {
            if playing {
                if note_left == 0 {
                    voices[5].start(notes[note % 16], 0.0, 0.23, 700.0);
                    note += 1;
                    note_left = 10584;
                }
                note_left -= 1;
            } else {
                voices[5].life = 0.0;
                note_left = 0;
            }
            *sample = voices
                .iter_mut()
                .map(Voice::sample)
                .sum::<f32>()
                .clamp(-32767.0, 32767.0) as i16;
        }
        if channel.output_blocking(0x6000, &samples).is_err() {
            return 2;
        }
    }
}

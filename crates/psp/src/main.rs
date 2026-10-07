#![no_std]
#![no_main]

mod audio;

use psp::{
    config::{Config, ConfigValue},
    framebuffer::DoubleBuffer,
    input::Controller,
    sys::{CtrlButtons, DisplayPixelFormat},
    time::FrameTimer,
};
use sky_hopper_game::{
    Game, Progress, Screen, BACK, DASH, DOWN, DT, JUMP, LEFT, PAUSE, RIGHT, SAVE_KEYS, SAVE_WORDS,
    UP,
};

psp::module!("Sky Hopper PSP", 1, 1);
const SAVE: &str = "sky-hopper-save.bin";

fn psp_main() {
    if psp::callback::setup_exit_callback().is_err() {
        return;
    }
    psp::input::enable_analog();
    unsafe {
        psp::sys::scePowerSetClockFrequency(333, 333, 166);
    }
    let seed = psp::time::Instant::now().as_ticks() as u32;
    let mut game = Game::new(seed);
    if let Ok(config) = Config::load(SAVE) {
        let mut words = [0; SAVE_WORDS];
        for (i, key) in SAVE_KEYS.iter().enumerate() {
            words[i] = config.get_u32(key).unwrap_or(0);
        }
        game.progress = Progress::from_words(words);
    }
    let mut screen = Screen::new();
    let mut display = DoubleBuffer::new(DisplayPixelFormat::Psm8888, true);
    display.init();
    let mut controller = Controller::new();
    let mut timer = FrameTimer::new();
    let mut accumulator = 0.0f32;
    // Keep the handle alive: dropping a PSP JoinHandle terminates its thread.
    let _audio = psp::thread::spawn(b"sky_hopper_audio\0", audio::run).ok();
    loop {
        controller.update();
        let mut buttons = 0;
        for (psp_button, game_button) in [
            (CtrlButtons::CROSS, JUMP),
            (CtrlButtons::LEFT, LEFT),
            (CtrlButtons::RIGHT, RIGHT),
            (CtrlButtons::START, PAUSE),
            (CtrlButtons::LTRIGGER, DASH),
            (CtrlButtons::RTRIGGER, DASH),
            (CtrlButtons::CIRCLE, BACK),
            (CtrlButtons::UP, UP),
            (CtrlButtons::DOWN, DOWN),
        ] {
            if controller.is_held(psp_button) {
                buttons |= game_button;
            }
        }
        let analog = controller.analog_x_f32(0.25);
        if analog < -0.4 {
            buttons |= LEFT;
        } else if analog > 0.4 {
            buttons |= RIGHT;
        }
        accumulator += timer.tick().clamp(0.0, 0.1);
        let mut events = 0;
        while accumulator >= DT {
            game.step(buttons);
            events |= game.events;
            accumulator -= DT;
        }
        audio::update(events, game.mode == sky_hopper_game::Mode::Playing);
        screen.draw(&game);
        let out = display.draw_buffer();
        // RGBA byte order matches the PSP's ABGR little-endian PSM8888 layout.
        // Only visible pixels are copied; each VRAM row has 512 pixels of stride.
        unsafe {
            for row in 0..272 {
                core::ptr::copy_nonoverlapping(
                    screen.pixels.as_ptr().add(row * 480 * 4),
                    out.add(row * 512 * 4),
                    480 * 4,
                );
            }
        }
        display.swap();
        if game.dirty {
            let mut config = Config::new();
            for (i, key) in SAVE_KEYS.iter().enumerate() {
                config.set(key, ConfigValue::U32(game.progress.words()[i]));
            }
            if config.save(SAVE).is_ok() {
                game.dirty = false;
            }
        }
    }
}

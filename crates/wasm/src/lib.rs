use sky_hopper_game::{Game, Progress, Screen, SAVE_WORDS};
use std::cell::RefCell;

thread_local! {static STATE: RefCell<Option<(Game,Screen)>> = const {RefCell::new(None)};}
thread_local! {static LOAD: RefCell<[u32;SAVE_WORDS]> = const {RefCell::new([0;SAVE_WORDS])};}

#[no_mangle]
pub extern "C" fn init(seed: u32) {
    STATE.with(|s| *s.borrow_mut() = Some((Game::new(seed), Screen::new())));
}
#[no_mangle]
pub extern "C" fn tick(buttons: u32) {
    STATE.with(|s| {
        if let Some((g, _)) = s.borrow_mut().as_mut() {
            g.step(buttons);
        }
    });
}
#[no_mangle]
pub extern "C" fn render() -> *const u8 {
    STATE.with(|s| {
        let mut b = s.borrow_mut();
        let (g, screen) = b.as_mut().expect("init first");
        screen.draw(g);
        screen.pixels.as_ptr()
    })
}
#[no_mangle]
pub extern "C" fn stat(key: u32) -> u32 {
    STATE.with(|s| {
        let b = s.borrow();
        let (g, _) = b.as_ref().expect("init first");
        match key {
            0 => g.mode as u32,
            1 => g.score,
            2 => g.run_coins,
            3 => g.powers.mask() as u32,
            4 => g.events,
            5 => g.x as u32,
            6 => g.player_frame() as u32,
            7 => g.dirty as u32,
            8 => g.grounded as u32,
            9 => g.progress.language,
            80..=85 => (g.powers.remaining((key - 79) as u8) * 1000.0) as u32,
            90..=95 => g.powers.suspended((key - 89) as u8) as u32,
            10..=41 => g
                .progress
                .words()
                .get((key - 10) as usize)
                .copied()
                .unwrap_or(0),
            _ => 0,
        }
    })
}
#[no_mangle]
pub extern "C" fn progress_len() -> u32 {
    SAVE_WORDS as u32
}
#[no_mangle]
pub extern "C" fn load_word(index: u32, value: u32) {
    LOAD.with(|buffer| {
        if let Some(word) = buffer.borrow_mut().get_mut(index as usize) {
            *word = value;
        }
    });
}
#[no_mangle]
pub extern "C" fn commit_progress() {
    LOAD.with(|buffer| {
        STATE.with(|state| {
            if let Some((g, _)) = state.borrow_mut().as_mut() {
                g.progress = Progress::from_words(*buffer.borrow());
            }
        })
    });
}
#[no_mangle]
pub extern "C" fn set_language(language: u32) {
    STATE.with(|state| {
        if let Some((g, _)) = state.borrow_mut().as_mut() {
            g.progress.language = language.min(1);
            g.dirty = true;
        }
    });
}
#[no_mangle]
pub extern "C" fn load_progress(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) {
    STATE.with(|s| {
        if let Some((g, _)) = s.borrow_mut().as_mut() {
            g.progress = Progress::from_words([a, b, c, d, e, f]);
        }
    });
}
#[no_mangle]
pub extern "C" fn mark_saved() {
    STATE.with(|s| {
        if let Some((g, _)) = s.borrow_mut().as_mut() {
            g.dirty = false;
        }
    });
}

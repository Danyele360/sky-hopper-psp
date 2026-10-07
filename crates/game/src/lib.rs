#![no_std]
extern crate alloc;

mod powers;
mod progression;
mod render;
use alloc::vec::Vec;
use libm::sinf;
pub use powers::{PowerState, POWER_LABELS, POWER_NAMES};
pub use progression::{tr, Progress, ITEMS, SAVE_KEYS, SAVE_WORDS, WORLD_ZOOM};
use progression::{DURATION_STEP, JET_DURATION_STEP};
pub use render::{Screen, HEIGHT, WIDTH};

pub const JUMP: u32 = 1;
pub const LEFT: u32 = 2;
pub const RIGHT: u32 = 4;
pub const PAUSE: u32 = 8;
pub const DASH: u32 = 16;
pub const BACK: u32 = 32;
pub const UP: u32 = 64;
pub const DOWN: u32 = 128;
pub const DT: f32 = 1.0 / 60.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode {
    Menu,
    Playing,
    Paused,
    GameOver,
    Shop,
    Help,
    Journal,
}

#[derive(Clone, Copy, Debug)]
pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub kind: u8,
    pub phase: f32,
    pub hazard: bool,
    pub enemy: bool,
    pub power: u8,
    pub picked: bool,
}
impl Platform {
    pub fn top(self, x: f32, time: f32) -> f32 {
        self.y
            + if self.kind == 1 {
                sinf(time * 1.4 + self.phase) * 8.0
            } else if self.kind == 2 {
                -((x - self.x) / self.width).clamp(0.0, 1.0) * 22.0
            } else {
                0.0
            }
    }
    pub fn end_top(self) -> f32 {
        self.y - if self.kind == 2 { 22.0 } else { 0.0 }
    }
}
#[derive(Clone, Copy)]
pub struct Coin {
    pub x: f32,
    pub y: f32,
    pub taken: bool,
}
#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub color: [u8; 3],
}

pub struct Game {
    pub mode: Mode,
    pub progress: Progress,
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub cam_x: f32,
    pub cam_y: f32,
    pub platforms: Vec<Platform>,
    pub coins: Vec<Coin>,
    pub particles: Vec<Particle>,
    pub time: f32,
    pub world_time: f32,
    pub grounded: bool,
    pub jumps: u8,
    pub powers: PowerState,
    pub run_coins: u32,
    pub run_pickups: u32,
    pub earned_coins: u32,
    pub mission_coins: u32,
    pub completed_goals: u8,
    pub score: u32,
    pub invincible: f32,
    pub dash_time: f32,
    pub dash_cooldown: f32,
    pub pose_time: f32,
    pub death_time: f32,
    pub menu: usize,
    pub shop: usize,
    pub store_notice: u8,
    pub dirty: bool,
    pub events: u32,
    previous: u32,
    rng: u32,
    seed: u32,
    next_id: u32,
    coyote: f32,
    jump_buffer: f32,
    pose: u8,
}
impl Game {
    pub fn new(seed: u32) -> Self {
        let mut game = Self {
            mode: Mode::Menu,
            progress: Progress::default(),
            x: 60.0,
            y: 182.0,
            vy: 0.0,
            cam_x: 0.0,
            cam_y: 0.0,
            platforms: Vec::with_capacity(16),
            coins: Vec::with_capacity(64),
            particles: Vec::with_capacity(80),
            time: 0.0,
            world_time: 0.0,
            grounded: true,
            jumps: 0,
            powers: PowerState::default(),
            run_coins: 0,
            run_pickups: 0,
            earned_coins: 0,
            mission_coins: 0,
            completed_goals: 0,
            score: 0,
            invincible: 0.0,
            dash_time: 0.0,
            dash_cooldown: 0.0,
            pose_time: 0.0,
            death_time: 0.0,
            menu: 0,
            shop: 0,
            store_notice: 0,
            dirty: false,
            events: 0,
            previous: 0,
            rng: seed.max(1),
            seed: seed.max(1),
            next_id: 0,
            coyote: 0.1,
            jump_buffer: 0.0,
            pose: 8,
        };
        game.reset();
        game.mode = Mode::Menu;
        game
    }
    fn rand(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        (self.rng >> 8) as f32 / 16777216.0
    }
    pub fn reset(&mut self) {
        self.platforms.clear();
        self.coins.clear();
        self.particles.clear();
        self.platforms.push(Platform {
            x: 0.0,
            y: 210.0,
            width: 240.0,
            kind: 0,
            phase: 0.0,
            hazard: false,
            enemy: false,
            power: 0,
            picked: false,
        });
        self.x = 60.0;
        self.y = 182.0;
        self.vy = 0.0;
        self.cam_x = 0.0;
        self.cam_y = 0.0;
        self.grounded = true;
        self.jumps = 0;
        self.run_coins = 0;
        self.run_pickups = 0;
        self.earned_coins = 0;
        self.mission_coins = 0;
        self.completed_goals = 0;
        self.score = 0;
        self.powers = PowerState::default();
        self.invincible = 0.0;
        self.dash_time = 0.0;
        self.dash_cooldown = 0.0;
        self.world_time = 0.0;
        self.coyote = 0.1;
        self.jump_buffer = 0.0;
        self.pose_time = 0.0;
        self.death_time = 0.0;
        self.next_id = 0;
        self.rng = (self.seed ^ self.progress.runs.wrapping_mul(0x9e3779b9)).max(1);
        self.mode = Mode::Playing;
        self.generate();
        let capsule = self.progress.begin_run();
        if capsule != 0 {
            self.activate_power(capsule);
            self.dirty = true;
        }
    }
    fn generate(&mut self) {
        while self.platforms.last().unwrap().x < self.x + 760.0 {
            let prev = *self.platforms.last().unwrap();
            self.next_id += 1;
            let difficulty = (self.next_id as f32 / 70.0).min(1.0);
            let gap = 30.0 + self.rand() * (24.0 + 18.0 * difficulty);
            let width = 120.0 + self.rand() * 45.0 - difficulty * 25.0;
            let rise = 12.0 + self.rand() * 16.0;
            let kind = if self.next_id % 7 == 0 {
                1
            } else if self.next_id % 9 == 0 {
                2
            } else if self.next_id % 11 == 0 {
                3
            } else {
                0
            };
            let power = if self.next_id % 3 == 1 {
                [1, 2, 3, 4, 6, 1, 2, 5][(self.rand() * 8.0) as usize]
            } else {
                0
            };
            let p = Platform {
                x: prev.x + prev.width + gap,
                y: prev.end_top() - rise,
                width,
                kind,
                phase: self.rand() * 6.28,
                hazard: self.next_id > 5 && self.next_id % 4 == 0,
                enemy: self.next_id > 8 && self.next_id % 6 == 0,
                power,
                picked: false,
            };
            for k in 0..4 {
                let cx = p.x + 18.0 + k as f32 * 24.0;
                let cy = p.top(cx, 0.0) - 30.0 - sinf(k as f32 * 1.04) * 14.0;
                self.coins.push(Coin {
                    x: cx,
                    y: cy,
                    taken: false,
                });
            }
            self.platforms.push(p);
        }
    }
    pub fn player_frame(&self) -> usize {
        if self.mode == Mode::GameOver {
            return if self.death_time < 0.3 { 11 } else { 15 };
        }
        if self.mode != Mode::Playing && self.mode != Mode::Paused {
            return if (self.time * 0.7) as u32 % 5 == 0 {
                9
            } else {
                8
            };
        }
        if self.invincible > 0.0 && self.pose == 11 && self.pose_time > 0.0 {
            11
        } else if self.powers.effective(5) {
            12
        } else if self.dash_time > 0.0 || self.powers.effective(3) {
            13
        } else if self.pose_time > 0.0 {
            self.pose as usize
        } else if self.grounded {
            (self.time * 12.0) as usize % 4
        } else if self.vy < 0.0 {
            5
        } else {
            7
        }
    }
    fn burst(&mut self, x: f32, y: f32, color: [u8; 3], count: usize) {
        for _ in 0..count {
            if self.particles.len() >= 80 {
                break;
            }
            let vx = (self.rand() - 0.5) * 120.0;
            let vy = (self.rand() - 0.7) * 100.0;
            let life = 0.3 + self.rand() * 0.3;
            self.particles.push(Particle {
                x,
                y,
                vx,
                vy,
                life,
                color,
            });
        }
    }
    pub fn activate_power(&mut self, power: u8) {
        if !(1..=6).contains(&power) {
            return;
        }
        let was_flying = self.powers.effective(5);
        let extra = self.progress.levels[power as usize - 1] as f32
            * if power == 5 {
                JET_DURATION_STEP
            } else {
                DURATION_STEP
            };
        self.powers
            .collect_extended(power, self.progress.upgraded > 0, extra);
        let flying = self.powers.effective(5);
        if was_flying && !flying {
            self.leave_flight();
        }
        if flying && !was_flying {
            self.vy = -80.0;
            self.grounded = false;
            self.jumps = 1;
            self.jump_buffer = 0.0;
        }
        self.events |= 4;
        self.burst(self.x + 12.0, self.y + 14.0, [94, 235, 255], 12);
    }
    fn leave_flight(&mut self) {
        self.vy = -200.0;
        self.invincible = self.invincible.max(2.0);
        self.grounded = false;
        self.jumps = 1;
        self.jump_buffer = 0.0;
    }
    pub fn hit(&mut self) {
        if self.invincible > 0.0 || self.dash_time > 0.0 || self.powers.effective(3) {
            return;
        }
        if self.powers.consume(2) {
            self.invincible = 1.5;
            self.pose = 11;
            self.pose_time = 0.25;
            self.events |= 8;
            self.burst(self.x + 12.0, self.y + 14.0, [140, 244, 255], 20);
        } else {
            self.finish();
        }
    }
    fn finish(&mut self) {
        if self.mode != Mode::Playing {
            return;
        }
        self.mode = Mode::GameOver;
        self.death_time = 0.0;
        (self.earned_coins, self.mission_coins, self.completed_goals) = self.progress.bank_run(
            (self.x - 60.0).max(0.0) as u32,
            self.run_coins,
            self.run_pickups,
            self.score,
        );
        self.dirty = true;
        self.events |= 16;
        self.burst(self.x + 12.0, self.y + 14.0, [255, 163, 109], 22);
    }
    fn buy(&mut self) {
        self.store_notice = self.progress.purchase(self.shop);
        if matches!(self.store_notice, 1 | 4 | 5) {
            self.dirty = true;
            self.events |= 4;
        } else {
            self.events |= 8;
        }
    }
    pub fn toggle_language(&mut self) {
        self.progress.language = 1 - self.progress.language;
        self.dirty = true;
    }
    pub fn step(&mut self, buttons: u32) {
        self.events = 0;
        let pressed = buttons & !self.previous;
        self.previous = buttons;
        if self.mode == Mode::Paused {
            if pressed & PAUSE != 0 {
                self.mode = Mode::Playing;
            }
            if pressed & BACK != 0 {
                self.finish_from_pause();
            }
            return;
        }
        if self.mode == Mode::Playing && pressed & PAUSE != 0 {
            self.mode = Mode::Paused;
            return;
        }
        self.time += DT;
        for p in &mut self.particles {
            p.x += p.vx * DT;
            p.y += p.vy * DT;
            p.vy += 120.0 * DT;
            p.life -= DT;
        }
        self.particles.retain(|p| p.life > 0.0);
        match self.mode {
            Mode::Menu => {
                if pressed & (DOWN | RIGHT) != 0 {
                    self.menu = (self.menu + 1) % 5;
                }
                if pressed & (UP | LEFT) != 0 {
                    self.menu = (self.menu + 4) % 5;
                }
                if pressed & (JUMP | PAUSE) != 0 {
                    match self.menu {
                        0 => self.reset(),
                        1 => self.mode = Mode::Shop,
                        2 => self.mode = Mode::Journal,
                        3 => self.mode = Mode::Help,
                        _ => self.toggle_language(),
                    }
                }
                return;
            }
            Mode::Shop => {
                let category = ITEMS[self.shop].category;
                let (start, end) = [(0, 9), (9, 12), (12, 15)][category];
                if pressed & DOWN != 0 {
                    self.shop = start + (self.shop - start + 1) % (end - start);
                    self.store_notice = 0;
                }
                if pressed & UP != 0 {
                    self.shop = start + (self.shop - start + end - start - 1) % (end - start);
                    self.store_notice = 0;
                }
                if pressed & RIGHT != 0 {
                    self.shop = [0, 9, 12][(category + 1) % 3];
                    self.store_notice = 0;
                }
                if pressed & LEFT != 0 {
                    self.shop = [0, 9, 12][(category + 2) % 3];
                    self.store_notice = 0;
                }
                if pressed & DASH != 0 && ITEMS[self.shop].kind == 2 {
                    if self.progress.arm(ITEMS[self.shop].index) {
                        self.dirty = true;
                        self.store_notice = if self.progress.launch == 0 { 5 } else { 4 };
                    } else {
                        self.store_notice = 3;
                    }
                }
                if pressed & JUMP != 0 {
                    self.buy();
                }
                if pressed & BACK != 0 {
                    self.mode = Mode::Menu;
                }
                return;
            }
            Mode::Help | Mode::Journal => {
                if pressed & (BACK | JUMP) != 0 {
                    self.mode = Mode::Menu;
                }
                return;
            }
            Mode::GameOver => {
                self.death_time += DT;
                if self.death_time > 0.35 {
                    if pressed & (JUMP | PAUSE) != 0 {
                        self.reset();
                    }
                    if pressed & BACK != 0 {
                        self.mode = Mode::Menu;
                    }
                }
                return;
            }
            _ => {}
        }
        self.world_time += DT * if self.powers.effective(6) { 0.35 } else { 1.0 };
        self.invincible = (self.invincible - DT).max(0.0);
        self.pose_time = (self.pose_time - DT).max(0.0);
        self.dash_time = (self.dash_time - DT).max(0.0);
        self.dash_cooldown = (self.dash_cooldown - DT).max(0.0);
        if self.powers.tick(DT) & (1 << 4) != 0 {
            self.leave_flight();
        }
        if pressed & DASH != 0 && self.dash_cooldown <= 0.0 {
            self.dash_time = 0.4;
            self.dash_cooldown = self.progress.dash_seconds();
            if self.grounded {
                self.vy = -260.0;
                self.grounded = false;
                self.jumps = 1;
            }
            self.events |= 4;
        }
        if pressed & JUMP != 0 {
            self.jump_buffer = 0.12;
        }
        if self.grounded {
            self.coyote = 0.1;
        } else {
            self.coyote = (self.coyote - DT).max(0.0);
        }
        if self.jump_buffer > 0.0 && !self.powers.effective(5) {
            if self.grounded || self.coyote > 0.0 || self.jumps < 2 {
                let second = !self.grounded && self.coyote <= 0.0;
                self.vy = if self.powers.effective(4) {
                    -450.0
                } else {
                    -325.0
                };
                self.jumps = if second { 2 } else { 1 };
                self.grounded = false;
                self.coyote = 0.0;
                self.jump_buffer = 0.0;
                self.pose = if second { 6 } else { 4 };
                self.pose_time = if second { 0.14 } else { 0.07 };
                self.burst(
                    self.x + 12.0,
                    self.y + 28.0,
                    [115, 232, 255],
                    if second { 10 } else { 6 },
                );
                self.events |= 1;
            } else {
                self.jump_buffer -= DT;
            }
        }
        let speed = 132.0
            + (self.x / 200.0).min(38.0)
            + if buttons & LEFT != 0 {
                -42.0
            } else if buttons & RIGHT != 0 {
                38.0
            } else {
                0.0
            }
            + if self.powers.effective(3) {
                70.0
            } else if self.dash_time > 0.0 {
                95.0
            } else {
                0.0
            };
        if self.powers.effective(3) && self.grounded {
            let edge = self
                .platforms
                .iter()
                .find(|p| self.x + 12.0 >= p.x && self.x + 12.0 <= p.x + p.width)
                .map(|p| p.x + p.width - self.x);
            if edge.is_some_and(|e| e < 38.0) {
                self.vy = -340.0;
                self.grounded = false;
                self.jumps = 1;
            }
        }
        let old_feet = self.y + 28.0;
        self.x += speed * DT;
        if self.powers.effective(5) {
            self.vy = if buttons & (JUMP | UP) != 0 {
                -160.0
            } else if buttons & DOWN != 0 {
                170.0
            } else {
                -18.0
            };
            self.vy = self.vy.clamp(-180.0, 180.0);
            // Flight stays within the traversable corridor, avoiding unseen collision surfaces.
            if let Some(p) = self.platforms.iter().find(|p| p.x + p.width > self.x) {
                self.y = self.y.max(p.y - 180.0);
            }
        } else {
            self.vy = (self.vy + 780.0 * DT).min(650.0);
        }
        self.y += self.vy * DT;
        self.grounded = false;
        let mut spring = false;
        if self.vy >= 0.0 {
            for p in &self.platforms {
                if self.x + 22.0 > p.x && self.x + 2.0 < p.x + p.width {
                    let top = p.top(self.x + 12.0, self.world_time);
                    let previous_top = p.top(self.x + 12.0 - speed * DT, self.world_time - DT);
                    if old_feet <= previous_top + 4.0 && self.y + 28.0 >= top {
                        self.y = top - 28.0;
                        self.vy = 0.0;
                        self.grounded = true;
                        self.jumps = 0;
                        if old_feet < top - 2.0 {
                            self.pose = 10;
                            self.pose_time = 0.1;
                            self.events |= 32;
                        }
                        if p.kind == 3 && (self.x + 12.0 - (p.x + p.width * 0.60)).abs() < 21.0 {
                            spring = true;
                        }
                        break;
                    }
                }
            }
        }
        if spring {
            self.vy = -465.0;
            self.grounded = false;
            self.jumps = 1;
            self.pose = 6;
            self.pose_time = 0.15;
            self.events |= 1;
            self.burst(self.x + 12.0, self.y + 28.0, [255, 174, 87], 14);
        }
        let mut got = 0;
        let radius = self.progress.magnet_radius();
        for c in &mut self.coins {
            if c.taken {
                continue;
            }
            let dx = self.x + 12.0 - c.x;
            let dy = self.y + 14.0 - c.y;
            if self.powers.effective(1) && dx * dx + dy * dy < radius * radius {
                c.x += dx * 8.0 * DT;
                c.y += dy * 8.0 * DT;
            }
            if dx * dx + dy * dy < 23.0 * 23.0 {
                c.taken = true;
                got += 1;
            }
        }
        if got > 0 {
            self.run_coins += got;
            self.events |= 2;
            self.burst(
                self.x + 12.0,
                self.y + 10.0,
                [255, 218, 80],
                got as usize * 3,
            );
        }
        let mut pickups = 0u8;
        let mut collision = false;
        for p in &mut self.platforms {
            let px = p.x + p.width * 0.56;
            let py = p.top(px, self.world_time);
            if (1..=6).contains(&p.power)
                && !p.picked
                && (self.x + 12.0 - px).abs() < 26.0
                && (self.y + 14.0 - (py - 44.0)).abs() < 34.0
            {
                pickups |= 1 << (p.power - 1);
                self.run_pickups += 1;
                p.picked = true;
            }
            if p.hazard
                && (self.x + 12.0 - (p.x + p.width - 27.0)).abs() < 22.0
                && self.y + 28.0 > py - 14.0
                && self.y < py
            {
                collision = true;
            }
            if p.enemy {
                let ex = p.x + p.width * 0.5 + sinf(self.world_time * 2.0 + p.phase) * 22.0;
                let ey = py - 58.0 + sinf(self.world_time * 3.0 + p.phase) * 10.0;
                if (self.x + 12.0 - ex).abs() < 23.0 && (self.y + 14.0 - ey).abs() < 22.0 {
                    collision = true;
                }
            }
        }
        for power in 1..=6 {
            if pickups & (1 << (power - 1)) != 0 {
                self.activate_power(power);
            }
        }
        if collision {
            self.hit();
        }
        self.score = ((self.x - 60.0).max(0.0) / 3.0) as u32 + self.run_coins * 25;
        self.cam_x = (self.x - 115.0).max(0.0);
        // Follow the route's height, avoiding camera bob on every jump and a camera that follows falls.
        let route = self
            .platforms
            .iter()
            .find(|p| p.x + p.width > self.x)
            .map_or(self.cam_y + 210.0, |p| p.y);
        let target = route - 210.0;
        self.cam_y += (target - self.cam_y) * 0.035;
        if self.y - self.cam_y > HEIGHT as f32 + 70.0 {
            self.finish();
        }
        if self.mode == Mode::Playing {
            self.generate();
            self.platforms
                .retain(|p| p.x + p.width > self.cam_x - 100.0);
            self.coins.retain(|c| c.x > self.cam_x - 100.0);
            if (self.time * 60.0) as u32 % 3 == 0
                && (self.dash_time > 0.0 || self.powers.effective(3) || self.powers.effective(5))
            {
                self.burst(
                    self.x - 3.0,
                    self.y + 20.0,
                    if self.powers.effective(5) {
                        [255, 157, 80]
                    } else {
                        [104, 224, 255]
                    },
                    2,
                );
            }
        }
    }
    fn finish_from_pause(&mut self) {
        self.mode = Mode::Playing;
        self.finish();
        self.mode = Mode::Menu;
    }
}

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn route_is_horizontal_and_reachable() {
        for seed in 1..80 {
            let mut g = Game::new(seed);
            for _ in 0..120 {
                for w in g.platforms.windows(2) {
                    let gap = w[1].x - w[0].x - w[0].width;
                    assert!((29.9..=72.1).contains(&gap));
                    let rise = w[0].end_top() - w[1].y;
                    assert!((11.9..=28.1).contains(&rise));
                    assert!(gap / (132.0 + 38.0) < 0.55);
                }
                g.x = g.platforms.last().unwrap().x;
                g.generate();
                g.platforms.drain(..4);
            }
        }
    }
    #[test]
    fn pause_freezes_everything() {
        let mut g = Game::new(42);
        g.reset();
        g.activate_power(1);
        g.activate_power(2);
        g.step(PAUSE);
        let state = (g.x, g.y, g.time, g.world_time, g.powers);
        for _ in 0..180 {
            g.step(0);
        }
        assert_eq!(state, (g.x, g.y, g.time, g.world_time, g.powers));
        g.step(PAUSE);
        assert_eq!(g.mode, Mode::Playing);
    }
    #[test]
    fn jump_has_edge_detection_and_limit() {
        let mut g = Game::new(42);
        g.reset();
        g.step(JUMP);
        assert_eq!(g.jumps, 1);
        for _ in 0..5 {
            g.step(JUMP);
        }
        assert_eq!(g.jumps, 1);
        g.step(0);
        g.step(JUMP);
        assert_eq!(g.jumps, 2);
        g.step(0);
        g.step(JUMP);
        assert_eq!(g.jumps, 2);
    }
    #[test]
    fn shield_absorbs_exactly_one_hit() {
        let mut g = Game::new(1);
        g.reset();
        g.activate_power(1);
        g.activate_power(4);
        g.activate_power(2);
        g.hit();
        assert_eq!(g.mode, Mode::Playing);
        assert!(!g.powers.has(2));
        assert!(g.powers.has(1));
        assert!(g.powers.has(4));
        g.invincible = 0.0;
        g.hit();
        assert_eq!(g.mode, Mode::GameOver);
    }
    #[test]
    fn death_banks_coins_once() {
        let mut g = Game::new(1);
        g.reset();
        g.run_coins = 7;
        g.score = 200;
        g.hit();
        g.hit();
        assert_eq!(g.progress.bank, 7);
        assert_eq!(g.progress.best, 200);
        assert!(g.dirty);
    }
    #[test]
    fn magnet_collects_distant_coin() {
        let mut g = Game::new(1);
        g.reset();
        g.coins.clear();
        g.coins.push(Coin {
            x: 125.0,
            y: 160.0,
            taken: false,
        });
        g.activate_power(1);
        for _ in 0..22 {
            g.step(LEFT);
        }
        assert_eq!(g.run_coins, 1);
    }
    #[test]
    fn purchases_preserve_bank_and_unlocks() {
        let mut g = Game::new(1);
        g.progress.bank = 200;
        g.shop = 12;
        g.buy();
        assert_eq!(g.progress.bank, 140);
        assert_eq!(g.progress.skin, 1);
        g.buy();
        assert_eq!(g.progress.bank, 140);
        assert_eq!(g.progress.skin, 0);
        g.shop = 14;
        g.buy();
        assert_eq!(g.progress.bank, 140);
        assert_eq!(g.progress.world, 0);
    }
    #[test]
    fn flight_expiry_gives_safe_transition() {
        let mut g = Game::new(1);
        g.reset();
        g.activate_power(5);
        g.powers.set_remaining(5, 0.005);
        g.step(0);
        assert!(!g.powers.has(5));
        assert!(g.vy < 0.0);
        assert!(g.invincible > 1.9);
    }
    #[test]
    fn compatible_pickups_and_jetpack_refresh_do_not_restart_flight() {
        let mut g = Game::new(1);
        g.reset();
        g.activate_power(5);
        g.vy = 27.0;
        g.jumps = 2;
        for power in [1, 2, 4, 6, 5] {
            g.activate_power(power);
            assert_eq!(g.vy, 27.0);
            assert_eq!(g.jumps, 2);
        }
        assert_eq!(g.powers.mask(), 0b11_1011);
        assert!(g.powers.suspended(4));
        g.activate_power(3);
        assert!(!g.powers.has(5));
        assert!(g.powers.effective(4));
        assert!(g.vy < 0.0 && g.invincible >= 2.0);
        for power in [1, 2, 3, 4, 6] {
            assert!(g.powers.has(power));
        }
    }
    #[test]
    fn boost_protects_without_using_the_shield() {
        let mut g = Game::new(1);
        g.reset();
        for power in [1, 2, 3, 4, 6] {
            g.activate_power(power);
        }
        let before = g.powers;
        g.hit();
        assert_eq!(g.powers, before);
        assert_eq!(g.mode, Mode::Playing);
        g.reset();
        assert_eq!(g.powers.mask(), 0);
    }
    #[test]
    fn restart_varies_route_and_consumes_only_the_equipped_capsule() {
        let mut g = Game::new(42);
        g.reset();
        let previous = g.platforms[1].x;
        g.progress.stock = [2, 1, 0];
        g.progress.launch = 2;
        g.finish();
        g.reset();
        assert_ne!(g.platforms[1].x, previous);
        assert_eq!(g.progress.stock, [2, 0, 0]);
        assert!(g.powers.has(2));
        assert_eq!(g.run_pickups, 0);
        assert!(g.dirty);
    }
    #[test]
    fn language_and_individual_upgrade_work_without_clearing_other_bonuses() {
        let mut g = Game::new(42);
        g.reset();
        g.activate_power(2);
        g.progress.levels[0] = 3;
        g.activate_power(1);
        assert_eq!(g.powers.remaining(1), 11.0);
        assert_eq!(g.powers.remaining(2), 8.0);
        g.toggle_language();
        assert_eq!(g.progress.language, 1);
        g.toggle_language();
        assert_eq!(g.progress.language, 0);
        assert!(g.powers.has(1) && g.powers.has(2));
    }
    #[test]
    fn save_validation_rejects_locked_selections() {
        let p = Progress::from_words([1, 2, 0, 2, 1, 99]);
        assert_eq!(p.skin, 0);
        assert_eq!(p.world, 0);
        assert_eq!(p.upgraded, 1);
    }
    #[test]
    fn autoplay_traverses_many_platforms() {
        let mut g = Game::new(42);
        g.reset();
        for _ in 0..3600 {
            let edge = g
                .platforms
                .iter()
                .find(|p| g.x + 12.0 >= p.x && g.x + 12.0 <= p.x + p.width)
                .map(|p| p.x + p.width - g.x);
            let buttons = if g.grounded && edge.is_some_and(|e| e < 56.0) {
                JUMP
            } else {
                0
            };
            // Collision-free traversal checks the generated route, independently of hazards.
            for p in &mut g.platforms {
                p.hazard = false;
                p.enemy = false;
                p.power = 0;
            }
            g.step(buttons);
            assert_eq!(g.mode, Mode::Playing, "fell at x={}, y={}", g.x, g.y);
        }
        assert!(g.x > 8000.0);
        assert!(g.cam_y < -700.0);
        assert!(g.platforms.len() < 16 && g.coins.len() < 64 && g.particles.len() <= 80);
    }
}

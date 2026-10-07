use super::{tr, Game, Mode, ITEMS, WORLD_ZOOM};
use crate::progression::{COIN_YIELD_PERCENT, DURATION_STEP, JET_DURATION_STEP};
use alloc::{format, vec, vec::Vec};
use libm::sinf;

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 272;
const SKY: &[u8] = include_bytes!("../../../assets/runtime/sky.rgba");
const ROBOT: &[u8] = include_bytes!("../../../assets/runtime/robot.rgba");
const WORLD: &[u8] = include_bytes!("../../../assets/runtime/world.rgba");
const FONT: &[u8] = include_bytes!("../../../assets/runtime/font.bin");
const WHITE: [u8; 4] = [236, 249, 255, 255];
const CYAN: [u8; 4] = [91, 223, 255, 255];
const GOLD: [u8; 4] = [255, 221, 100, 255];

pub struct Screen {
    pub pixels: Vec<u8>,
    world_view: bool,
}
impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT * 4],
            world_view: false,
        }
    }
    fn pixel(&mut self, x: i32, y: i32, c: [u8; 4]) {
        if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
            return;
        }
        let i = (y as usize * WIDTH + x as usize) * 4;
        if c[3] == 255 {
            self.pixels[i..i + 4].copy_from_slice(&c);
        } else if c[3] > 0 {
            let a = c[3] as u32;
            let b = 255 - a;
            for (k, v) in c.iter().enumerate().take(3) {
                self.pixels[i + k] = ((*v as u32 * a + self.pixels[i + k] as u32 * b) / 255) as u8;
            }
            self.pixels[i + 3] = 255;
        }
    }
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: [u8; 4]) {
        let (x, y, w, h) = self.transform(x, y, w, h);
        for py in y.max(0)..(y + h).min(HEIGHT as i32) {
            for px in x.max(0)..(x + w).min(WIDTH as i32) {
                self.pixel(px, py, c);
            }
        }
    }
    fn transform(&self, x: i32, y: i32, w: i32, h: i32) -> (i32, i32, i32, i32) {
        if self.world_view {
            (
                (115.0 + (x as f32 - 115.0) * WORLD_ZOOM) as i32,
                (210.0 + (y as f32 - 210.0) * WORLD_ZOOM) as i32,
                (w as f32 * WORLD_ZOOM).max(1.0) as i32,
                (h as f32 * WORLD_ZOOM).max(1.0) as i32,
            )
        } else {
            (x, y, w, h)
        }
    }
    fn panel(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.rect(x, y, w, h, [4, 15, 37, 224]);
        self.rect(x, y, w, 1, [80, 190, 240, 210]);
        self.rect(x, y + h - 1, w, 1, [39, 108, 165, 210]);
        self.rect(x, y, 1, h, [54, 148, 209, 210]);
        self.rect(x + w - 1, y, 1, h, [54, 148, 209, 210]);
    }
    pub fn text(&mut self, x: i32, y: i32, s: &str, scale: i32, color: [u8; 4]) {
        let mut px = x;
        for ch in s.bytes() {
            let ch = ch.to_ascii_uppercase() as usize;
            if ch < 128 {
                for row in 0..7 {
                    let bits = FONT[ch * 7 + row];
                    for col in 0..5 {
                        if bits & (1 << (4 - col)) != 0 {
                            self.rect(
                                px + col * scale,
                                y + row as i32 * scale,
                                scale,
                                scale,
                                color,
                            );
                        }
                    }
                }
            }
            px += 6 * scale;
        }
    }
    fn center(&mut self, y: i32, s: &str, scale: i32, color: [u8; 4]) {
        self.text(
            (WIDTH as i32 - s.len() as i32 * 6 * scale) / 2,
            y,
            s,
            scale,
            color,
        );
    }
    fn sprite(&mut self, robot: bool, index: usize, x: i32, y: i32, w: i32, h: i32, skin: u32) {
        if w <= 0 || h <= 0 {
            return;
        }
        let (x, y, w, h) = self.transform(x, y, w, h);
        let (data, cell) = if robot { (ROBOT, 48) } else { (WORLD, 64) };
        let stride = cell * 4;
        let ox = index % 4 * cell;
        let oy = index / 4 * cell;
        for py in y.max(0)..(y + h).min(HEIGHT as i32) {
            let sy = (py - y) as usize * cell / h as usize + oy;
            for px in x.max(0)..(x + w).min(WIDTH as i32) {
                let sx = (px - x) as usize * cell / w as usize + ox;
                let i = (sy * stride + sx) * 4;
                let mut c = [data[i], data[i + 1], data[i + 2], data[i + 3]];
                if skin == 1 && c[2] > c[0] {
                    c[0] = c[0].saturating_add(80);
                    c[1] = (c[1] as u16 * 3 / 4) as u8;
                }
                if skin == 2 && c[2] > c[0] {
                    c[0] = c[0].saturating_add(120);
                    c[1] = c[1].saturating_add(30);
                    c[2] = (c[2] as u16 / 2) as u8;
                }
                self.pixel(px, py, c);
            }
        }
    }
    fn ring(&mut self, cx: i32, cy: i32, r: i32, color: [u8; 4]) {
        let (cx, cy, r, _) = self.transform(cx, cy, r, r);
        for y in -r..=r {
            for x in -r..=r {
                let d = x * x + y * y;
                if d <= r * r && d >= (r - 2) * (r - 2) {
                    self.pixel(cx + x, cy + y, color);
                }
            }
        }
    }
    fn background(&mut self, g: &Game) {
        let shift = ((g.cam_x * 0.06 + g.time * 0.35) as usize) % 960;
        for y in 0..HEIGHT {
            let dst = y * WIDTH * 4;
            // Ping-pong sampling keeps the edges of this single generated painting continuous.
            for x in 0..WIDTH {
                let sx = (x + shift) % 960;
                let sx = if sx >= 480 { 959 - sx } else { sx };
                let src = (y * WIDTH + sx) * 4;
                self.pixels[dst + x * 4..dst + x * 4 + 4].copy_from_slice(&SKY[src..src + 4]);
                if g.progress.world == 1 {
                    let i = dst + x * 4;
                    self.pixels[i] =
                        (self.pixels[i] as u16 / 2 + self.pixels[i + 2] as u16 / 3).min(255) as u8;
                    self.pixels[i + 1] = (self.pixels[i + 1] as u16 * 2 / 3) as u8;
                }
            }
        }
        for i in 0..9 {
            let raw = (i * 67 + 431) as f32 - g.cam_x * 0.14;
            let x = raw - libm::floorf(raw / 560.0) * 560.0 - 40.0;
            let y = 30.0 + (i % 3) as f32 * 32.0 + sinf(g.time + i as f32) * 3.0;
            let a = (120.0 + sinf(g.time * 2.0 + i as f32) * 100.0) as u8;
            self.rect(x as i32, y as i32, 2, 2, [194, 237, 255, a]);
        }
    }
    pub fn draw(&mut self, g: &Game) {
        self.world_view = false;
        self.background(g);
        self.world_view = true;
        for p in &g.platforms {
            let x = (p.x - g.cam_x) as i32;
            let y = (p.top(p.x, g.world_time) - g.cam_y) as i32;
            let (index, top, h) = if p.kind == 1 {
                (1, y - 4, 58)
            } else if p.kind == 2 {
                (2, y - 25, 83)
            } else {
                (0, y - 4, 75)
            };
            self.sprite(false, index, x, top, p.width as i32, h, 0);
            if p.kind == 1 {
                self.rect(x + 8, y + 5, p.width as i32 - 16, 2, [76, 224, 255, 180]);
            }
            let cx = (p.x + p.width * 0.56 - g.cam_x) as i32;
            let cy = (p.top(p.x + p.width * 0.56, g.world_time) - g.cam_y) as i32;
            if p.kind == 3 {
                let compress = g.vy < -300.0 && (g.x + 12.0 - (p.x + p.width * 0.6)).abs() < 40.0;
                self.sprite(
                    false,
                    3,
                    (p.x + p.width * 0.6 - g.cam_x) as i32 - 14,
                    cy - 25,
                    28,
                    if compress { 16 } else { 28 },
                    0,
                );
            }
            if p.hazard {
                self.sprite(false, 7, x + p.width as i32 - 45, y - 22, 35, 24, 0);
            }
            if p.enemy {
                let ex = p.x + p.width * 0.5 + sinf(g.world_time * 2.0 + p.phase) * 22.0 - g.cam_x;
                let ey = p.top(p.x + p.width * 0.5, g.world_time) - 58.0
                    + sinf(g.world_time * 3.0 + p.phase) * 10.0
                    - g.cam_y;
                self.sprite(false, 14, ex as i32 - 19, ey as i32 - 18, 38, 36, 0);
                // Two rotor streaks animate at 15 Hz; red lens pulse remains readable.
                let flip = (g.world_time * 15.0) as i32 % 2;
                self.rect(
                    ex as i32 - 13 - flip * 3,
                    ey as i32 - 15,
                    26 + flip * 6,
                    1,
                    [142, 220, 255, 190],
                );
            }
            if p.power > 0 && !p.picked {
                let bob = (sinf(g.time * 3.0 + p.phase) * 3.0) as i32;
                self.ring(cx, cy - 44 + bob, 18, [89, 217, 255, 70]);
                self.sprite(
                    false,
                    p.power as usize + 7,
                    cx - 17,
                    cy - 61 + bob,
                    34,
                    34,
                    0,
                );
            }
        }
        for c in &g.coins {
            if !c.taken {
                let frame = [4, 5, 6, 5][((g.time * 8.0 + c.x * 0.01) as usize) % 4];
                self.sprite(
                    false,
                    frame,
                    (c.x - g.cam_x) as i32 - 8,
                    (c.y - g.cam_y) as i32 - 10,
                    16,
                    20,
                    0,
                );
            }
        }
        for p in &g.particles {
            self.rect(
                (p.x - g.cam_x) as i32,
                (p.y - g.cam_y) as i32,
                3,
                3,
                [
                    p.color[0],
                    p.color[1],
                    p.color[2],
                    (p.life * 400.0).min(255.0) as u8,
                ],
            );
        }
        let px = (g.x - g.cam_x) as i32;
        let py = (g.y - g.cam_y) as i32;
        if g.powers.effective(2) || g.invincible > 0.0 {
            self.ring(px + 12, py + 14, 25, [133, 235, 255, 200]);
            self.ring(px + 12, py + 14, 28, [89, 212, 255, 70]);
        }
        if g.powers.effective(1) {
            let r = 29 + (sinf(g.time * 8.0) * 4.0) as i32;
            self.ring(px + 12, py + 14, r, [154, 126, 255, 180]);
            self.ring(px + 12, py + 14, r + 12, [112, 213, 255, 65]);
        }
        if g.powers.effective(4) {
            self.ring(px + 12, py + 19, 22, [213, 129, 255, 150]);
        }
        if g.powers.effective(6) {
            self.world_view = false;
            self.rect(0, 0, WIDTH as i32, HEIGHT as i32, [72, 170, 220, 24]);
            self.world_view = true;
        }
        if g.invincible <= 0.0 || (g.time * 15.0) as u32 % 2 == 0 {
            self.sprite(
                true,
                g.player_frame(),
                px - 12,
                py - 18,
                48,
                48,
                g.progress.skin,
            );
        }
        self.world_view = false;
        if matches!(g.mode, Mode::Playing | Mode::Paused | Mode::GameOver) {
            self.hud(g);
        }
        match g.mode {
            Mode::Menu => self.menu(g),
            Mode::Paused => self.pause(g),
            Mode::GameOver => {
                self.rect(0, 0, 480, 272, [3, 10, 26, 130]);
                self.panel(95, 52, 290, 176);
                self.center(66, tr(g.progress.language, "run_end"), 2, WHITE);
                self.center(
                    99,
                    &format!("{} {}", tr(g.progress.language, "score"), g.score),
                    2,
                    GOLD,
                );
                self.center(
                    126,
                    &format!(
                        "{} +{}   {} +{}",
                        tr(g.progress.language, "banked"),
                        g.earned_coins,
                        tr(g.progress.language, "mission_reward"),
                        g.mission_coins
                    ),
                    1,
                    WHITE,
                );
                self.sprite(
                    true,
                    if g.score >= g.progress.best && g.score > 0 {
                        14
                    } else {
                        15
                    },
                    209,
                    138,
                    60,
                    60,
                    g.progress.skin,
                );
                self.center(
                    200,
                    &format!(
                        "{}   {}",
                        tr(g.progress.language, "retry"),
                        tr(g.progress.language, "back")
                    ),
                    1,
                    CYAN,
                );
            }
            Mode::Shop => self.shop(g),
            Mode::Help => self.help(g),
            Mode::Journal => self.journal(g),
            _ => {}
        }
    }

    fn hud(&mut self, g: &Game) {
        let lang = g.progress.language;
        self.panel(6, 6, 104, 18);
        self.text(
            11,
            12,
            &format!("{} {:05}", tr(lang, "score"), g.score.min(9999999)),
            1,
            WHITE,
        );
        self.panel(404, 6, 70, 18);
        self.sprite(false, 4, 408, 7, 12, 15, 0);
        self.text(423, 12, &format!("{}", g.run_coins.min(99999)), 1, GOLD);
        self.panel(6, 28, 46, 18);
        self.text(10, 33, "L/R", 1, CYAN);
        self.rect(32, 32, 15, 8, [29, 60, 91, 255]);
        let ready = 1.0 - g.dash_cooldown / g.progress.dash_seconds();
        self.rect(32, 32, (15.0 * ready.clamp(0.0, 1.0)) as i32, 8, CYAN);
        let count = g.powers.mask().count_ones() as i32;
        let mut slot = 0;
        for power in 1..=6 {
            if !g.powers.has(power) {
                continue;
            }
            let x = 474 - count * 35 + slot * 35;
            let remaining = g.powers.remaining(power);
            let suspended = g.powers.suspended(power);
            let color = if suspended {
                [155, 172, 194, 255]
            } else if remaining <= 3.0 {
                if remaining < 1.5 && (g.time * 6.0) as u32 % 2 == 0 {
                    [255, 112, 107, 255]
                } else {
                    GOLD
                }
            } else {
                CYAN
            };
            self.panel(x, 28, 32, 22);
            self.sprite(false, power as usize + 7, x + 1, 30, 18, 18, 0);
            self.text(
                x + 20,
                35,
                &if suspended {
                    "II".into()
                } else {
                    format!("{}", libm::ceilf(remaining) as u32)
                },
                1,
                color,
            );
            self.rect(x + 2, 47, 28, 2, [29, 60, 91, 255]);
            self.rect(
                x + 2,
                47,
                (28.0 * remaining / g.powers.duration(power)).clamp(0.0, 28.0) as i32,
                2,
                color,
            );
            slot += 1;
        }
        if g.x < 280.0 && g.progress.runs < 3 {
            let text = tr(lang, "controls_hint");
            let width = text.len() as i32 * 6 + 12;
            self.panel((480 - width) / 2, 253, width, 13);
            self.center(256, text, 1, WHITE);
        }
    }
    fn pause(&mut self, g: &Game) {
        let lang = g.progress.language;
        self.rect(0, 0, 480, 272, [3, 10, 26, 150]);
        self.panel(74, 30, 332, 216);
        self.center(42, tr(lang, "pause"), 2, WHITE);
        self.center(
            65,
            &format!(
                "{} {}    {} {}",
                tr(lang, "height"),
                ((210.0 - g.y).max(0.0) / 10.0) as u32,
                tr(lang, "score"),
                g.score
            ),
            1,
            GOLD,
        );
        self.text(90, 87, tr(lang, "active_bonus"), 1, CYAN);
        let mut row = 0;
        for power in 1..=6 {
            if !g.powers.has(power) {
                continue;
            }
            let y = 102 + row * 16;
            self.sprite(false, power as usize + 7, 90, y - 3, 15, 15, 0);
            let key = format!("power_{}", power);
            self.text(111, y, tr(lang, &key), 1, WHITE);
            self.text(
                258,
                y,
                &if g.powers.suspended(power) {
                    tr(lang, "suspended").into()
                } else {
                    format!("{:.1} s", g.powers.remaining(power))
                },
                1,
                CYAN,
            );
            row += 1;
        }
        if row == 0 {
            self.text(90, 109, tr(lang, "no_bonus"), 1, WHITE);
        }
        self.center(193, tr(lang, "help_conflict"), 1, [155, 172, 194, 255]);
        self.center(213, tr(lang, "resume"), 1, CYAN);
        self.center(230, tr(lang, "leave"), 1, WHITE);
    }
    fn menu(&mut self, g: &Game) {
        let lang = g.progress.language;
        self.rect(0, 0, 480, 272, [4, 17, 39, 90]);
        self.panel(22, 20, 289, 230);
        self.text(38, 33, "SKY HOPPER", 3, WHITE);
        self.text(40, 61, tr(lang, "subtitle"), 1, CYAN);
        self.rect(40, 79, 247, 1, [93, 196, 239, 180]);
        for (i, key) in ["play", "store", "journal", "help", "language"]
            .iter()
            .enumerate()
        {
            let y = 94 + i as i32 * 25;
            if i == g.menu {
                self.rect(32, y - 5, 267, 20, [21, 78, 121, 235]);
                self.text(38, y, ">", 1, GOLD);
            }
            let label = if i == 4 {
                format!("{}  ITA / ENG", tr(lang, key))
            } else {
                tr(lang, key).into()
            };
            self.text(
                54,
                y,
                &label,
                1,
                if i == g.menu {
                    WHITE
                } else {
                    [155, 192, 216, 255]
                },
            );
        }
        self.text(
            40,
            229,
            &format!(
                "{} {}  {} {}",
                tr(lang, "best"),
                g.progress.best.min(999999),
                tr(lang, "coins"),
                g.progress.bank.min(999999)
            ),
            1,
            GOLD,
        );
        self.sprite(
            true,
            if (g.time * 1.5) as u32 % 6 == 0 { 9 } else { 8 },
            328,
            84 + (sinf(g.time * 2.0) * 4.0) as i32,
            112,
            112,
            g.progress.skin,
        );
        self.text(324, 211, tr(lang, "launch"), 1, CYAN);
        let launch = if g.progress.launch == 0 {
            tr(lang, "none")
        } else {
            tr(
                lang,
                match g.progress.launch {
                    1 => "power_1",
                    2 => "power_2",
                    _ => "power_5",
                },
            )
        };
        self.text(324, 227, launch, 1, GOLD);
        self.center(
            258,
            &format!("{}   {}", tr(lang, "confirm"), tr(lang, "choose")),
            1,
            WHITE,
        );
    }
    fn wrap(&mut self, x: i32, y: i32, text: &str, columns: usize, color: [u8; 4]) {
        let mut line = alloc::string::String::new();
        let mut row = 0;
        for word in text.split_whitespace() {
            if !line.is_empty() && line.len() + word.len() + 1 > columns {
                self.text(x, y + row * 12, &line, 1, color);
                line.clear();
                row += 1;
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        self.text(x, y + row * 12, &line, 1, color);
    }
    fn shop(&mut self, g: &Game) {
        let lang = g.progress.language;
        self.rect(0, 0, 480, 272, [4, 17, 39, 180]);
        self.panel(8, 8, 464, 248);
        self.text(20, 21, tr(lang, "store"), 2, WHITE);
        self.text(
            314,
            26,
            &format!("{} {}", tr(lang, "coins"), g.progress.bank.min(999999)),
            1,
            GOLD,
        );
        let item = ITEMS[g.shop];
        let category = item.category;
        for (i, key) in ["upgrades", "stock", "style"].iter().enumerate() {
            let x = 20 + i as i32 * 147;
            self.rect(
                x,
                43,
                137,
                18,
                if i == category {
                    [24, 77, 123, 255]
                } else {
                    [10, 32, 54, 255]
                },
            );
            self.text(
                x + 7,
                48,
                tr(lang, key),
                1,
                if i == category { WHITE } else { CYAN },
            );
        }
        let (start, end) = [(0, 9), (9, 12), (12, 15)][category];
        let first = start + (g.shop - start).saturating_sub(4);
        for i in first..(first + 5).min(end) {
            let y = 75 + (i - first) as i32 * 26;
            if i == g.shop {
                self.rect(18, y - 6, 245, 23, [24, 77, 123, 255]);
            }
            self.text(24, y, tr(lang, ITEMS[i].key), 1, WHITE);
            let level = g.progress.level(ITEMS[i]);
            self.text(
                216,
                y,
                &format!("{}/{}", level, ITEMS[i].max),
                1,
                if level == ITEMS[i].max { CYAN } else { GOLD },
            );
        }
        if end - start > 5 {
            self.text(
                22,
                208,
                &format!("{} / {}", g.shop - start + 1, end - start),
                1,
                CYAN,
            );
        }
        self.rect(274, 71, 1, 145, [54, 148, 209, 180]);
        self.wrap(286, 76, tr(lang, item.key), 28, WHITE);
        let desc = match (item.kind, item.index) {
            (0, 4) => "jet_desc",
            (0, _) => "duration_desc",
            (1, 0) => "magnet_desc",
            (1, 1) => "dash_desc",
            (1, _) => "coin_desc",
            (2, _) => "capsule_desc",
            _ => "cosmetic_desc",
        };
        self.wrap(286, 102, tr(lang, desc), 28, CYAN);
        let value = match item.kind {
            0 => format!(
                "+{:.1} s",
                g.progress.level(item) as f32
                    * if item.index == 4 {
                        JET_DURATION_STEP
                    } else {
                        DURATION_STEP
                    }
            ),
            1 if item.index == 0 => format!("{}", g.progress.magnet_radius() as u32),
            1 if item.index == 1 => format!("{:.1} s", g.progress.dash_seconds()),
            1 => format!("+{}%", g.progress.skills[2] * COIN_YIELD_PERCENT),
            2 => format!("{} / {}", g.progress.stock[item.index], item.max),
            _ => tr(
                lang,
                if g.progress.level(item) > 0 {
                    "unlocked"
                } else {
                    "locked"
                },
            )
            .into(),
        };
        self.text(286, 143, &value, 2, GOLD);
        let full = g.progress.level(item) >= item.max;
        let action = if item.kind == 3 && full {
            let equipped = if item.index < 2 {
                g.progress.skin == item.index as u32 + 1
            } else {
                g.progress.world == 1
            };
            tr(lang, if equipped { "unequip" } else { "equip" }).into()
        } else if full {
            tr(lang, "max").into()
        } else {
            format!("{} {}", g.progress.cost(item), tr(lang, "coins"))
        };
        self.text(
            286,
            170,
            &action,
            1,
            if !full && g.progress.bank < g.progress.cost(item) {
                [255, 112, 107, 255]
            } else {
                CYAN
            },
        );
        if item.kind == 2 {
            self.text(286, 187, "L/R", 1, WHITE);
            self.text(
                315,
                187,
                tr(
                    lang,
                    if g.progress.launch == item.index as u32 + 1 {
                        "unequip_short"
                    } else {
                        "equip_short"
                    },
                ),
                1,
                CYAN,
            );
        }
        let notice = match g.store_notice {
            1 => "purchased",
            2 => "need_coins",
            3 => "full",
            4 => "equipped",
            5 => "none",
            _ => "stored",
        };
        self.text(
            22,
            224,
            tr(lang, notice),
            1,
            if g.store_notice == 2 { GOLD } else { CYAN },
        );
        self.text(
            22,
            243,
            &format!(
                "{}  |  <> {}  |  {}",
                tr(lang, "buy"),
                tr(lang, "choose"),
                tr(lang, "back")
            ),
            1,
            WHITE,
        );
    }
    fn journal(&mut self, g: &Game) {
        let lang = g.progress.language;
        self.rect(0, 0, 480, 272, [4, 17, 39, 180]);
        self.panel(8, 8, 464, 248);
        self.text(20, 22, tr(lang, "objectives"), 2, WHITE);
        self.text(
            310,
            27,
            &format!("{} {}", tr(lang, "runs"), g.progress.runs),
            1,
            GOLD,
        );
        for (i, key) in ["mission_distance", "mission_coins", "mission_pickups"]
            .iter()
            .enumerate()
        {
            let y = 58 + i as i32 * 36;
            self.text(22, y, tr(lang, key), 1, WHITE);
            self.text(
                239,
                y,
                &format!("{} / {}", g.progress.goals[i], g.progress.target(i)),
                1,
                CYAN,
            );
            self.text(370, y, &format!("+{}", g.progress.reward(i)), 1, GOLD);
            self.rect(22, y + 14, 425, 3, [29, 60, 91, 255]);
            self.rect(
                22,
                y + 14,
                (425.0 * g.progress.goals[i] as f32 / g.progress.target(i) as f32) as i32,
                3,
                CYAN,
            );
        }
        self.text(
            22,
            174,
            &format!(
                "{} {}/9",
                tr(lang, "collection"),
                g.progress.relics.count_ones()
            ),
            1,
            WHITE,
        );
        for i in 0..9 {
            let x = 22 + i as i32 * 48;
            let owned = g.progress.relics & (1 << i) != 0;
            self.panel(x, 191, 39, 32);
            if owned {
                self.sprite(false, [8, 9, 12][i / 3], x + 7, 194, 25, 25, 0);
                self.text(x + 2, 216, &format!("{}", i % 3 + 1), 1, GOLD);
            } else {
                self.text(x + 17, 202, "?", 1, [155, 172, 194, 255]);
            }
        }
        self.text(
            22,
            243,
            &format!("{}  |  {}", tr(lang, "journal_hint"), tr(lang, "back")),
            1,
            CYAN,
        );
    }
    fn help(&mut self, g: &Game) {
        let lang = g.progress.language;
        self.rect(0, 0, 480, 272, [4, 17, 39, 180]);
        self.panel(8, 8, 464, 248);
        self.text(22, 23, tr(lang, "help"), 2, WHITE);
        for (i, key) in [
            "help_route",
            "help_jump",
            "help_left",
            "help_right",
            "help_dash",
            "help_pause",
            "help_jet",
            "help_stack",
            "help_conflict",
            "help_save",
        ]
        .iter()
        .enumerate()
        {
            self.text(
                22,
                57 + i as i32 * 17,
                tr(lang, key),
                1,
                if i == 0 { GOLD } else { WHITE },
            );
        }
        self.text(22, 243, tr(lang, "back"), 1, CYAN);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PowerState;
    #[test]
    fn clips_and_blends_at_screen_edges() {
        let mut s = Screen::new();
        s.rect(-5, -5, 10, 10, [200, 100, 50, 255]);
        assert_eq!(&s.pixels[..4], &[200, 100, 50, 255]);
        s.rect(0, 0, 1, 1, [0, 0, 0, 128]);
        assert_eq!(s.pixels[0], 99);
        s.sprite(true, 15, -32, 240, 96, 96, 2);
        assert_eq!(s.pixels.len(), 480 * 272 * 4);
    }
    #[test]
    fn renders_every_mode_and_power() {
        let mut s = Screen::new();
        let mut g = Game::new(1);
        for mode in [
            Mode::Menu,
            Mode::Playing,
            Mode::Paused,
            Mode::GameOver,
            Mode::Shop,
            Mode::Help,
            Mode::Journal,
        ] {
            for power in 0..7 {
                g.mode = mode;
                g.powers = PowerState::default();
                g.powers.collect(power, false);
                s.draw(&g);
            }
        }
        g.mode = Mode::Playing;
        for power in [1, 2, 4, 5, 6] {
            g.powers.collect(power, false);
        }
        s.draw(&g);
        assert!(s.pixels.chunks_exact(4).all(|p| p[3] == 255));
    }
    #[test]
    fn camera_scales_world_but_keeps_hud_at_native_size() {
        let mut s = Screen::new();
        assert_eq!(s.transform(115, 210, 48, 48), (115, 210, 48, 48));
        s.world_view = true;
        assert_eq!(s.transform(115, 210, 48, 48), (115, 210, 38, 38));
        s.world_view = false;
        assert_eq!(s.transform(404, 6, 70, 18), (404, 6, 70, 18));
    }
}

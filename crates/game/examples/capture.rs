use sky_hopper_game::{Game, Mode, Screen, JUMP};
use std::{fs, path::Path};
fn write(screen: &Screen, name: &str) {
    fs::write(format!("docs/captures/{name}.rgba"), &screen.pixels).unwrap();
}
fn main() {
    fs::create_dir_all("docs/captures").unwrap();
    let mut g = Game::new(42);
    let mut s = Screen::new();
    s.draw(&g);
    write(&s, "menu");
    g.reset();
    for i in 0..480 {
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
        g.step(buttons);
        if i == 80 {
            g.activate_power(2);
        }
        if i == 150 {
            s.draw(&g);
            write(&s, "gameplay");
        }
        if i % 8 == 0 && g.mode == Mode::Playing {
            s.draw(&g);
            write(&s, &format!("demo-{i:03}"));
        }
        if g.mode == Mode::GameOver {
            break;
        }
    }
    for power in 1..7 {
        g.reset();
        g.x = 340.0;
        g.y = 110.0;
        g.cam_x = 225.0;
        g.activate_power(power);
        s.draw(&g);
        write(&s, &format!("power-{power}"));
    }
    g.mode = Mode::Shop;
    s.draw(&g);
    write(&s, "hangar");
    assert!(Path::new("docs/captures/gameplay.rgba").exists());
}

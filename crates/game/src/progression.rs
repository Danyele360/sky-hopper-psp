//! Permanent progression and legacy save migration. No network or clock needed.
#[derive(Clone, Copy)]
pub struct Item {
    pub key: &'static str,
    pub kind: u8,
    pub index: usize,
    pub max: u32,
    pub cost: u32,
    pub step: u32,
    pub category: usize,
    pub power: u8,
}
include!("design_rules.rs");

pub const SAVE_WORDS: usize = 32;
pub const SAVE_KEYS: [&str; SAVE_WORDS] = [
    "best",
    "bank",
    "owned",
    "skin",
    "world",
    "upgrade",
    "version",
    "duration1",
    "duration2",
    "duration3",
    "duration4",
    "duration5",
    "duration6",
    "range",
    "cooldown",
    "yield",
    "stock1",
    "stock2",
    "stock5",
    "launch",
    "language",
    "runs",
    "distance",
    "coins",
    "pickups",
    "goal1",
    "goal2",
    "goal3",
    "tier1",
    "tier2",
    "tier3",
    "relics",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Progress {
    pub best: u32,
    pub bank: u32,
    pub owned: u32,
    pub skin: u32,
    pub world: u32,
    pub upgraded: u32,
    pub levels: [u32; 6],
    pub skills: [u32; 3],
    pub stock: [u32; 3],
    /// 0 means no capsule armed; 1..=3 identifies a stock slot.
    pub launch: u32,
    pub language: u32,
    pub runs: u32,
    pub totals: [u32; 3],
    pub goals: [u32; 3],
    pub tiers: [u32; 3],
    pub relics: u32,
}
impl Progress {
    pub fn words(self) -> [u32; SAVE_WORDS] {
        let mut w = [0; SAVE_WORDS];
        w[..6].copy_from_slice(&[
            self.best,
            self.bank,
            self.owned,
            self.skin,
            self.world,
            self.upgraded,
        ]);
        w[6] = 2;
        w[7..13].copy_from_slice(&self.levels);
        w[13..16].copy_from_slice(&self.skills);
        w[16..19].copy_from_slice(&self.stock);
        w[19] = self.launch;
        w[20] = self.language;
        w[21] = self.runs;
        w[22..25].copy_from_slice(&self.totals);
        w[25..28].copy_from_slice(&self.goals);
        w[28..31].copy_from_slice(&self.tiers);
        w[31] = self.relics;
        w
    }
    pub fn from_words<const N: usize>(words: [u32; N]) -> Self {
        let mut w = [0; SAVE_WORDS];
        let n = N.min(SAVE_WORDS);
        w[..n].copy_from_slice(&words[..n]);
        let mut p = Self::default();
        p.best = w[0];
        p.bank = w[1];
        p.owned = w[2] & 7;
        let skin = w[3].min(2);
        p.skin = if skin == 0 || p.owned & (1 << (skin - 1)) != 0 {
            skin
        } else {
            0
        };
        p.world = if p.owned & 4 != 0 { w[4].min(1) } else { 0 };
        // The original all-bonus +2 s purchase stays valid after migration.
        p.upgraded = w[5].min(1);
        if w[6] == 2 {
            for i in 0..6 {
                p.levels[i] = w[7 + i].min(3);
            }
            for i in 0..3 {
                p.skills[i] = w[13 + i].min(3);
                p.stock[i] = w[16 + i].min(9);
                p.totals[i] = w[22 + i];
                p.tiers[i] = w[28 + i];
                p.goals[i] = w[25 + i].min(p.target(i).saturating_sub(1));
            }
            p.launch = w[19].min(3);
            if p.launch != 0 && p.stock[p.launch as usize - 1] == 0 {
                p.launch = 0;
            }
            p.language = w[20].min(1);
            p.runs = w[21];
            p.relics = w[31] & 511;
        }
        p
    }
    pub fn level(&self, item: Item) -> u32 {
        match item.kind {
            0 => self.levels[item.index],
            1 => self.skills[item.index],
            2 => self.stock[item.index],
            _ => (self.owned >> item.index) & 1,
        }
    }
    pub fn cost(&self, item: Item) -> u32 {
        item.cost
            + if item.kind < 2 {
                self.level(item) * item.step
            } else {
                0
            }
    }
    /// 1 purchased, 2 insufficient, 3 max, 4 equipped, 5 unequipped.
    pub fn purchase(&mut self, index: usize) -> u8 {
        let Some(&item) = ITEMS.get(index) else {
            return 3;
        };
        let level = self.level(item);
        if item.kind == 3 && level != 0 {
            if item.index < 2 {
                let skin = item.index as u32 + 1;
                self.skin = if self.skin == skin { 0 } else { skin };
                return if self.skin == 0 { 5 } else { 4 };
            }
            self.world = 1 - self.world;
            return if self.world == 0 { 5 } else { 4 };
        }
        if level >= item.max {
            return 3;
        }
        let cost = self.cost(item);
        if self.bank < cost {
            return 2;
        }
        self.bank -= cost;
        match item.kind {
            0 => self.levels[item.index] += 1,
            1 => self.skills[item.index] += 1,
            2 => self.stock[item.index] += 1,
            _ => {
                self.owned |= 1 << item.index;
                if item.index < 2 {
                    self.skin = item.index as u32 + 1;
                } else {
                    self.world = 1;
                }
            }
        }
        1
    }
    pub fn arm(&mut self, slot: usize) -> bool {
        if slot >= 3 || self.stock[slot] == 0 {
            return false;
        }
        let selected = slot as u32 + 1;
        self.launch = if self.launch == selected { 0 } else { selected };
        true
    }
    pub fn begin_run(&mut self) -> u8 {
        if !(1..=3).contains(&self.launch) {
            return 0;
        }
        let slot = self.launch as usize - 1;
        if self.stock[slot] == 0 {
            self.launch = 0;
            return 0;
        }
        self.stock[slot] -= 1;
        let power = [1, 2, 5][slot];
        if self.stock[slot] == 0 {
            self.launch = 0;
        }
        power
    }
    pub fn target(&self, i: usize) -> u32 {
        MISSION_TARGET[i] + (self.tiers[i] % 4) * MISSION_STEP[i]
    }
    pub fn reward(&self, i: usize) -> u32 {
        MISSION_REWARD[i] + (self.tiers[i] / 4).min(20) * 5
    }
    pub fn bank_run(
        &mut self,
        distance: u32,
        coins: u32,
        pickups: u32,
        score: u32,
    ) -> (u32, u32, u8) {
        self.runs = self.runs.saturating_add(1);
        self.best = self.best.max(score);
        let stats = [distance, coins, pickups];
        let mut rewards = 0;
        let mut completed = 0;
        for i in 0..3 {
            self.totals[i] = self.totals[i].saturating_add(stats[i]);
            self.goals[i] = self.goals[i].saturating_add(stats[i]);
            let target = self.target(i);
            // One reward per objective per run; surplus carries into the next goal.
            if self.goals[i] >= target {
                rewards += self.reward(i);
                self.relics |= 1 << (i * 3 + (self.tiers[i] % 3) as usize);
                self.tiers[i] = self.tiers[i].saturating_add(1);
                self.goals[i] = (self.goals[i] - target).min(self.target(i) - 1);
                completed |= 1 << i;
            }
        }
        let earned = coins.saturating_add(
            (coins as u64 * self.skills[2] as u64 * COIN_YIELD_PERCENT as u64 / 100)
                .min(u32::MAX as u64) as u32,
        );
        self.bank = self.bank.saturating_add(earned).saturating_add(rewards);
        (earned, rewards, completed)
    }
    pub fn dash_seconds(&self) -> f32 {
        3.0 - self.skills[1] as f32 * DASH_RECOVERY_STEP
    }
    pub fn magnet_radius(&self) -> f32 {
        125.0 + self.skills[0] as f32 * MAGNET_RANGE_STEP
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_save_preserves_coins_equipment_and_global_upgrade() {
        let p = Progress::from_words([100, 200, 7, 2, 1, 1]);
        assert_eq!((p.bank, p.skin, p.world, p.upgraded), (200, 2, 1, 1));
        assert_eq!(p.levels, [0; 6]);
        assert_eq!(Progress::from_words(p.words()), p);
    }
    #[test]
    fn purchases_are_bounded_and_insufficient_funds_leave_everything_unchanged() {
        let mut p = Progress::default();
        let before = p;
        assert_eq!(p.purchase(0), 2);
        assert_eq!(p, before);
        p.bank = 1000;
        for _ in 0..3 {
            assert_eq!(p.purchase(0), 1);
        }
        assert_eq!(p.bank, 1000 - 28 - 53 - 78);
        let before = p;
        assert_eq!(p.purchase(0), 3);
        assert_eq!(p, before);
    }
    #[test]
    fn capsules_accumulate_and_only_armed_one_is_consumed() {
        let mut p = Progress::default();
        p.bank = 100;
        p.purchase(9);
        p.purchase(9);
        p.purchase(10);
        assert_eq!(p.begin_run(), 0);
        assert_eq!(p.stock, [2, 1, 0]);
        assert!(p.arm(0));
        assert_eq!(p.begin_run(), 1);
        assert_eq!(p.stock, [1, 1, 0]);
        assert_eq!(p.begin_run(), 1);
        assert_eq!(p.launch, 0);
    }
    #[test]
    fn objectives_carry_between_runs_and_award_collection_once_per_completion() {
        let mut p = Progress::default();
        assert_eq!(p.bank_run(300, 6, 1, 50), (6, 0, 0));
        assert_eq!(p.bank_run(300, 6, 1, 50), (6, 60, 7));
        assert_eq!(p.bank, 72);
        assert_eq!(p.relics, 0b001001001);
        assert_eq!(p.tiers, [1; 3]);
        assert_eq!(p.goals, [0; 3]);
    }
    #[test]
    fn coin_upgrade_and_all_new_fields_roundtrip() {
        let mut p = Progress::default();
        p.bank = 1000;
        p.purchase(8);
        p.purchase(10);
        p.arm(1);
        p.language = 1;
        assert_eq!(p.bank_run(800, 16, 3, 236), (17, 60, 7));
        assert_eq!(Progress::from_words(p.words()), p);
    }
}

//! Concurrent bonuses, with one independent timer per bonus.
include!("power_rules.rs");

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PowerState {
    timers: [f32; 6],
    durations: [f32; 6],
}

impl PowerState {
    pub fn has(&self, id: u8) -> bool {
        self.remaining(id) > 0.0
    }
    pub fn remaining(&self, id: u8) -> f32 {
        if (1..=6).contains(&id) {
            self.timers[id as usize - 1]
        } else {
            0.0
        }
    }
    pub fn duration(&self, id: u8) -> f32 {
        if (1..=6).contains(&id) {
            self.durations[id as usize - 1]
        } else {
            0.0
        }
    }
    pub fn mask(&self) -> u8 {
        (1..=6).fold(0, |mask, id| {
            mask | if self.has(id) { 1 << (id - 1) } else { 0 }
        })
    }
    pub fn suspended(&self, id: u8) -> bool {
        self.has(id) && SUSPENDED_BY[id as usize - 1] & self.mask() != 0
    }
    pub fn effective(&self, id: u8) -> bool {
        self.has(id) && !self.suspended(id)
    }
    /// A repeat pickup refreshes its own timer; it does not add effect strength.
    /// Only explicitly incompatible bonuses are replaced by the new pickup.
    pub fn collect(&mut self, id: u8, upgraded: bool) {
        self.collect_extended(id, upgraded, 0.0);
    }
    pub fn collect_extended(&mut self, id: u8, upgraded: bool, extra_seconds: f32) {
        if !(1..=6).contains(&id) {
            return;
        }
        let index = id as usize - 1;
        let conflicts = INCOMPATIBLE[index];
        for other in 1..=6 {
            if conflicts & (1 << (other - 1)) != 0 {
                self.consume(other);
            }
        }
        let duration = BASE_SECONDS[index]
            + if upgraded { UPGRADE_SECONDS } else { 0.0 }
            + if extra_seconds.is_finite() {
                extra_seconds.clamp(0.0, 3.0)
            } else {
                0.0
            };
        self.timers[index] = self.timers[index].max(duration);
        self.durations[index] = self.timers[index];
    }
    pub fn consume(&mut self, id: u8) -> bool {
        if !self.has(id) {
            return false;
        }
        self.timers[id as usize - 1] = 0.0;
        self.durations[id as usize - 1] = 0.0;
        true
    }
    /// Return a bitmask of bonuses that expired. Call only while the game runs.
    pub fn tick(&mut self, dt: f32) -> u8 {
        if !dt.is_finite() || dt <= 0.0 {
            return 0;
        }
        let before = self.timers;
        let mut expired = 0;
        for index in 0..6 {
            if before[index] <= 0.0 {
                continue;
            }
            // If a suspending bonus ends partway through this step, resume the
            // suspended timer for the remaining part of the step.
            let mut suspended_seconds = 0.0f32;
            for other in 0..6 {
                if SUSPENDED_BY[index] & (1 << other) != 0 {
                    suspended_seconds = suspended_seconds.max(before[other]);
                }
            }
            let elapsed = (dt - suspended_seconds.min(dt)).max(0.0);
            self.timers[index] = (before[index] - elapsed).max(0.0);
            if self.timers[index] == 0.0 {
                self.durations[index] = 0.0;
                expired |= 1 << index;
            }
        }
        expired
    }
    #[cfg(test)]
    pub(crate) fn set_remaining(&mut self, id: u8, seconds: f32) {
        self.timers[id as usize - 1] = seconds;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compatible_pickups_keep_every_previous_bonus() {
        let mut powers = PowerState::default();
        for id in [1, 2, 3, 4, 6] {
            powers.collect(id, false);
        }
        assert_eq!(powers.mask(), 0b10_1111);
        assert_eq!(powers.remaining(1), 8.0);
    }
    #[test]
    fn movement_conflict_replaces_only_the_conflicting_bonus_in_both_orders() {
        for (first, last) in [(3, 5), (5, 3)] {
            let mut powers = PowerState::default();
            for id in [1, 2, 4, 6, first] {
                powers.collect(id, false);
            }
            powers.collect(last, false);
            assert!(!powers.has(first));
            assert!(powers.has(last));
            for id in [1, 2, 4, 6] {
                assert!(powers.has(id));
            }
        }
    }
    #[test]
    fn repeat_pickup_refreshes_only_its_timer() {
        let mut powers = PowerState::default();
        powers.collect(1, false);
        powers.collect(2, false);
        powers.tick(3.0);
        powers.collect(1, false);
        assert_eq!(powers.remaining(1), 8.0);
        assert_eq!(powers.remaining(2), 5.0);
        powers.collect(1, false);
        assert_eq!(powers.remaining(1), 8.0);
    }
    #[test]
    fn expiration_does_not_clear_other_bonuses() {
        let mut powers = PowerState::default();
        powers.collect(1, false);
        powers.tick(3.0);
        powers.collect(2, false);
        assert_eq!(powers.tick(5.0), 1);
        assert!(!powers.has(1));
        assert_eq!(powers.remaining(2), 3.0);
    }
    #[test]
    fn flight_suspends_super_jump_and_resumes_it_at_the_expiry_boundary() {
        let mut powers = PowerState::default();
        powers.collect(4, false);
        powers.collect(5, false);
        powers.tick(2.0);
        assert!(powers.suspended(4));
        assert!(!powers.effective(4));
        assert_eq!(powers.remaining(4), 8.0);
        assert_eq!(powers.tick(3.0), 1 << 4);
        assert!(!powers.has(5));
        assert!(powers.effective(4));
        assert_eq!(powers.remaining(4), 7.5);
    }
    #[test]
    fn consuming_a_shield_keeps_the_magnet() {
        let mut powers = PowerState::default();
        powers.collect(1, false);
        powers.collect(2, false);
        assert!(powers.consume(2));
        assert!(!powers.consume(2));
        assert!(powers.has(1));
    }
    #[test]
    fn upgrades_extend_each_bonus_independently() {
        let mut powers = PowerState::default();
        powers.collect(1, true);
        powers.collect(5, true);
        assert_eq!(powers.remaining(1), 10.0);
        assert_eq!(powers.remaining(5), 6.5);
    }
    #[test]
    fn invalid_inputs_leave_state_unchanged() {
        let mut powers = PowerState::default();
        powers.collect(1, false);
        let before = powers;
        powers.collect(0, false);
        powers.collect(7, false);
        powers.tick(f32::NAN);
        powers.tick(-1.0);
        assert_eq!(powers, before);
        assert!(!powers.suspended(255));
    }
}

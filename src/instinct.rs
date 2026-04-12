#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstinctType {
    Survive,
    Flee,
    Curious,
    Cooperate,
    Guard,
    Report,
    Hoard,
    Teach,
    Mour,
    Evolve,
    None,
}

#[derive(Debug)]
pub struct InstinctThresholds {
    pub energy_below: f32,
    pub energy_critical: f32,
    pub trust_threshold: f32,
    pub threat_threshold: f32,
    pub idle_cycles: u32,
    pub explore_interval: u32,
}

impl Default for InstinctThresholds {
    fn default() -> Self {
        Self {
            energy_below: 0.4,
            energy_critical: 0.1,
            trust_threshold: 0.7,
            threat_threshold: 0.6,
            idle_cycles: 10,
            explore_interval: 50,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstinctReflex {
    pub instinct_type: InstinctType,
    pub urgency: f32,
    pub energy_cost: f32,
}

pub struct InstinctEngine {
    thresholds: InstinctThresholds,
    cycle_count: u32,
    idle_count: u32,
}

impl InstinctEngine {
    pub fn new() -> Self {
        Self {
            thresholds: InstinctThresholds::default(),
            cycle_count: 0,
            idle_count: 0,
        }
    }

    pub fn evaluate(
        &mut self,
        energy_frac: f32,
        threat: f32,
        trust: f32,
        peer_alive: bool,
        has_work: bool,
    ) -> Vec<InstinctReflex> {
        self.cycle_count += 1;
        if !has_work {
            self.idle_count += 1;
        } else {
            self.idle_count = 0;
        }

        let mut reflexes = Vec::new();

        // Survive when critically low energy
        if energy_frac <= self.thresholds.energy_critical {
            reflexes.push(InstinctReflex {
                instinct_type: InstinctType::Survive,
                urgency: 1.0 - energy_frac,
                energy_cost: 0.1,
            });
        }

        // Hoard when energy is below threshold but not critical
        if energy_frac <= self.thresholds.energy_below && energy_frac > self.thresholds.energy_critical {
            reflexes.push(InstinctReflex {
                instinct_type: InstinctType::Hoard,
                urgency: self.thresholds.energy_below - energy_frac,
                energy_cost: 0.3,
            });
        }

        // Flee at high threat
        if threat >= self.thresholds.threat_threshold {
            reflexes.push(InstinctReflex {
                instinct_type: InstinctType::Flee,
                urgency: threat,
                energy_cost: 1.0,
            });
        }

        // Cooperate with high trust
        if trust >= self.thresholds.trust_threshold && peer_alive {
            reflexes.push(InstinctReflex {
                instinct_type: InstinctType::Cooperate,
                urgency: trust * 0.5,
                energy_cost: 0.2,
            });
        }

        // Curious when idle
        if self.idle_count >= self.thresholds.idle_cycles {
            reflexes.push(InstinctReflex {
                instinct_type: InstinctType::Curious,
                urgency: 0.3,
                energy_cost: 0.5,
            });
        }

        // Evolve periodically
        if self.cycle_count % self.thresholds.explore_interval == 0 {
            reflexes.push(InstinctReflex {
                instinct_type: InstinctType::Evolve,
                urgency: 0.1,
                energy_cost: 5.0,
            });
        }

        reflexes
    }

    pub fn highest_priority(&self, reflexes: &[InstinctReflex]) -> Option<InstinctReflex> {
        reflexes.iter().max_by(|a, b| a.urgency.partial_cmp(&b.urgency).unwrap()).cloned()
    }

    pub fn is_firing(&self, reflexes: &[InstinctReflex], t: InstinctType) -> bool {
        reflexes.iter().any(|r| r.instinct_type == t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instinct_survive_at_critical() {
        let mut eng = InstinctEngine::new();
        let refs = eng.evaluate(0.05, 0.0, 0.0, false, true);
        assert!(eng.is_firing(&refs, InstinctType::Survive));
    }

    #[test]
    fn instinct_hoard_at_low() {
        let mut eng = InstinctEngine::new();
        let refs = eng.evaluate(0.25, 0.0, 0.0, false, true);
        assert!(eng.is_firing(&refs, InstinctType::Hoard));
        assert!(!eng.is_firing(&refs, InstinctType::Survive));
    }

    #[test]
    fn instinct_flee_at_high_threat() {
        let mut eng = InstinctEngine::new();
        let refs = eng.evaluate(0.8, 0.9, 0.0, false, true);
        assert!(eng.is_firing(&refs, InstinctType::Flee));
    }

    #[test]
    fn instinct_cooperate_with_trust() {
        let mut eng = InstinctEngine::new();
        let refs = eng.evaluate(0.8, 0.0, 0.8, true, true);
        assert!(eng.is_firing(&refs, InstinctType::Cooperate));
    }

    #[test]
    fn instinct_priority_ordering() {
        let mut eng = InstinctEngine::new();
        // Critical energy (urgency ~0.95) + high threat (urgency 0.9)
        let refs = eng.evaluate(0.05, 0.9, 0.0, false, true);
        let top = eng.highest_priority(&refs).unwrap();
        assert_eq!(top.instinct_type, InstinctType::Survive);
    }
}

use crate::apoptosis::ApoptosisLevel;
use crate::circadian::CircadianRhythm;
use crate::costs::EnergyCosts;

#[derive(Debug)]
pub enum EnergyResult {
    Ok(f32),
    Insufficient { needed: f32, available: f32 },
}

pub struct AtpPool {
    atp: f32,
    max_atp: f32,
    base_rate: f32,
    peak_rate: f32,
}

impl AtpPool {
    pub fn new(max: f32, base: f32, peak: f32) -> Self {
        Self { atp: max, max_atp: max, base_rate: base, peak_rate: peak }
    }

    pub fn consume(&mut self, cost: f32) -> EnergyResult {
        if cost <= self.atp {
            self.atp -= cost;
            EnergyResult::Ok(cost)
        } else {
            EnergyResult::Insufficient { needed: cost, available: self.atp }
        }
    }

    pub fn generate(&mut self, rhythm: &CircadianRhythm, hour: u8) -> f32 {
        let mult = rhythm.multiplier(hour);
        let rate = self.base_rate + mult * (self.peak_rate - self.base_rate);
        self.atp = (self.atp + rate).min(self.max_atp);
        rate
    }

    pub fn fraction(&self) -> f32 {
        self.atp / self.max_atp
    }

    pub fn can_afford(&self, costs: &EnergyCosts, op: &str) -> bool {
        let cost = match op {
            "perception" => costs.perception,
            "arithmetic" => costs.arithmetic,
            "deliberation" => costs.deliberation,
            "communication" => costs.communication,
            "memory_read" => costs.memory_read,
            "memory_write" => costs.memory_write,
            "instinct" => costs.instinct,
            "evolution" => costs.evolution,
            _ => return false,
        };
        self.atp >= cost
    }

    pub fn is_depleted(&self) -> bool {
        self.atp <= 0.0
    }

    pub fn apoptosis_level(&self) -> ApoptosisLevel {
        let f = self.fraction();
        if f <= 0.05 { ApoptosisLevel::Critical }
        else if f <= 0.2 { ApoptosisLevel::Starving }
        else if f <= 0.4 { ApoptosisLevel::LowEnergy }
        else { ApoptosisLevel::None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atp_new() {
        let pool = AtpPool::new(100.0, 2.0, 10.0);
        assert!((pool.fraction() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn consume_deducts() {
        let mut pool = AtpPool::new(100.0, 2.0, 10.0);
        let r = pool.consume(10.0);
        assert!(matches!(r, EnergyResult::Ok(10.0)));
        assert!((pool.fraction() - 0.9).abs() < 1e-6);
    }

    #[test]
    fn consume_insufficient() {
        let mut pool = AtpPool::new(5.0, 2.0, 10.0);
        let r = pool.consume(10.0);
        assert!(matches!(r, EnergyResult::Insufficient { .. }));
    }

    #[test]
    fn generate_produces() {
        let mut pool = AtpPool::new(100.0, 2.0, 10.0);
        pool.consume(50.0);
        let rhythm = CircadianRhythm::default();
        let gen = pool.generate(&rhythm, 12);
        assert!(gen > 0.0);
        assert!(pool.fraction() > 0.5);
    }

    #[test]
    fn can_afford_checks() {
        let pool = AtpPool::new(100.0, 2.0, 10.0);
        let costs = EnergyCosts::default();
        assert!(pool.can_afford(&costs, "perception"));
        assert!(pool.can_afford(&costs, "evolution"));
    }

    #[test]
    fn can_afford_unknown_op() {
        let pool = AtpPool::new(100.0, 2.0, 10.0);
        let costs = EnergyCosts::default();
        assert!(!pool.can_afford(&costs, "nonexistent"));
    }

    #[test]
    fn apoptosis_levels() {
        let mut pool = AtpPool::new(100.0, 2.0, 10.0);
        assert_eq!(pool.apoptosis_level(), ApoptosisLevel::None);
        pool.consume(61.0); // 39 left
        assert_eq!(pool.apoptosis_level(), ApoptosisLevel::LowEnergy);
        pool.consume(21.0); // 18 left
        assert_eq!(pool.apoptosis_level(), ApoptosisLevel::Starving);
        pool.consume(15.0); // 3 left
        assert_eq!(pool.apoptosis_level(), ApoptosisLevel::Critical);
    }
}

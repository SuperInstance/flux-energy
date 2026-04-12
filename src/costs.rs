use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnergyCosts {
    pub perception: f32,
    pub arithmetic: f32,
    pub deliberation: f32,
    pub communication: f32,
    pub memory_read: f32,
    pub memory_write: f32,
    pub instinct: f32,
    pub evolution: f32,
}

impl Default for EnergyCosts {
    fn default() -> Self {
        Self {
            perception: 0.5,
            arithmetic: 0.1,
            deliberation: 2.0,
            communication: 1.0,
            memory_read: 0.3,
            memory_write: 0.5,
            instinct: 0.2,
            evolution: 5.0,
        }
    }
}

impl EnergyCosts {
    pub fn estimate(&self, percepts: u32, arith: u32, delib: u32, comms: u32) -> f32 {
        percepts as f32 * self.perception
            + arith as f32 * self.arithmetic
            + delib as f32 * self.deliberation
            + comms as f32 * self.communication
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn costs_default() {
        let c = EnergyCosts::default();
        assert!((c.perception - 0.5).abs() < f32::EPSILON);
        assert!((c.arithmetic - 0.1).abs() < f32::EPSILON);
        assert!((c.evolution - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn costs_estimate() {
        let c = EnergyCosts::default();
        let cost = c.estimate(2, 3, 1, 0);
        let expected = 2.0 * 0.5 + 3.0 * 0.1 + 1.0 * 2.0;
        assert!((cost - expected).abs() < 1e-6);
    }

    #[test]
    fn costs_serde_roundtrip() {
        let c = EnergyCosts::default();
        let json = serde_json::to_string(&c).unwrap();
        let back: EnergyCosts = serde_json::from_str(&json).unwrap();
        assert!((back.perception - c.perception).abs() < f32::EPSILON);
    }
}

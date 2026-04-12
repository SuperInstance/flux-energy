use std::f32::consts::PI;

pub struct CircadianRhythm {
    peak_start: u8,
    peak_end: u8,
    peak_mult: f32,
    trough_mult: f32,
}

impl CircadianRhythm {
    pub fn default() -> Self {
        Self { peak_start: 8, peak_end: 20, peak_mult: 1.0, trough_mult: 0.2 }
    }

    pub fn multiplier(&self, hour: u8) -> f32 {
        let h = hour as f32;
        let ps = self.peak_start as f32;
        let pe = self.peak_end as f32;
        let peak_mid = (ps + pe) / 2.0;
        let peak_half = (pe - ps) / 2.0;

        // Normalized phase: 0 at peak center, PI at trough center
        let phase = ((h - peak_mid) / peak_half) * (PI / 2.0);
        // Cosine: 1 at peak, -1 at trough
        let cos_val = phase.cos();
        let t = (cos_val + 1.0) / 2.0; // 1=peak, 0=trough
        self.trough_mult + t * (self.peak_mult - self.trough_mult)
    }

    pub fn is_peak(&self, hour: u8) -> bool {
        let start = self.peak_start;
        let end = self.peak_end;
        if start <= end {
            hour >= start && hour < end
        } else {
            hour >= start || hour < end
        }
    }

    pub fn is_dreaming(&self, hour: u8) -> bool {
        // Dreaming in the deep trough: roughly 1-5 AM
        hour >= 1 && hour <= 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circadian_peak_multiplier() {
        let r = CircadianRhythm::default();
        let peak = r.multiplier(14); // middle of 8-20
        assert!(peak >= 0.9);
    }

    #[test]
    fn circadian_trough_lower() {
        let r = CircadianRhythm::default();
        let trough = r.multiplier(2); // deep night
        let peak = r.multiplier(14);
        assert!(trough < peak);
    }

    #[test]
    fn circadian_midnight_wrap() {
        let r = CircadianRhythm::default();
        let m0 = r.multiplier(0);
        let m23 = r.multiplier(23);
        // Both should be low and close
        assert!((m0 - m23).abs() < 0.2);
        assert!(m0 < 0.5);
    }
}

use flux_energy::*;

/// Integration test: full energy lifecycle simulating a VM cycle.
#[test]
fn full_lifecycle_energy_conserve_and_generate() {
    let mut pool = AtpPool::new(100.0, 2.0, 10.0);
    let costs = EnergyCosts::default();
    let rhythm = CircadianRhythm::default();

    // Start at full energy
    assert_eq!(pool.apoptosis_level(), ApoptosisLevel::None);

    // Simulate several heavy deliberation operations during peak hours
    for _ in 0..5 {
        pool.consume(costs.deliberation);
    }

    // Energy should have dropped
    assert!(pool.fraction() < 1.0);

    // Generate some energy at noon (peak)
    let gen = pool.generate(&rhythm, 12);
    assert!(gen > 0.0);

    // Still not depleted
    assert!(!pool.is_depleted());
}

/// Integration test: circadian rhythm affects generation at different hours.
#[test]
fn circadian_night_generates_less_than_day() {
    let mut pool_day = AtpPool::new(100.0, 2.0, 10.0);
    let mut pool_night = AtpPool::new(100.0, 2.0, 10.0);
    pool_day.consume(50.0);
    pool_night.consume(50.0);

    let rhythm = CircadianRhythm::default();
    let gen_day = pool_day.generate(&rhythm, 14);
    let gen_night = pool_night.generate(&rhythm, 3);

    assert!(gen_day > gen_night, "Day generation ({}) should exceed night ({})", gen_day, gen_night);
}

/// Integration test: instinct engine triggers based on energy state.
#[test]
fn instinct_triggers_survive_when_depleted() {
    let mut pool = AtpPool::new(100.0, 2.0, 10.0);
    let mut engine = InstinctEngine::new();

    // Drain energy to critical
    pool.consume(96.0); // 4% left

    assert_eq!(pool.apoptosis_level(), ApoptosisLevel::Critical);

    // Instinct engine should fire Survive
    let reflexes = engine.evaluate(pool.fraction(), 0.0, 0.0, false, true);
    assert!(engine.is_firing(&reflexes, InstinctType::Survive));
}

/// Integration test: is_peak and dreaming are mutually exclusive.
#[test]
fn peak_and_dreaming_mutually_exclusive() {
    let rhythm = CircadianRhythm::default();

    for hour in 0..=23u8 {
        let is_peak = rhythm.is_peak(hour);
        let is_dreaming = rhythm.is_dreaming(hour);
        // They can both be false (e.g., 6am, 7am), but not both true
        if is_peak && is_dreaming {
            panic!("Hour {} cannot be both peak and dreaming", hour);
        }
    }
}

/// Integration test: energy costs serialization roundtrip.
#[test]
fn energy_costs_json_roundtrip_full() {
    let costs = EnergyCosts::default();
    let json = serde_json::to_string(&costs).unwrap();
    let deserialized: EnergyCosts = serde_json::from_str(&json).unwrap();

    assert!((deserialized.perception - costs.perception).abs() < f32::EPSILON);
    assert!((deserialized.arithmetic - costs.arithmetic).abs() < f32::EPSILON);
    assert!((deserialized.deliberation - costs.deliberation).abs() < f32::EPSILON);
    assert!((deserialized.communication - costs.communication).abs() < f32::EPSILON);
    assert!((deserialized.memory_read - costs.memory_read).abs() < f32::EPSILON);
    assert!((deserialized.memory_write - costs.memory_write).abs() < f32::EPSILON);
    assert!((deserialized.instinct - costs.instinct).abs() < f32::EPSILON);
    assert!((deserialized.evolution - costs.evolution).abs() < f32::EPSILON);
}

/// Integration test: multiple instinct reflexes can fire simultaneously.
#[test]
fn multiple_reflexes_fire_together() {
    let mut engine = InstinctEngine::new();

    // High threat + low energy = Flee + Survive possible
    let reflexes = engine.evaluate(0.08, 0.8, 0.0, false, true);
    assert!(engine.is_firing(&reflexes, InstinctType::Survive));
    assert!(engine.is_firing(&reflexes, InstinctType::Flee));
    assert!(reflexes.len() >= 2);
}

/// Integration test: can_afford with all known operations.
#[test]
fn can_afford_all_operations() {
    let pool = AtpPool::new(100.0, 2.0, 10.0);
    let costs = EnergyCosts::default();

    let ops = [
        "perception", "arithmetic", "deliberation", "communication",
        "memory_read", "memory_write", "instinct", "evolution",
    ];

    for op in &ops {
        assert!(pool.can_afford(&costs, op), "Should afford '{}' at full energy", op);
    }

    // Drain to near zero
    let pool2 = AtpPool::new(1.0, 0.0, 0.0);
    assert!(!pool2.can_afford(&costs, "evolution")); // costs 5.0
}

/// Integration test: apoptosis levels progress as energy decreases.
#[test]
fn apoptosis_levels_progress() {
    let mut pool = AtpPool::new(100.0, 0.0, 0.0); // no regeneration

    // Full energy
    assert_eq!(pool.apoptosis_level(), ApoptosisLevel::None);

    // 39% → LowEnergy
    pool.consume(61.0);
    assert_eq!(pool.apoptosis_level(), ApoptosisLevel::LowEnergy);

    // 18% → Starving
    pool.consume(21.0);
    assert_eq!(pool.apoptosis_level(), ApoptosisLevel::Starving);

    // 3% → Critical
    pool.consume(15.0);
    assert_eq!(pool.apoptosis_level(), ApoptosisLevel::Critical);
}

/// Integration test: instinct evolve fires periodically.
#[test]
fn instinct_evolve_fires_periodically() {
    let mut engine = InstinctEngine::new();

    // At cycle 50 (explore_interval), Evolve should fire
    for i in 1..=50 {
        let reflexes = engine.evaluate(0.8, 0.0, 0.0, false, true);
        if i == 50 {
            assert!(engine.is_firing(&reflexes, InstinctType::Evolve), "Evolve should fire at cycle 50");
        }
    }
}

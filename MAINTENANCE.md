# MAINTENANCE.md

## flux-energy Maintenance

### Build & Test
```bash
cargo build
cargo test
cargo test -- --nocapture  # verbose output
```

### Architecture Notes
- `AtpPool` depends on `CircadianRhythm` for generation rate modulation
- `InstinctEngine` is stateful (tracks cycle/idle counts); not thread-safe without external sync
- `EnergyCosts` is fully serde-serializable; safe to persist to config files

### Syncing with energy-c
When energy-c API changes, update the corresponding Rust types:
1. `atp.h` → `src/atp.rs`
2. `energy_costs.h` → `src/costs.rs`
3. `circadian.h` → `src/circadian.rs`
4. `instinct.h` → `src/instinct.rs`

### Adding New Instincts
1. Add variant to `InstinctType` in `src/instinct.rs`
2. Add evaluation logic in `InstinctEngine::evaluate()`
3. Add tests covering the new instinct

### Version Policy
- Semver. Breaking changes to public API bump major.
- Keep energy-c parity at minor version boundaries.

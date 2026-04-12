# flux-energy

ATP energy system for the FLUX VM — a Rust library modeling biological energy dynamics with circadian rhythm modulation, instinct-driven resource management, and apoptosis signaling.

## Features

- **AtpPool** — Finite energy store with consume/generate/fraction tracking
- **CircadianRhythm** — Cosine-interpolated peak/trough modulation (configurable hours)
- **EnergyCosts** — Serde-serializable operation cost table with batch estimation
- **InstinctEngine** — Reflex-based survival logic matching [instinct-c](../instinct-c) behavior
- **ApoptosisLevel** — Four-tier energy crisis signaling (None → LowEnergy → Starving → Critical)

## Quick Start

```rust
use flux_energy::{AtpPool, CircadianRhythm, EnergyCosts, InstinctEngine};

let mut pool = AtpPool::new(100.0, 2.0, 10.0);
let rhythm = CircadianRhythm::default();
let costs = EnergyCosts::default();

pool.consume(costs.perception);
let generated = pool.generate(&rhythm, 14); // 2 PM peak
println!("Energy: {}%", pool.fraction() * 100.0);
println!("Status: {:?}", pool.apoptosis_level());
```

## Cross-Pollination

This crate is part of the FLUX VM energy ecosystem:

| Crate / Project | Language | Role |
|---|---|---|
| **energy-c** | C | Reference C implementation, embeddable in no_std targets |
| **cuda-energy** | Rust | GPU-accelerated energy simulation for large FLUX swarms |
| **cuda-dream-cycle** | Rust/CUDA | Dream-cycle integration with energy-aware memory consolidation |
| **flux-energy** | Rust | This crate — core energy library with serde support |

## License

MIT

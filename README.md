# initioChem

| | |
|---|---|
| **Version** | 0.1.0 (alpha) |
| **Status** | pseudoSpore consumer wired — Tier 1 operational |
| **Rust files** | 1 (main.rs + tests) |
| **Tests** | 2 (envelope load + pack/unpack round-trip) |
| **MSRV** | 1.87 (edition 2024) |
| **License** | AGPL-3.0 |
| **Unsafe** | `#![forbid(unsafe_code)]` |
| **C deps** | Zero (ecoBin compliant) |
| **Subcommands** | 5 (status, capabilities, view, validate, unpack) |
| **Primals consumed** | pseudospore-core (lithoSpore) — first consumer |
| **Source spring** | hotSpring (computational chemistry) |
| **Last validation** | Jul 17, 2026 — 2/2 tests, 0 clippy, 0 fmt, 0 doc |

**A [sporeGarden](https://github.com/sporeGarden) project — interactive computational chemistry via composed primals.**

initioChem is not a spring. It is a standalone sporeGarden product that
composes deployed primals (genomeBins/ecoBins from `plasmidBin/`) into an
interactive 3D molecular research platform via BYOB composition. Its dual purpose:

1. **Build a real research tool** — an interactive CompChem explorer where a
   chemist can view enzyme conformational landscapes, manipulate ring puckering
   in 3D, launch simulations, and share validated results as pseudoSpores.
2. **Find every gap** — in molecular visualization, GPU tessellation, scientific
   provenance, and compute dispatch. Gaps feed back as evolution pressure on
   petalTongue (Phase 4 molecular viz), barraCuda (tessellation), and the
   NUCLEUS composition patterns.

## Quick Start

```bash
cargo build --release
./target/release/initiochem status
./target/release/initiochem view <pseudospore-dir>
./target/release/initiochem validate <pseudospore-dir>
./target/release/initiochem unpack <tarball.tar.gz>
```

## CLI Subcommands

| Command | What |
|---------|------|
| `status` | Product version, degradation tier, composition status |
| `capabilities` | Available capabilities at current tier |
| `view <dir>` | Load + inspect pseudoSpore envelope (scope, modules, validation, checksums) |
| `validate <dir>` | Full envelope validation with integrity checks (spec VALID tier) |
| `unpack <tarball>` | Extract `.tar.gz` + validate envelope in one step |

## Architecture

initioChem sits atop the primal stack as a **molecular exploration engine**.
It consumes primals via JSON-RPC IPC — zero Rust crate dependencies on any
spring. Primals are resolved from `plasmidBin/` or discovered via Songbird
at runtime.

```
Springs (science + experiments)  →  produce  →  primals (genomeBin/ecoBin)
                                                       ↓
                                               plasmidBin/ (deployment)
                                                       ↓
                                      initioChem discovers + composes via IPC
```

### pseudoSpore Integration

initioChem is the **first external consumer** of the pseudoSpore format:

```
lithoSpore emit-pseudospore → pack-pseudospore → [distribute]
                                                       ↓
                              initioChem unpack → validate → view/analyze
```

Dependencies:
- `pseudospore-core` (git dependency from lithoSpore) — envelope loading, validation, tarball operations

## Graceful Degradation

| Tier | Requires | Capabilities |
|------|----------|-------------|
| 1 (standalone) | Nothing | Browse pseudoSpore, view pre-rendered figures, validate integrity |
| 2 (GPU compute) | toadStool + barraCuda | Live FES reconstruction, parameter sweeps, cross-landscape comparison |
| 3 (interactive) | + petalTongue | 3D molecular visualization, orbit navigation, FEL surface meshes |
| 4 (persistent) | + NestGate | Save/load sessions, share pseudoSpores |
| 5 (provenance) | + trio (rhizo/loam/sweet) | Sealed experiments, cryptographic lineage |
| 6 (full) | + Squirrel | AI-assisted analysis |

## Quality

```
cargo test                                             # 2 tests
cargo clippy --all-targets -- -W clippy::pedantic      # 0 warnings
cargo fmt --check                                      # clean
cargo doc --no-deps                                    # 0 warnings
```

## Directory Structure

```
initioChem/
├── README.md              # This file
├── Cargo.toml             # Workspace definition
├── deny.toml              # ecoBin C-dep bans
├── LICENSE                # AGPL-3.0
├── graphs/                # biomeOS deploy graphs (6 tiers)
├── niches/                # biomeOS niche YAML
├── specs/                 # Product specifications
│   └── PRODUCT_SPEC.md
└── src/                   # Product source
    ├── Cargo.toml
    └── main.rs            # CLI binary + tests
```

## References

- [sporeGarden org](https://github.com/sporeGarden)
- Source spring: `ecoPrimals/springs/hotSpring/`
- Product spec: `specs/PRODUCT_SPEC.md`
- lithoSpore pseudoSpore API: `pseudospore-core` crate
- Domain profile: `lithoSpore/profiles/initiochem-general.toml`

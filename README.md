# initioChem

| | |
|---|---|
| **Version** | 0.1.0 (seed) |
| **Status** | Architectural draft — no product binary yet |
| **Rust files** | 0 (seed structure only) |
| **MSRV** | 1.87 (edition 2024) |
| **License** | AGPL-3.0 |
| **Unsafe** | `#![forbid(unsafe_code)]` (target) |
| **C deps** | Zero (ecoBin compliant) |
| **Bridge methods** | 0 designed, 10+ planned |
| **Primals consumed** | 8 domains planned (compute, math, viz, storage, dag, lineage, security, ai) |
| **Source spring** | hotSpring (computational chemistry) |
| **Last validation** | N/A (pre-alpha) |

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

## Relationship to Siblings

| Product | Domain | Source Spring | Status |
|---------|--------|--------------|--------|
| **esotericWebb** | Narrative CRPG | ludoSpring | V6, 342 tests |
| **blueFish** | TBD | TBD | Pending transfer |
| **helixVision** | TBD | TBD | Planned |
| **initioChem** | Computational chemistry | hotSpring | Seed (this) |

All sporeGarden products follow the same architectural pattern:
- Consume primals via JSON-RPC IPC (zero crate deps on springs)
- Discover capabilities via Songbird at runtime
- Degrade gracefully when primals are absent
- Feed gaps back into the ecosystem

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

| Domain | Primal | Role | Status | IPC methods |
|--------|--------|------|--------|-------------|
| security | BearDog | Cryptographic identity, pseudoSpore signing | Planned | `crypto.sign`, `crypto.verify` |
| discovery | Songbird | Capability mesh, primal routing | Planned | `capability.discover`, `capability.register` |
| compute | ToadStool | GPU dispatch for FES reconstruction | Planned | `compute.dispatch.submit` |
| math | BarraCuda | Tessellation, projection, Gaussian summation | Planned | `math.tessellate.isosurface`, `math.project.perspective` |
| shader | CoralReef | WGSL shader compilation | Planned | `shader.compile.wgsl` |
| visualization | petalTongue | 3D molecular rendering, orbit camera, interaction | Planned | `visualization.render.scene`, `visualization.render.stream`, `interaction.poll` |
| storage | NestGate | PseudoSpore persistence | Planned | `storage.store`, `storage.retrieve` |
| dag | RhizoCrypt | Experiment session lineage | Planned | `dag.session.create`, `dag.event.append` |
| ledger | LoamSpine | Permanent experiment record | Planned | `entry.append` |
| attribution | SweetGrass | Collaborative authorship | Planned | `braid.create` |
| ai | Squirrel | FEL analysis suggestions | Planned | `ai.query` |

## Science Domain

initioChem wraps hotSpring's validated CompChem pipeline:

- **Backend:** `nest-validate` (Rust CLI — run/finalize/validate/deploy)
- **Science library:** `cazyme-fel` (Rust crate — FES reconstruction, cross-landscape analysis)
- **GPU shader:** Paper 50 FES Gaussian summation (11-14× acceleration, RMSD 1e-14)
- **Reference data:** pseudoSpore v1.7.0 (16 FEL landscapes, 190/190 checks, Derivation Anchoring)
- **Quality standard:** Derivation Anchoring v1.0 (zero magic numbers)

The product binary orchestrates these — it does not re-implement MD or FES.

## Graceful Degradation

| Tier | Requires | Capabilities |
|------|----------|-------------|
| 1 (standalone) | Nothing | Browse pseudoSpore, view pre-rendered figures, validate integrity |
| 2 (GPU compute) | toadStool + barraCuda | Live FES reconstruction, parameter sweeps, cross-landscape comparison |
| 3 (interactive) | + petalTongue | 3D molecular visualization, orbit navigation, FEL surface meshes |
| 4 (persistent) | + NestGate | Save/load sessions, share pseudoSpores |
| 5 (provenance) | + trio (rhizo/loam/sweet) | Sealed experiments, cryptographic lineage |
| 6 (full) | + Squirrel | AI-assisted analysis |

## Quick Start

```bash
# Not yet buildable — seed structure only
# Future:
cargo build --release
biomeos deploy --graph graphs/initiochem_full.toml
./target/release/initiochem
```

## Directory Structure

```
initioChem/
├── README.md              # This file
├── Cargo.toml             # Workspace definition
├── deny.toml              # ecoBin C-dep bans
├── LICENSE                # AGPL-3.0
├── graphs/                # biomeOS deploy graphs (6 tiers)
│   ├── initiochem_tower.toml
│   ├── initiochem_node.toml
│   ├── initiochem_viz.toml
│   ├── initiochem_nest.toml
│   ├── initiochem_provenance.toml
│   └── initiochem_full.toml
├── niches/                # biomeOS niche YAML
│   └── initiochem.yaml
├── specs/                 # Product specifications
│   └── PRODUCT_SPEC.md   # Composition contract, evolution roadmap
└── src/                   # Product source (future)
    └── lib.rs
```

## Upstream Specification

The full product specification lives in the source spring:
`hotSpring/specs/COMPCHEM_SPOREGARDEN_PRODUCT.md`

This is intentional — the spring owns the science, the garden owns the product.
The spec migrates here as the product materializes.

## References

- [sporeGarden org](https://github.com/sporeGarden)
- Source spring: `ecoPrimals/springs/hotSpring/`
- Product spec: `hotSpring/specs/COMPCHEM_SPOREGARDEN_PRODUCT.md`
- Deploy graphs (spring-side): `hotSpring/graphs/compchem_*.toml`
- Niche (spring-side): `hotSpring/niches/compchem-explorer.yaml`
- Upstream handoff: `infra/wateringHole/handoffs/HOTSPRING_COMPCHEM_PETALTONGUE_BARRACUDA_MAY28_2026.md`
- Composition onramp: `infra/wateringHole/GARDEN_COMPOSITION_ONRAMP.md`
- esotericWebb (reference pattern): `gardens/esotericWebb/`

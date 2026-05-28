# initioChem — Product Specification

**Version:** 0.1.0 (seed)
**Status:** Architectural draft — composition contract defined
**Pattern:** sporeGarden gen4 (BYOB, PrimalBridge, graceful degradation)
**Domain:** Computational chemistry — enzyme conformational landscapes
**Remote:** git@github.com:sporeGarden/initioChem.git

---

## Product Identity

| Field | Value |
|-------|-------|
| **Name** | initioChem |
| **Org** | [sporeGarden](https://github.com/sporeGarden) |
| **Source spring** | hotSpring (computational chemistry) |
| **Domain** | CAZyme conformational free energy landscapes |
| **Audience** | Computational chemists studying enzyme catalysis |
| **Complements** | esotericWebb (CRPG), helixVision, blueFish |
| **Pattern analog** | esotericWebb : ludoSpring :: initioChem : hotSpring |

## What It Does

An interactive 3D molecular research tool:

1. View conformational energy landscapes (FELs) as navigable 3D surfaces
2. Manipulate ring puckering in real-time (Cremer-Pople sphere)
3. Compare free vs enzyme-bound conformational preferences
4. Launch new GROMACS+PLUMED simulations and monitor live
5. Share validated results as pseudoSpores via NUCLEUS

## Why It Exists

The science is proven (pseudoSpore v1.7.0, 190/190 checks).
The pipeline is reproducible (nest-validate).
What's missing: **a human can't interact with it.** initioChem makes the
validated computation tangible — rotate a sugar ring in 3D, feel the energy
barriers, compare enzymes visually.

---

## Composition Contract

Fully specified in the source spring:
`hotSpring/specs/COMPCHEM_SPOREGARDEN_PRODUCT.md`

Key properties:
- **Particle profile:** proton-heavy (compute-dominated)
- **Mixed atomic:** node + dedicated tower (GPU compute + secure transport)
- **Bonding:** ionic lease (compute sessions) + covalent (persistent provenance)
- **Degradation:** 6 tiers (standalone → GPU → viz → storage → provenance → AI)

## Deploy Graphs

Six tiered graphs in `graphs/`, following the esotericWebb pattern:

| Graph | Tier | What it enables |
|-------|------|-----------------|
| `initiochem_tower.toml` | 1 | Secure transport, signed artifacts |
| `initiochem_node.toml` | 2 | GPU FES compute, shader dispatch |
| `initiochem_viz.toml` | 3 | 3D molecular viz + interaction |
| `initiochem_nest.toml` | 4 | Persistent storage, pseudoSpore archive |
| `initiochem_provenance.toml` | 5 | Sealed experiment sessions |
| `initiochem_full.toml` | 6 | AI-assisted analysis + full composition |

## Evolution Roadmap

| Phase | Target | Blocks On |
|-------|--------|-----------|
| 0.2 | FEL viewer (static data from pseudoSpore) | petalTongue FieldMap |
| 0.4 | Interactive compute (GPU FES, parameter sweeps) | petalTongue Phase 4 |
| 0.6 | Live simulation (GROMACS orchestration, streaming COLVAR) | biomeOS nucleus ingest |
| 1.0 | Collaborative + publication (share, compare, figures) | NC-5.live, cross-gate |

## Upstream Dependencies

| What | Owner | Status |
|------|-------|--------|
| petalTongue Phase 4 | petalTongue team | Specified, not implemented |
| barraCuda `math.tessellate.isosurface` | barraCuda team | Not yet implemented |
| biomeOS `nucleus ingest` CLI | biomeOS team | NC-1 gate |
| Derivation Anchoring lithoSpore gate | lithoSpore team | Handoff delivered |

## References

- Detailed spec: `hotSpring/specs/COMPCHEM_SPOREGARDEN_PRODUCT.md`
- Upstream handoff: `infra/wateringHole/handoffs/HOTSPRING_COMPCHEM_PETALTONGUE_BARRACUDA_MAY28_2026.md`
- Composition onramp: `infra/wateringHole/GARDEN_COMPOSITION_ONRAMP.md`
- esotericWebb pattern: `gardens/esotericWebb/`
- Science source: `hotSpring/pseudoSpore_hotSpring-CompChem-GuideStone_v1.7.0/`

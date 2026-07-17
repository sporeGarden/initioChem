// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "initiochem", version, about = "Interactive CompChem explorer")]
enum Cli {
    /// Display product version and composition status
    Status,
    /// List available capabilities at current degradation tier
    Capabilities,
    /// View a pseudoSpore: load envelope, show scope, modules, and validation status
    View {
        /// Path to pseudoSpore directory
        path: PathBuf,
    },
    /// Validate a pseudoSpore: run full envelope validation with integrity checks
    Validate {
        /// Path to pseudoSpore directory
        path: PathBuf,
        /// Output results as JSON
        #[arg(long)]
        json: bool,
    },
    /// Unpack a pseudoSpore tarball, validate, and display summary
    Unpack {
        /// Path to .tar.gz pseudoSpore tarball
        tarball: PathBuf,
        /// Output directory (pseudoSpore extracted inside)
        #[arg(long, default_value = ".")]
        output: PathBuf,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli {
        Cli::Status => cmd_status(),
        Cli::Capabilities => cmd_capabilities(),
        Cli::View { path } => cmd_view(&path),
        Cli::Validate { path, json } => cmd_validate(&path, json),
        Cli::Unpack { tarball, output } => cmd_unpack(&tarball, &output),
    }
}

fn cmd_status() {
    println!("initioChem v{}", env!("CARGO_PKG_VERSION"));
    println!("Status: alpha — pseudoSpore consumer wired");
    println!("Tier: 1 (standalone — no primals discovered)");
    println!();
    println!("Source spring: hotSpring");
    println!("Consumer API: pseudospore-core (PseudoSporeEnvelope)");
    println!("Composition: not yet wired (Tier 2+ requires NUCLEUS)");
}

fn cmd_capabilities() {
    println!("Tier 1 capabilities (standalone, no NUCLEUS):");
    println!("  initiochem view <dir>      — inspect pseudoSpore envelope");
    println!("  initiochem validate <dir>  — full envelope validation");
    println!("  initiochem unpack <tar.gz> — extract + validate tarball");
    println!();
    println!("Tier 2+ requires NUCLEUS primals (toadStool, barraCuda, petalTongue)");
    println!("Deploy: biomeos deploy --graph graphs/initiochem_full.toml");
}

fn cmd_view(path: &std::path::Path) {
    let envelope = match pseudospore_core::PseudoSporeEnvelope::load(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to load pseudoSpore at {}: {e}", path.display());
            std::process::exit(1);
        }
    };

    if let Some(scope) = &envelope.scope {
        println!(
            "pseudoSpore: {} v{}",
            scope.artifact.name, scope.artifact.version
        );
        if !scope.artifact.origin.is_empty() {
            println!("  origin: {}", scope.artifact.origin);
        }
        if !scope.artifact.artifact_type.is_empty() {
            println!("  type: {}", scope.artifact.artifact_type);
        }
        if !scope.module.is_empty() {
            println!("  modules: {}", scope.module.len());
            for m in &scope.module {
                let status = if m.status.is_empty() {
                    "unknown"
                } else {
                    &m.status
                };
                println!("    - {} ({})", m.name, status);
            }
        }
    }

    if let Some(validation) = &envelope.validation {
        let pass = validation
            .modules
            .iter()
            .filter(|m| m.status == "pass")
            .count();
        let total = validation.modules.len();
        println!("  validation: {pass}/{total} modules pass");
    }

    if let Some(ferment) = &envelope.ferment {
        println!("  ferment transcript: dataset_id={}", ferment.dataset_id);
        if !ferment.spring.is_empty() {
            println!("    spring: {}", ferment.spring);
        }
    }

    if !envelope.checksums.is_empty() {
        println!("  checksums: {} entries", envelope.checksums.len());
    }

    if !envelope.load_warnings.is_empty() {
        println!("  warnings during load: {}", envelope.load_warnings.len());
        for w in &envelope.load_warnings {
            println!("    - {w}");
        }
    }
}

fn cmd_validate(path: &std::path::Path, json: bool) {
    let envelope = match pseudospore_core::PseudoSporeEnvelope::load(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to load pseudoSpore at {}: {e}", path.display());
            std::process::exit(1);
        }
    };

    let result = envelope.validate();

    if json {
        let output = serde_json::json!({
            "valid": result.valid,
            "checksums_verified": result.checksums_verified,
            "checksums_failed": result.checksums_failed,
            "errors": result.errors,
            "warnings": result.warnings,
            "scope": result.scope.as_ref().map(|s| {
                serde_json::json!({
                    "name": s.artifact.name,
                    "version": s.artifact.version,
                    "type": s.artifact.artifact_type,
                })
            }),
        });
        match serde_json::to_string_pretty(&output) {
            Ok(json) => println!("{json}"),
            Err(e) => eprintln!("Failed to serialize JSON: {e}"),
        }
    } else {
        if result.valid {
            println!("VALID: {}", path.display());
        } else {
            println!("INVALID: {}", path.display());
        }

        if let Some(scope) = &result.scope {
            println!(
                "  {} v{} ({})",
                scope.artifact.name, scope.artifact.version, scope.artifact.artifact_type
            );
        }

        println!(
            "  checksums: {} verified, {} failed",
            result.checksums_verified, result.checksums_failed
        );

        if !result.errors.is_empty() {
            println!("  errors:");
            for e in &result.errors {
                println!("    ERROR: {e}");
            }
        }

        if !result.warnings.is_empty() {
            println!("  warnings:");
            for w in &result.warnings {
                println!("    WARN: {w}");
            }
        }
    }

    if !result.valid {
        std::process::exit(1);
    }
}

fn cmd_unpack(tarball: &std::path::Path, output: &std::path::Path) {
    if !tarball.is_file() {
        eprintln!("Not a file: {}", tarball.display());
        std::process::exit(1);
    }

    println!("Unpacking: {}", tarball.display());

    let extracted = match pseudospore_core::extract_tarball(tarball, output) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to extract: {e}");
            std::process::exit(1);
        }
    };

    println!("Extracted to: {}", extracted.display());

    println!("Validating envelope...");
    let envelope = match pseudospore_core::PseudoSporeEnvelope::load(&extracted) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to load extracted pseudoSpore: {e}");
            std::process::exit(1);
        }
    };

    let result = envelope.validate();
    if result.valid {
        if let Some(scope) = &result.scope {
            println!(
                "VALID: {} v{} ({} checksums verified)",
                scope.artifact.name, scope.artifact.version, result.checksums_verified
            );
        } else {
            println!("VALID: {} checksums verified", result.checksums_verified);
        }
    } else {
        println!("INVALID:");
        for e in &result.errors {
            println!("  ERROR: {e}");
        }
        std::process::exit(1);
    }

    for w in &result.warnings {
        println!("  WARN: {w}");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::fs;

    const SCOPE: &str = r#"
[artifact]
name = "test-compchem"
version = "0.1.0"
type = "pseudoSpore"
origin = "ecoPrimals/springs/hotSpring"
"#;

    #[test]
    fn load_and_validate_pseudospore() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();

        fs::write(root.join("scope.toml"), SCOPE).expect("scope");
        fs::create_dir_all(root.join("outputs")).expect("outputs");
        fs::write(root.join("README.md"), "# Test CompChem pseudoSpore\n").expect("readme");

        let envelope = pseudospore_core::PseudoSporeEnvelope::load(root).expect("load");
        assert!(envelope.scope.is_some());
        assert_eq!(
            envelope.scope.as_ref().expect("scope").artifact.name,
            "test-compchem"
        );

        let result = envelope.validate();
        assert!(
            result.valid,
            "basic envelope should be valid: {:?}",
            result.errors
        );
    }

    #[test]
    fn pack_unpack_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let spore = dir.path().join("compchem_v0.1.0");
        fs::create_dir_all(spore.join("outputs")).expect("outputs");
        fs::create_dir_all(spore.join("receipts")).expect("receipts");

        fs::write(spore.join("scope.toml"), SCOPE).expect("scope");
        fs::write(spore.join("README.md"), "# Comp chem spore\n").expect("readme");

        let data = b"theta,energy\n0,5.2\n30,3.1\n60,0.0\n";
        fs::write(spore.join("outputs/fes.dat"), data).expect("fes");

        let hash = blake3::hash(data).to_hex().to_string();
        let checksums = format!("{hash}  outputs/fes.dat\n");
        fs::write(spore.join("receipts/checksums.blake3"), &checksums).expect("checksums");

        let out = tempfile::tempdir().expect("out");
        let tarball = out.path().join("test.tar.gz");
        let tb_hash = pseudospore_core::create_tarball(
            &spore,
            &tarball,
            pseudospore_core::tarball::DEFAULT_EXTERNAL_PATTERNS,
        )
        .expect("pack");
        assert!(!tb_hash.is_empty());

        let extract = tempfile::tempdir().expect("extract");
        let extracted =
            pseudospore_core::extract_tarball(&tarball, extract.path()).expect("unpack");

        let envelope = pseudospore_core::PseudoSporeEnvelope::load(&extracted).expect("load");
        let result = envelope.validate();

        assert!(
            result.valid,
            "round-trip should validate: {:?}",
            result.errors
        );
        assert!(result.checksums_verified > 0, "checksums verified");
        assert_eq!(result.checksums_failed, 0);
    }
}

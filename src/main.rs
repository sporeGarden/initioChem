// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

use clap::Parser;

#[derive(Parser)]
#[command(name = "initiochem", version, about = "Interactive CompChem explorer")]
enum Cli {
    /// Display product version and composition status
    Status,
    /// List available capabilities at current degradation tier
    Capabilities,
    /// View a pseudoSpore directory (Tier 1 — no primals needed)
    View {
        /// Path to pseudoSpore directory
        path: std::path::PathBuf,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli {
        Cli::Status => {
            println!("initioChem v{}", env!("CARGO_PKG_VERSION"));
            println!("Status: seed (pre-alpha)");
            println!("Tier: 1 (standalone — no primals discovered)");
            println!();
            println!("Source spring: hotSpring");
            println!("Science artifact: pseudoSpore_hotSpring-CompChem-GuideStone_v1.7.0");
            println!("Composition: not yet wired");
        }
        Cli::Capabilities => {
            println!("Tier 1 capabilities (standalone, no NUCLEUS):");
            println!("  compchem.fel.view        — display pre-computed FEL from pseudoSpore");
            println!("  compchem.structure.display — static PDB structure info");
            println!();
            println!("Tier 2+ requires NUCLEUS primals (toadStool, barraCuda, petalTongue)");
            println!("Deploy: biomeos deploy --graph graphs/initiochem_full.toml");
        }
        Cli::View { path } => {
            if !path.exists() {
                eprintln!("Error: pseudoSpore directory not found: {}", path.display());
                std::process::exit(1);
            }
            let scope = path.join("scope.toml");
            if scope.exists() {
                println!("pseudoSpore: {}", path.display());
                match std::fs::read_to_string(&scope) {
                    Ok(content) => println!("{content}"),
                    Err(e) => eprintln!("Failed to read scope.toml: {e}"),
                }
            } else {
                eprintln!("Not a valid pseudoSpore directory (missing scope.toml)");
                std::process::exit(1);
            }
        }
    }
}

use anyhow::Result;
use crate::cli::DocsArgs;

pub fn run(args: DocsArgs) -> Result<()> {
    // cargo bundles docs/ next to the repo; at runtime just point there.
    const DOCS: &[&str] = &[
        "docs/README.md",
        "docs/HPC_OVERVIEW.md",
        "docs/CLUSTER_SPECS.md",
        "docs/GATED_DOCS_INDEX.md",
        "docs/SOURCES.md",
    ];
    if args.list {
        for d in DOCS {
            println!("{d}");
        }
        return Ok(());
    }
    println!("TN Tech HPC — bundled docs (see docs/ in repo):\n");
    for d in DOCS {
        println!("  - {d}");
    }
    println!("\nTip: most how-to articles live behind TN Tech SSO (see GATED_DOCS_INDEX.md).");
    Ok(())
}

use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::{Parser, Subcommand};
use uniffi_bindgen::BindgenCrateConfigSupplier;

// Structs to help our cmdline parsing. Note that docstrings below form part
// of the "help" output.

/// Scaffolding and bindings generator for Rust
#[derive(Parser)]
#[clap(name = "uniffi-bindgen-dart")]
#[clap(version)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate foreign language bindings
    Generate {
        /// Directory in which to write generated files. Default is same folder as .udl file.
        #[clap(long, short)]
        out_dir: Option<Utf8PathBuf>,

        /// Do not try to format the generated bindings.
        #[clap(long, short)]
        no_format: bool,

        /// Path to optional uniffi config file. This config is merged with the `uniffi.toml` config present in each crate, with its values taking precedence.
        #[clap(long, short)]
        config: Option<Utf8PathBuf>,

        /// Deprecated
        ///
        /// This used to signal that a source file is a library rather than a UDL file.
        /// Nowadays, UniFFI will auto-detect this.
        #[clap(long = "library")]
        _library_mode: bool,

        /// When `--library` is passed, only generate bindings for one crate.
        /// When `--library` is not passed, use this as the crate name instead of attempting to
        /// locate and parse Cargo.toml.
        #[clap(long = "crate")]
        crate_name: Option<String>,

        /// Path to the UDL file, or cdylib if `library-mode` is specified
        source: Utf8PathBuf,

        /// Whether we should exclude dependencies when running "cargo metadata".
        /// This will mean external types may not be resolved if they are implemented in crates
        /// outside of this workspace.
        /// This can be used in environments when all types are in the namespace and fetching
        /// all sub-dependencies causes obscure platform specific problems.
        #[clap(long)]
        metadata_no_deps: bool,
    },
}

pub fn run_main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            out_dir,
            no_format,
            config,
            source,
            crate_name,
            metadata_no_deps,
            ..
        } => {
            let out_dir = out_dir.unwrap_or_else(|| {
                source
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| Utf8PathBuf::from("."))
            });

            let is_library = is_library_file(&source);

            if is_library {
                generate_library_mode(
                    &source,
                    config.as_deref(),
                    &out_dir,
                    crate_name,
                    !no_format,
                    metadata_no_deps,
                )
            } else {
                generate_udl_mode(&source, config.as_deref(), &out_dir, !no_format)
            }
        }
    }
}

fn is_library_file(path: &Utf8Path) -> bool {
    let extension = path.extension().unwrap_or("");
    matches!(extension, "dll" | "so" | "dylib")
}

fn generate_library_mode(
    library_path: &Utf8Path,
    config_path: Option<&Utf8Path>,
    out_dir: &Utf8Path,
    crate_filter: Option<String>,
    format: bool,
    _metadata_no_deps: bool,
) -> Result<()> {
    println!(
        "Generating Dart bindings in library mode from: {}",
        library_path
    );
    println!("Output directory: {}", out_dir);

    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("Failed to create output directory: {}", out_dir))?;

    let config_supplier: Box<dyn BindgenCrateConfigSupplier> =
        Box::new(uniffi_bindgen::EmptyCrateConfigSupplier {});

    uniffi_bindgen::library_mode::generate_bindings(
        library_path,
        crate_filter,
        &crate::gen::DartBindingGenerator {},
        config_supplier.as_ref(),
        config_path,
        out_dir,
        format,
    )?;

    println!("Dart bindings generated successfully!");
    Ok(())
}

fn generate_udl_mode(
    udl_path: &Utf8Path,
    config_path: Option<&Utf8Path>,
    out_dir: &Utf8Path,
    format: bool,
) -> Result<()> {
    println!("Generating Dart bindings from UDL: {}", udl_path);
    println!("Output directory: {}", out_dir);

    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("Failed to create output directory: {}", out_dir))?;

    // Try to find a compiled library for hybrid mode (UDL + proc-macros)
    let library_path = find_library_for_udl(udl_path);

    uniffi_bindgen::generate_external_bindings(
        &crate::gen::DartBindingGenerator {},
        udl_path,
        config_path,
        Some(out_dir),
        library_path.as_deref(),
        None,
        format,
    )?;

    println!("Dart bindings generated successfully!");
    Ok(())
}

fn find_library_for_udl(udl_path: &Utf8Path) -> Option<Utf8PathBuf> {
    // Look for a compiled library in typical build locations
    // Priority: 1. Same directory as UDL 2. target/release 3. target/debug

    let udl_dir = udl_path.parent()?;
    let udl_stem = udl_path.file_stem()?;

    // Convert snake_case UDL name to crate name format
    let lib_name = udl_stem.replace('_', "-");

    // Check same directory as UDL
    for ext in &["dll", "so", "dylib"] {
        let lib_path = udl_dir.join(format!("{}.{}", lib_name, ext));
        if lib_path.exists() {
            return Some(lib_path);
        }
    }

    // Try to find Cargo.toml and check target directories
    let mut current = udl_dir;
    while let Some(parent) = current.parent() {
        let cargo_toml = parent.join("Cargo.toml");
        if cargo_toml.exists() {
            // Found project root, check target directories
            for profile in &["release", "debug"] {
                for ext in &["dll", "so", "dylib"] {
                    let lib_path = parent.join("target").join(profile).join(format!("{}.{}", lib_name, ext));
                    if lib_path.exists() {
                        println!("Found compiled library for hybrid mode: {}", lib_path);
                        return Some(lib_path);
                    }
                }
            }
            break;
        }
        current = parent;
    }

    None
}

#[cfg(feature = "build")]
mod build;
#[cfg(feature = "bindgen-tests")]
pub mod testing;
#[cfg(feature = "build")]
pub use build::generate_scaffolding;

pub mod gen;

#[cfg(feature = "cli")]
mod cli;

pub use uniffi_dart_macro::*;

#[cfg(feature = "cli")]
pub fn uniffi_bindgen_dart_main() {
    if let Err(e) = cli::run_main() {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}

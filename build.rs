use mopro_ffi::app_config::{build_uniffi, install_archs};

fn main() {
    rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
    install_archs();
    build_uniffi();
}

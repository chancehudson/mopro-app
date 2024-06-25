use mopro_ffi::app_config::{build_uniffi, install_archs};

fn main() {
    rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
    install_archs();
    // uniffi is built/linked in the mopro-ffi build, no need to do it here
    build_uniffi();
}

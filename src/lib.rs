use mopro_ffi::app;
use mopro_ffi::circom::WtnsFn;
use std::path::Path;

rust_witness::witness!(multiplier3);

pub fn circuit_data(zkey_path: &str) -> Result<WtnsFn, MoproError> {
    let name = Path::new(zkey_path).file_stem().unwrap();
    match name.to_str().unwrap() {
        "multiplier3_final" => Ok(multiplier3_witness),
        _ => Err(MoproError::CircomError("Unknown circuit name".to_string())),
    }
}

app!();

use mopro_ffi::{app, WtnsFn};

rust_witness::witness!(multiplier3);

app!();

fn circuit_data(name: &str) -> Result<WtnsFn, MoproError> {
    match name {
        "multiplier3_final" => Ok(multiplier3_witness),
        _ => Err(MoproError::CircomError("Unknown circuit name".to_string())),
    }
}

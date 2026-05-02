use ninja_build_rs::{Result, nightly::Nightly};

fn main() -> Result<()> {
    let ac = autocfg::new();
    ac.emit_unstable_feature("bool_to_result");
    ac.emit_unstable_feature("never_type");
    Ok(())
}

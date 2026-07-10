use build_safely::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(bool_to_result, &allowed_features);
    ac.emit_unstable_feature(never_type, &allowed_features);
    Ok(())
}

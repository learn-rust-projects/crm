use anyhow::Result;

fn main() -> Result<()> {
    crm_common::build::ProtoConfig::new("pb")
        .add_proto("crm/message")
        .is_build(false)
        .build()?;
    Ok(())
}

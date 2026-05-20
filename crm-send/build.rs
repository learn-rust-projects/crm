use anyhow::Result;

fn main() -> Result<()> {
    crm_common::build::ProtoConfig::new("pb")
        .add_protos(["notification/messages", "notification/rpc"])
        .is_build(false)
        .build()?;
    println!("Notification protobufs built successfully");
    Ok(())
}

use anyhow::Result;

fn main() -> Result<()> {
    crm_common::build::ProtoConfig::new("pb")
        .add_protos(["metadata/messages", "metadata/rpc"])
        // Example: add message attributes
        // .message_attribute("metadata.Content", "#[derive(sqlx::FromRow)]")
        .is_build(false)
        .build()?;
    println!("Metadata protobufs built successfully");
    Ok(())
}

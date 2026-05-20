use anyhow::Result;

fn main() -> Result<()> {
    crm_common::build::ProtoConfig::new("pb")
        .add_protos(["crm/messages", "crm/rpc"])
        .message_attribute("crm.WelcomeRequest", "#[derive(derive_builder::Builder)]")
        .message_attribute("crm.RecallRequest", "#[derive(derive_builder::Builder)]")
        .message_attribute("crm.RemindRequest", "#[derive(derive_builder::Builder)]")
        .field_attribute(
            "crm.WelcomeRequest.content_ids",
            r#"#[builder(setter(each(name = "content_id", into)))]"#,
        )
        .field_attribute(
            "crm.RecallRequest.content_ids",
            r#"#[builder(setter(each(name = "content_id", into)))]"#,
        )
        .is_build(false)
        .build()?;
    println!("Crm protobufs built successfully");
    Ok(())
}

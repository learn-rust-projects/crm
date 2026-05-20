use anyhow::Result;

fn main() -> Result<()> {
    crm_common::build::ProtoConfig::new("pb")
        .add_protos(["user-stats/messages", "user-stats/rpc"])
        .message_attribute("user_stats.User", "#[derive(sqlx::FromRow)]")
        .message_attribute("user_stats.User", "#[derive(derive_builder::Builder)]")
        .field_attribute("user_stats.User.email", "#[builder(setter(into))]")
        .message_attribute(
            "user_stats.QueryRequest",
            "#[derive(derive_builder::Builder)]",
        )
        .field_attribute(
            "user_stats.QueryRequest.timestamps",
            r#"#[builder(setter(each(name="timestamp", into)))]"#,
        )
        .field_attribute(
            "user_stats.QueryRequest.ids",
            r#"#[builder(setter(each(name="id", into)))]"#,
        )
        .message_attribute(
            "user_stats.RawQueryRequest",
            "#[derive(derive_builder::Builder)]",
        )
        .field_attribute(
            "user_stats.RawQueryRequest.query",
            "#[builder(setter(into))]",
        )
        .message_attribute("user_stats.TimeQuery", "#[derive(derive_builder::Builder)]")
        .field_attribute(
            "user_stats.TimeQuery.before",
            "#[builder(setter(into,strip_option))]",
        )
        .field_attribute(
            "user_stats.TimeQuery.after",
            "#[builder(setter(into,strip_option))]",
        )
        .message_attribute("user_stats.IdQuery", "#[derive(derive_builder::Builder)]")
        .field_attribute(
            "user_stats.IdQuery.ids",
            r#"#[builder(setter(each(name="id", into)))]"#,
        )
        .is_build(false)
        .build()?;
    Ok(())
}

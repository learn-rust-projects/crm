use anyhow::Result;
fn main() -> Result<()> {
    // let builder = tonic_prost_build::configure()
    //     .message_attribute("user_stats.User", "#[derive(sqlx::FromRow)]")
    //     .message_attribute("user_stats.User",
    // "#[derive(derive_builder::Builder)]")     .field_attribute("email",
    // "#[builder(setter(into))]")     .message_attribute(
    //         "user_stats.QueryRequest",
    //         "#[derive(derive_builder::Builder)]",
    //     )
    //     .field_attribute(
    //         "timestamps",
    //         r#"#[builder(setter(each(name="timestamp", into)))]"#,
    //     )
    //     .message_attribute(
    //         "user_stats.RawQueryRequest",
    //         "#[derive(derive_builder::Builder)]",
    //     )
    //     .field_attribute("query", "#[builder(setter(into))]")
    //     .message_attribute("user_stats.TimeQuery",
    // "#[derive(derive_builder::Builder)]")     .field_attribute("before",
    // "#[builder(setter(into,strip_option))]")     .field_attribute("after",
    // "#[builder(setter(into,strip_option))]")     .message_attribute("
    // user_stats.IdQuery", "#[derive(derive_builder::Builder)]")
    //     .field_attribute("ids", r#"#[builder(setter(each(name="id", into)))]"#);
    // build_proto::BuildProto::new(builder, "pb", &["user-stats/messages",
    // "user-stats/rpc"])     .build_proto()?;
    Ok(())
}
#[allow(dead_code)]
mod build_proto {
    use std::{
        fs::{self, OpenOptions},
        io::Write,
    };

    use anyhow::Result;
    use tonic_prost_build::Builder;
    pub struct BuildProto<'a> {
        mod_name: &'a str,
        proto_rs_files: &'a [&'a str],
        builder: Builder,
    }
    impl<'a> BuildProto<'a> {
        pub fn new(builder: Builder, mod_name: &'a str, proto_rs_files: &'a [&'a str]) -> Self {
            Self {
                mod_name,
                proto_rs_files,
                builder,
            }
        }
        pub fn build_proto(self) -> Result<()> {
            fs::create_dir_all(format!("src/{}", self.mod_name))?;
            self.builder
                .out_dir(format!("src/{}", self.mod_name))
                .compile_protos(
                    &create_compile_protos(self.proto_rs_files),
                    &["../protos".to_string()],
                )?;
            create_pb_mod_file(self.mod_name, self.proto_rs_files)?;
            update_lib_rs(self.mod_name)?;
            Ok(())
        }
    }

    fn create_pb_mod_file(mod_name: &str, _proto_rs_files: &[&str]) -> Result<()> {
        let mut mod_file: fs::File = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(format!("src/{}/mod.rs", mod_name))?;
        let files = fs::read_dir(format!("src/{}", mod_name))?;
        for file in files {
            let file = file?;
            let file_name: String = file.file_name().to_string_lossy().to_string();
            if let Some(file_name) = file_name.strip_suffix(".rs")
                && file_name != "mod"
            {
                writeln!(mod_file, "pub mod {};", file_name)?;
            }
        }
        mod_file.flush()?;
        Ok(())
    }

    fn create_compile_protos(proto_rs_files: &[&str]) -> Vec<String> {
        proto_rs_files
            .iter()
            .map(|proto| format!("../protos/{}.proto", proto))
            .collect::<Vec<_>>()
    }

    fn update_lib_rs(mod_name: &str) -> Result<()> {
        let file = "src/lib.rs";
        if fs::metadata(file).is_ok()
            && fs::read_to_string(file)?.contains(&format!("pub mod {};", mod_name))
        {
            return Ok(());
        }
        let mut f = OpenOptions::new().create(true).append(true).open(file)?;
        writeln!(f, "pub mod {};", mod_name)?;
        f.flush()?;
        Ok(())
    }
}

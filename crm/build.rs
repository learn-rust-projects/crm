use anyhow::Result;

fn main() -> Result<()> {
    // build_proto::BuildProto::new("pb", &["crm"]).build_proto()?;
    Ok(())
}
#[allow(dead_code)]
mod build_proto {
    use std::{
        fs::{self, OpenOptions},
        io::Write,
    };

    use anyhow::Result;
    pub struct BuildProto<'a> {
        mod_name: &'a str,
        proto_rs_files: &'a [&'a str],
    }
    impl<'a> BuildProto<'a> {
        pub fn new(mod_name: &'a str, proto_rs_files: &'a [&'a str]) -> Self {
            Self {
                mod_name,
                proto_rs_files,
            }
        }
        pub fn build_proto(&self) -> Result<()> {
            fs::create_dir_all(format!("src/{}", self.mod_name))?;
            let builder = tonic_prost_build::configure();
            builder
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

    fn create_pb_mod_file(mod_name: &str, proto_rs_files: &[&str]) -> Result<()> {
        let file = format!("src/{}/mod.rs", mod_name);
        // check if mod.rs exists
        if fs::metadata(&file).is_ok() && fs::read_to_string(&file)?.trim() != String::new() {
            return Ok(());
        }
        // create mod.rs
        let mut file = fs::File::create(format!("src/{}/mod.rs", mod_name))?;
        for proto in proto_rs_files {
            writeln!(file, "pub mod {};", proto)?;
        }
        file.flush()?;
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
        let mut f = OpenOptions::new().append(true).open(file)?;
        writeln!(f, "pub mod {};", mod_name)?;
        f.flush()?;
        Ok(())
    }
}

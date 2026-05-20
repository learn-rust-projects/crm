//! Proto build utilities for generating Rust code from protobuf definitions.
//!
//! This module provides a reusable way to compile protobuf files and generate
//! tonic-compatible Rust code with customizable attributes.

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use anyhow::Result;
use tonic_prost_build::Builder;

/// Configures protobuf compilation with custom attributes.
pub struct ProtoConfig {
    builder: Builder,
    protos: Vec<String>,
    proto_paths: Vec<String>,
    out_dir: String,
    mod_name: String,
    src_dir: String,
    is_build: bool,
}

impl ProtoConfig {
    /// Creates a new ProtoConfig with default settings.
    pub fn new(mod_name: impl Into<String>) -> Self {
        Self {
            builder: tonic_prost_build::configure(),
            protos: Vec::new(),
            proto_paths: vec!["../protos".to_string()],
            out_dir: String::new(),
            mod_name: mod_name.into(),
            src_dir: "src".to_string(),
            is_build: true,
        }
    }

    /// Sets whether to execute the proto build (default: true).
    pub fn is_build(mut self, is_build: bool) -> Self {
        self.is_build = is_build;
        self
    }

    /// Adds a proto file to compile (without .proto extension).
    pub fn add_proto(mut self, proto: impl Into<String>) -> Self {
        self.protos.push(proto.into());
        self
    }

    /// Adds multiple proto files to compile.
    pub fn add_protos(mut self, protos: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for proto in protos {
            self.protos.push(proto.into());
        }
        self
    }

    /// Sets the include paths for protobuf imports.
    pub fn proto_paths(mut self, paths: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.proto_paths = paths.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the output directory relative to src.
    pub fn out_dir(mut self, out_dir: impl Into<String>) -> Self {
        self.out_dir = out_dir.into();
        self
    }

    /// Sets the source directory (default: "src").
    pub fn src_dir(mut self, src_dir: impl Into<String>) -> Self {
        self.src_dir = src_dir.into();
        self
    }

    /// Adds a message attribute (applied to the message struct).
    pub fn message_attribute<P: AsRef<str>, A: AsRef<str>>(
        mut self,
        full_name: P,
        attr: A,
    ) -> Self {
        self.builder = self.builder.message_attribute(full_name, attr);
        self
    }

    /// Adds a field attribute.
    pub fn field_attribute<P: AsRef<str>, A: AsRef<str>>(mut self, full_name: P, attr: A) -> Self {
        self.builder = self.builder.field_attribute(full_name, attr);
        self
    }

    /// Builds the protobuf files and generates Rust code.
    pub fn build(self) -> Result<()> {
        if !self.is_build {
            return Ok(());
        }

        let proto_files = self
            .protos
            .iter()
            .map(|p| format!("../protos/{}.proto", p))
            .collect::<Vec<_>>();

        let out_dir = if self.out_dir.is_empty() {
            format!("{}/{}", self.src_dir, self.mod_name)
        } else {
            format!("{}/{}", self.src_dir, self.out_dir)
        };

        fs::create_dir_all(&out_dir)?;

        self.builder
            .out_dir(&out_dir)
            .compile_protos(&proto_files, &self.proto_paths)?;

        create_pb_mod_file(Path::new(&out_dir))?;
        update_lib_rs(&self.src_dir, &self.mod_name)?;

        Ok(())
    }
}

fn create_pb_mod_file(out_dir: &Path) -> Result<()> {
    let mod_file_path = out_dir.join("mod.rs");
    let mut mod_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&mod_file_path)?;

    let files = fs::read_dir(out_dir)?;
    for file in files {
        let file = file?;
        let file_name = file.file_name().to_string_lossy().to_string();
        if let Some(name) = file_name.strip_suffix(".rs")
            && name != "mod"
        {
            writeln!(mod_file, "pub mod {};", name)?;
        }
    }
    mod_file.flush()?;
    Ok(())
}

fn update_lib_rs(src_dir: &str, mod_name: &str) -> Result<()> {
    let file = format!("{}/lib.rs", src_dir);
    let content = fs::read_to_string(&file).unwrap_or_default();

    if content.contains(&format!("pub mod {};", mod_name)) {
        return Ok(());
    }

    let mut f = OpenOptions::new().create(true).append(true).open(&file)?;
    writeln!(f, "pub mod {};", mod_name)?;
    f.flush()?;
    Ok(())
}

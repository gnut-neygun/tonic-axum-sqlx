use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    // Take care editing the path of remove_dir_all. Do not execute build when editing it
    let _ = fs::remove_dir_all("./generated/rust");
    fs::create_dir_all("./generated/rust").unwrap();
    let out_dir = Path::new("./generated/rust");
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir(out_dir)
        .file_descriptor_set_path(out_dir.join("object_api_descriptor.bin"))
        .compile(&["proto/service.proto"], &["proto"])?;
    Ok(())
}

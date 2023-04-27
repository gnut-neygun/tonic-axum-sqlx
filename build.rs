use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    // Take care editing the path of remove_dir_all. Do not execute build when editing it
    fs::remove_dir_all("./src/generated").unwrap();
    fs::create_dir_all("./src/generated").unwrap();
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("./src/generated")
        .compile(&["proto/service.proto"], &["proto"])?;
    Ok(())
}
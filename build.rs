fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    // Run tonic-build to generate rust code
    tonic_build::configure().out_dir("./generated/rust").
        compile(&["proto/service.proto"], &["proto"])?;
    Ok(())
}
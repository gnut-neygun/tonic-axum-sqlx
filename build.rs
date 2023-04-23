use std::{env};
use std::path::Path;
use std::process::Command;

// Print a cargo build warning
// We need this because by default, cargo hide build.rs stdout/stderr
macro_rules! print_warning {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn resolve_relative_path(path: String) -> String {
    let path = Path::new(&path);
    let path = path.canonicalize().expect(format!("Can not resolve path: {}", path.display())
        .as_str());
    let path = path.to_str().unwrap().to_string();
    path
}

fn compile_protobuf(path: &str) {
    let mut cmd = Command::new("protoc");
    cmd.arg(path)
        .arg("-I=./proto")
        .arg("--python_out=./generated/python");
    let output = cmd.output().expect("protoc exited with error");
    let output = String::from_utf8(output.stderr).expect("Can not parse stderror of protoc");
    if !output.is_empty() {
        print_warning!("Protoc compiling error: {}", output);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=.cargo/config.toml");
    let mut path = env::var("PATH").map_or(String::new(), |path| path);
    let protoc_path = resolve_relative_path(env::var("PROTOC_PATH").expect("PROTOC_PATH not set"));
    path.push(':');
    path.push_str(&protoc_path);
    let protoc_gen_openapi_path = resolve_relative_path(
        env::var("PROTOC_GEN_OPENAPI_PATH").expect("PROTOC_GEN_OPENAPI_PATH not set"),
    );
    if protoc_gen_openapi_path != protoc_path {
        path.push(':');
        path.push_str(&protoc_gen_openapi_path);
    }
    env::set_var("PATH", path);
    // Run protoc to generate openapi schema
    let output = Command::new("protoc")
        .args(&["proto/service.proto", "-I=./proto", "--openapi_out=./generated"])
        .output()
        .expect("protoc exited with error");
    let output = String::from_utf8(output.stderr).expect("Can not parse stderror of protoc");
    // Print the standard error of protoc, if any
    // We need this because if protoc can't find protoc-gen-openapi, it will print an error message
    // but silently succeed
    if !output.is_empty() {
        print_warning!("Unable to generate openapi schema: {}", output);
    }

    compile_protobuf("proto/google/api/annotations.proto");
    compile_protobuf("proto/google/api/http.proto");
    // Run tonic-build to generate rust code
    tonic_build::configure().out_dir("./generated").
        compile(&["proto/service.proto"], &["proto"])?;
    Ok(())
}
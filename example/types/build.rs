fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = "../../proto/src/hello.proto";
    println!("cargo:rerun-if-changed={}", proto_path);

    // Create a placeholder lib.rs if it doesn't exist
    let lib_path = "src/lib.rs";
    if !std::path::Path::new(lib_path).exists() {
        std::fs::write(lib_path, "// Auto-generated\n")?;
    }

    prost_build::Config::new()
        .out_dir("src/")
        .compile_protos(&[proto_path], &["../../proto/src"])
        .unwrap();

    Ok(())
}

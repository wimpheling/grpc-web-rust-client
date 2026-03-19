fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_paths = [
        "../proto/hello.proto",
        "../proto/arithmetic_progression_streaming.proto",
    ];
    for path in &proto_paths {
        println!("cargo:rerun-if-changed={}", path);
    }

    // Create a placeholder lib.rs if it doesn't exist
    let lib_path = "src/lib.rs";
    if !std::path::Path::new(lib_path).exists() {
        std::fs::write(lib_path, "// Auto-generated\n")?;
    }

    prost_build::Config::new()
        .out_dir("src/")
        .compile_protos(&proto_paths, &["../proto"])
        .unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_paths = [
        "../proto/hello.proto",
        "../proto/arithmetic_progression_streaming.proto",
    ];
    for path in &proto_paths {
        println!("cargo:rerun-if-changed={}", path);
    }

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/")
        .compile_protos(&proto_paths, &["../proto"])
        .unwrap();

    Ok(())
}

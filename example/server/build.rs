fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = "../proto/hello.proto";
    println!("cargo:rerun-if-changed={}", proto_path);

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/")
        .compile_protos(&[proto_path], &["../proto"])
        .unwrap();

    Ok(())
}

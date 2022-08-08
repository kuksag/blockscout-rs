fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--openapiv2_out=../docs/")
        .compile(
            &["src/proto/server_description.proto"],
            &[
                "src/proto",
                "src/proto/googleapis",
                "src/proto/grpc-gateway",
            ],
        )?;
    Ok(())
}

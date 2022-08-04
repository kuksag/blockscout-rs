fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &["src/proto/server_description.proto"],
        &[
            "src/proto/",
            "src/proto/googleapis/",
            "src/proto/grpc-gateway/",
        ],
    )?;
    Ok(())
}

use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("protoc")
        .args([
            "src/proto/server_description.proto",
            "-I=src/proto",
            "-I=src/proto/googleapis/",
            "-I=src/proto/grpc-gateway/",
            "--openapiv2_out=.",
        ])
        .status()
        .expect("Failed to execute process");

    if !output.success() {
        panic!("Failed to generate OpenAPI v2");
    }

    Ok(())
}

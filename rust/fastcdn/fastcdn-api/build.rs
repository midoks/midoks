fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./src/rpc/protos/ping.proto")?;
    tonic_build::compile_protos("./src/rpc/protos/hello.proto")?;
    Ok(())
}

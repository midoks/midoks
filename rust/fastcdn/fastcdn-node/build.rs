fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false) // 只生成客户端代码
        .compile(
            &[
                "../fastcdn-api/src/rpc/protos/hello.proto",
                "../fastcdn-api/src/rpc/protos/ping.proto",
            ],
            &["../fastcdn-api/src/rpc/protos"],
        )?;
    Ok(())
}
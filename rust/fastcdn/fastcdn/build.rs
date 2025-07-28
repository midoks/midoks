fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../fastcdn-api/src/rpc/protos";
    let hello_proto = format!("{}/hello.proto", proto_dir);
    let ping_proto = format!("{}/ping.proto", proto_dir);
    
    println!("cargo:rerun-if-changed={}", hello_proto);
    println!("cargo:rerun-if-changed={}", ping_proto);
    
    // 检查文件是否存在
    if !std::path::Path::new(&hello_proto).exists() {
        panic!("hello.proto not found at: {}", hello_proto);
    }
    if !std::path::Path::new(&ping_proto).exists() {
        panic!("ping.proto not found at: {}", ping_proto);
    }
    
    let out_dir = std::env::var("OUT_DIR")?;
    
    tonic_build::configure()
        .build_server(false) // 只生成客户端代码
        .out_dir(&out_dir)
        .compile(
            &[hello_proto, ping_proto],
            &[proto_dir],
        )?;
    
    Ok(())
}
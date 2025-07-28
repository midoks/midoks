fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "./src/rpc/protos";
    
    // 自动扫描protos目录下的所有.proto文件
    let mut proto_files = Vec::new();
    
    for entry in std::fs::read_dir(proto_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("proto") {
            proto_files.push(path.to_string_lossy().to_string());
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    
    if !proto_files.is_empty() {
        tonic_build::configure()
            .build_server(true)
            .build_client(false)
            .compile_protos(&proto_files, &[proto_dir])?;
    }
    
    Ok(())
}

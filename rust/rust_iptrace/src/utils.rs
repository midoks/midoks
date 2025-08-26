use std::net::IpAddr;
use tokio::process::Command;

pub async fn reverse_dns_lookup(ip: IpAddr) -> Option<String> {
    let output = Command::new("nslookup")
        .arg(ip.to_string())
        .output()
        .await
        .ok()?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // 解析nslookup输出
        for line in output_str.lines() {
            if line.contains("name =") {
                if let Some(name) = line.split("name =").nth(1) {
                    return Some(name.trim().trim_end_matches('.').to_string());
                }
            }
        }
    }
    
    None
}
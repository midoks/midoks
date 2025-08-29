use crate::config::api_node::ApiNode;
use serde::Serialize;
use tonic::{Request, Status, metadata::MetadataValue};
use base64::{Engine as _, engine::general_purpose};

/// RPC认证中间件
pub struct AuthMiddleware;

#[derive(Serialize, Debug)]
struct EncJsonData {
    timestamp: String,
    r#type: String,
    user_id: u16,
}

impl AuthMiddleware {
    /// 验证请求头中的nodeId和secret
    pub fn verify_request<T>(request: &Request<T>) -> Result<(), Status> {
        let metadata = request.metadata();

        println!("metadata:{:?}", metadata);

        let node_id = metadata
            .get("node-id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("缺少node-id请求头"))?;

        let secret = metadata
            .get("secret")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("缺少secret请求头"))?;

        // 获取配置并验证凭据
        let api_node = ApiNode::instance()
            .map_err(|e| Status::internal(format!("configuration loading failed: {}", e)))?;

        let config = api_node.lock().unwrap();

        if !config.verify_credentials(node_id, secret) {
            return Err(Status::unauthenticated("无效的nodeId或secret"));
        }

        Ok(())
    }

    /// 为客户端请求添加认证头
    pub fn add_header_api<T>(mut request: Request<T>) -> Result<Request<T>, Status> {
        let api_node = ApiNode::instance()
            .map_err(|e| Status::internal(format!("configuration loading failed: {}", e)))?;

        let config = api_node.lock().unwrap();

        // 添加nodeId和secret到请求头
        let node_id = MetadataValue::try_from(&config.node_id)
            .map_err(|e| Status::internal(format!("nodeId格式错误: {}", e)))?;
        let secret = MetadataValue::try_from(&config.secret)
            .map_err(|e| Status::internal(format!("secret格式错误: {}", e)))?;

        println!("node-id:{:?}", node_id);
        println!("secret:{:?}", secret);

        request.metadata_mut().insert("node-id", node_id);
        request.metadata_mut().insert("secret", secret);

        Ok(request)
    }

    /// 添加管理员请求头信息
    pub fn add_header_admin<T>(mut request: Request<T>) -> Result<Request<T>, Status> {
        let api_node = ApiNode::instance()
            .map_err(|e| Status::internal(format!("configuration loading failed: {}", e)))?;

        let config = api_node.lock().unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let args = &EncJsonData {
            timestamp: timestamp.clone(),
            user_id: 0,
            r#type: "admin".to_string(),
        };

        println!("args: {:?}", args);
        let serialized = serde_json::to_string(&args)
            .map_err(|e| Status::internal(format!("serialization error: {}", e)))?;
        println!("serialized: {:?}", serialized);

        println!("{:?}", config);
        let cipher = crate::utils::aes::AesCfbCipher::new(256)
            .map_err(|e| Status::internal(format!("AES cipher creation failed: {}", e)))?;
        let token = cipher
            .encrypt(
                config.secret.as_bytes(),
                config.node_id.as_bytes(),
                serialized.as_bytes(),
            )
            .map_err(|e| Status::internal(format!("encryption failed: {}", e)))?;
        let token_base64 = general_purpose::STANDARD.encode(&token);
        println!("token base64: {}", token_base64);

        // 添加nodeId和secret到请求头
        let node_id = MetadataValue::try_from(&config.node_id)
            .map_err(|e| Status::internal(format!("node-id error: {}", e)))?;
        let secret = MetadataValue::try_from(&config.secret)
            .map_err(|e| Status::internal(format!("secret error: {}", e)))?;

        println!("node-id:{:?}", node_id);
        println!("secret:{:?}", secret);
        println!("timestamp:{:?}", timestamp);

        request.metadata_mut().insert("node-id", node_id);
        request.metadata_mut().insert("secret", secret);

        Ok(request)
    }
}

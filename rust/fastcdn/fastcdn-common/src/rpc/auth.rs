use crate::config::api_node::ApiNode;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use tonic::{Request, Status, metadata::MetadataValue};

/// RPC认证中间件
pub struct AuthMiddleware;

#[derive(Serialize, Deserialize, Debug)]
struct MetaDataHeader {
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
            .ok_or_else(|| Status::unauthenticated("missing node-id request header"))?;

        let token = metadata
            .get("token")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("missing token request header"))?;

        // 获取配置并验证凭据
        let api_node = ApiNode::instance()
            .map_err(|e| Status::internal(format!("verify_request loading failed: {}", e)))?;

        let config = api_node.lock().unwrap();

        if !config.verify_credentials(node_id, token) {
            return Err(Status::unauthenticated("invalid node-id or secret"));
        }

        Ok(())
    }

    pub async fn verify_admin_request<T>(request: &Request<T>) -> Result<(), Status> {
        let metadata = request.metadata();

        // println!("metadata:{:?}", metadata);
        let node_id = metadata
            .get("node-id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("missing node-id request header"))?;

        let token = metadata
            .get("token")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("missing token request header"))?;

        let header_token = general_purpose::STANDARD
            .decode(&token)
            .map_err(|e| Status::invalid_argument(format!("decode token failed: {}", e)))?;

        // 配置用于解密
        let config = crate::orm::api_token::get_by_node_id(node_id)
            .await
            .map_err(|e| Status::internal(format!("database query failed: {}", e)))?;

        if config.iter().len() < 1 {
            return Err(Status::invalid_argument("illegal node_id!"));
        }

        // 使用AES解密
        let cipher = crate::utils::aes::AesCfbCipher::new(256)
            .map_err(|e| Status::internal(format!("aes cipher creation failed: {}", e)))?;

        let secret = config[0]
            .get("secret")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Status::internal("secret field not found or not a string"))?;
        let node_id_str = config[0]
            .get("node_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Status::internal("node_id field not found or not a string"))?;

        let decrypted_header = cipher
            .decrypt(secret.as_bytes(), node_id_str.as_bytes(), &header_token)
            .map_err(|e| Status::invalid_argument(format!("decryption header failed: {}", e)))?;

        let header_jstr = String::from_utf8(decrypted_header)
            .map_err(|e| Status::invalid_argument(format!("header json utf8: {}", e)))?;
        let header: MetaDataHeader = serde_json::from_str(&header_jstr)
            .map_err(|e| Status::invalid_argument(format!("header json parse failed: {}", e)))?;

        // println!("header:{:?}", header);
        // 验证 token 类型
        if header.r#type != "admin" {
            return Err(Status::unauthenticated("invalid admin token type"));
        }
        Ok(())
    }

    /// 为客户端请求添加认证头
    pub fn add_header_api<T>(mut request: Request<T>) -> Result<Request<T>, Status> {
        let api_node = ApiNode::instance()
            .map_err(|e| Status::internal(format!("add_header_api loading failed: {}", e)))?;

        let config = api_node.lock().unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let args = &MetaDataHeader {
            timestamp: timestamp.clone(),
            r#type: "api".to_string(),
            user_id: 0,
        };

        let serialized = serde_json::to_string(&args)
            .map_err(|e| Status::internal(format!("serialization error: {}", e)))?;
        let cipher = crate::utils::aes::AesCfbCipher::new(256)
            .map_err(|e| Status::internal(format!("aes cipher creation failed: {}", e)))?;
        let data = cipher
            .encrypt(
                config.secret.as_bytes(),
                config.node_id.as_bytes(),
                serialized.as_bytes(),
            )
            .map_err(|e| Status::internal(format!("encryption failed: {}", e)))?;
        let token = general_purpose::STANDARD.encode(&data);

        // 添加nodeId和secret到请求头
        let node_id = MetadataValue::try_from(&config.node_id)
            .map_err(|e| Status::internal(format!("node-d error: {}", e)))?;
        let req_token = MetadataValue::try_from(&token)
            .map_err(|e| Status::internal(format!("token error: {}", e)))?;

        // println!("node-id:{:?}", node_id);
        // println!("token:{:?}", token);

        request.metadata_mut().insert("node-id", node_id);
        request.metadata_mut().insert("token", req_token);

        Ok(request)
    }

    /// 添加管理员请求头信息
    pub fn add_header_admin<T>(mut request: Request<T>) -> Result<Request<T>, Status> {
        let api_admin = crate::config::api_admin::ApiAdmin::instance()
            .map_err(|e| Status::internal(format!("add_header_admin loading failed: {}", e)))?;

        let config = api_admin.lock().unwrap();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let args = &MetaDataHeader {
            timestamp: timestamp.clone(),
            r#type: "admin".to_string(),
            user_id: 0,
        };

        let serialized = serde_json::to_string(&args)
            .map_err(|e| Status::internal(format!("serialization error: {}", e)))?;
        let cipher = crate::utils::aes::AesCfbCipher::new(256)
            .map_err(|e| Status::internal(format!("aes cipher creation failed: {}", e)))?;
        let data = cipher
            .encrypt(
                config.secret.as_bytes(),
                config.node_id.as_bytes(),
                serialized.as_bytes(),
            )
            .map_err(|e| Status::internal(format!("encryption failed: {}", e)))?;
        let token = general_purpose::STANDARD.encode(&data);

        // 添加nodeId和token到请求头
        let node_id = MetadataValue::try_from(&config.node_id)
            .map_err(|e| Status::internal(format!("node-id error: {}", e)))?;
        let req_token = MetadataValue::try_from(&token)
            .map_err(|e| Status::internal(format!("token error: {}", e)))?;

        // println!("node-id:{:?}", node_id);
        // println!("token:{:?}", token);

        request.metadata_mut().insert("node-id", node_id);
        request.metadata_mut().insert("token", req_token);

        Ok(request)
    }
}

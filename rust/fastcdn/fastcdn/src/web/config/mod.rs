

pub mod yaml_api_admin;
pub mod yaml_server;

/// 配置管理器
pub struct ConfigManager {
    server: yaml_server::Server,
    api_admin: Option<yaml_api_admin::ApiAdmin>,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let server = yaml_server::Server::load_default()?;
        server
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        Ok(ConfigManager {
            server,
            api_admin: None,
        })
    }

    /// 创建包含API管理员配置的配置管理器
    pub fn new_with_api_admin() -> Result<Self, Box<dyn std::error::Error>> {
        let server = yaml_server::Server::load_default()?;
        server
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let api_admin = yaml_api_admin::ApiAdmin::load_default()?;
        api_admin
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        Ok(ConfigManager {
            server,
            api_admin: Some(api_admin),
        })
    }

    /// 加载API管理员配置
    pub fn load_api_admin(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let api_admin = yaml_api_admin::ApiAdmin::load_default()?;
        api_admin
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        self.api_admin = Some(api_admin);
        Ok(())
    }

    /// 获取服务器配置
    pub fn server(&self) -> &yaml_server::Server {
        &self.server
    }

    /// 获取API管理员配置
    pub fn api_admin(&self) -> Option<&yaml_api_admin::ApiAdmin> {
        self.api_admin.as_ref()
    }

    /// 重新加载服务器配置
    pub fn reload_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let new_server = yaml_server::Server::load_default()?;
        new_server
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        self.server = new_server;
        Ok(())
    }

    /// 重新加载API管理员配置
    pub fn reload_api_admin(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.api_admin.is_some() {
            let new_api_admin = yaml_api_admin::ApiAdmin::load_default()?;
            new_api_admin
                .validate()
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
            self.api_admin = Some(new_api_admin);
        }
        Ok(())
    }

    /// 重新加载所有配置
    pub fn reload_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.reload_server()?;
        self.reload_api_admin()?;
        Ok(())
    }
}

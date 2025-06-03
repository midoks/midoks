# 1. 安装或更新 NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

# 2. 重新加载 shell 配置
source ~/.bashrc  # 或 source ~/.zshrc

# 3. 查看可用版本
nvm list-remote

# 4. 安装指定版本（如最新 LTS 版本）
nvm install --lts

# 或安装特定版本
nvm install 20.13.1

# 5. 验证版本
node -v
npm -v

# 6. 设置默认版本
nvm alias default 20.13.1
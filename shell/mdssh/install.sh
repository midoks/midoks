#!/usr/bin/env bash

set -Eeuo pipefail
umask 022

# mdssh v1 Linux 安装脚本。
# 使用：sudo bash scripts/install.sh

readonly RELEASE_TAG="init"
readonly RELEASE_VERSION="v1"
readonly REPOSITORY="midoks/midoks"
readonly INSTALL_DIR="/opt/mdssh"
readonly CONFIG_PATH="${INSTALL_DIR}/custom/conf/app.yaml"
readonly BINARY_LINK="/usr/local/bin/mdssh"
readonly SERVICE_NAME="mdssh.service"

log() {
    printf '[mdssh] %s\n' "$*"
}

fail() {
    printf '[mdssh] 安装失败：%s\n' "$*" >&2
    exit 1
}

[ "$(uname -s)" = "Linux" ] || fail "当前安装包仅支持 Linux。"
[ "$(id -u)" -eq 0 ] || fail "请使用 root 用户或 sudo 执行安装脚本。"

for command_name in curl tar sha256sum install mktemp; do
    command -v "${command_name}" >/dev/null 2>&1 || fail "缺少必要命令：${command_name}"
done

case "$(uname -m)" in
    x86_64 | amd64)
        ARCH="amd64"
        ARCHIVE_SHA256="e782be3bb6ba6a87f34702df38435bf0ae621787c49bffbc4a5e6097eb9a8245"
        ;;
    aarch64 | arm64)
        ARCH="arm64"
        ARCHIVE_SHA256="b3c3b4f923175cbed9172b3630e40b07168a189bee1b52a6e58dc1a422b5e031"
        ;;
    *)
        fail "暂不支持 $(uname -m) 架构，仅支持 x86_64 和 ARM64。"
        ;;
esac
readonly ARCH ARCHIVE_SHA256

ASSET_NAME="mdssh_${RELEASE_VERSION}_linux_${ARCH}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPOSITORY}/releases/download/${RELEASE_TAG}/${ASSET_NAME}"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/mdssh-install.XXXXXX")"
trap 'rm -rf -- "${WORK_DIR}"' EXIT

ARCHIVE_PATH="${WORK_DIR}/${ASSET_NAME}"
EXTRACT_DIR="${WORK_DIR}/extract"

log "下载 ${ASSET_NAME}..."
curl --fail --location --silent --show-error \
    --connect-timeout 15 --max-time 300 --retry 3 \
    --output "${ARCHIVE_PATH}" "${DOWNLOAD_URL}" \
    || fail "下载失败，请检查网络或稍后重试。"

log "校验安装包 SHA-256..."
printf '%s  %s\n' "${ARCHIVE_SHA256}" "${ARCHIVE_PATH}" | sha256sum --check --status \
    || fail "安装包校验未通过，已停止安装。"

mkdir -p "${EXTRACT_DIR}"
tar -xzf "${ARCHIVE_PATH}" -C "${EXTRACT_DIR}" --no-same-owner --no-same-permissions \
    || fail "安装包解压失败。"

BINARY_SOURCE=""
for candidate in "${EXTRACT_DIR}/mdssh" "${EXTRACT_DIR}"/*/mdssh; do
    if [ -f "${candidate}" ] && [ ! -L "${candidate}" ]; then
        BINARY_SOURCE="${candidate}"
        break
    fi
done
[ -n "${BINARY_SOURCE}" ] || fail "安装包中未找到 mdssh 可执行文件。"

# 下载和校验全部通过后再停止旧服务，避免升级失败影响正在运行的实例。
if command -v systemctl >/dev/null 2>&1 && systemctl is-active --quiet "${SERVICE_NAME}"; then
    log "停止旧版本服务..."
    systemctl stop "${SERVICE_NAME}" || fail "无法停止旧版本服务。"
fi

log "安装到 ${INSTALL_DIR}..."
install -d -m 0755 "${INSTALL_DIR}"
install -d -m 0700 "${INSTALL_DIR}/custom" "${INSTALL_DIR}/custom/conf" "${INSTALL_DIR}/custom/data"
install -m 0755 "${BINARY_SOURCE}" "${INSTALL_DIR}/mdssh"
ln -sfn "${INSTALL_DIR}/mdssh" "${BINARY_LINK}"

if command -v systemctl >/dev/null 2>&1 && [ -d /run/systemd/system ]; then
    log "注册并启动 systemd 服务..."
    (
        cd "${INSTALL_DIR}"
        "${INSTALL_DIR}/mdssh" -c "${CONFIG_PATH}" install
    )
else
    log "未检测到 systemd，已跳过服务注册。"
    log "可手动启动：cd ${INSTALL_DIR} && ./mdssh -c ${CONFIG_PATH} web"
fi

log "安装完成。"
log "程序目录：${INSTALL_DIR}"
log "配置文件：${CONFIG_PATH}"
log "首次使用请访问：http://服务器地址:9696/setup"
log "查看安全登录地址：${BINARY_LINK} -c ${CONFIG_PATH} default"

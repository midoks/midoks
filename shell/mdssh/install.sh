#!/usr/bin/env bash

set -Eeuo pipefail
umask 022

# mdssh v1 Linux 安装脚本。
# 使用：sudo bash scripts/install.sh

readonly RELEASE_TAG="init"
readonly RELEASE_VERSION="v1.2"
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

for command_name in curl tar sha256sum install mktemp od tr; do
    command -v "${command_name}" >/dev/null 2>&1 || fail "缺少必要命令：${command_name}"
done

case "$(uname -m)" in
    x86_64 | amd64)
        ARCH="amd64"
        ARCHIVE_SHA256="201f117424c56b2edfa517667f4f1d55f99f2281842ebd8f59a539f589f7851f"
        ;;
    aarch64 | arm64)
        ARCH="arm64"
        ARCHIVE_SHA256="90baf358c8f3dc617882fc7f803433ae31a4c75f1f739d9281aa5699fe45f252"
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

# 使用系统加密随机源生成 256 位初始化令牌，避免远程安装无需鉴权。
SETUP_TOKEN="$(od -An -N32 -tx1 /dev/urandom | tr -d ' \n')"
[ "${#SETUP_TOKEN}" -eq 64 ] || fail "初始化令牌生成失败。"
readonly SETUP_TOKEN
SETUP_ENV_FILE="${WORK_DIR}/setup-token.conf"
printf '[Service]\nEnvironment=MDSSH_SETUP_TOKEN=%s\n' "${SETUP_TOKEN}" >"${SETUP_ENV_FILE}"

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
# 全局命令使用启动器而不是软链接：固定工作目录和配置，避免在调用目录生成另一份 custom。
WRAPPER_FILE="${WORK_DIR}/mdssh-wrapper"
printf '#!/bin/sh\ncd "%s" || exit 1\nexec "%s" -c "%s" "$@"\n' \
    "${INSTALL_DIR}" "${INSTALL_DIR}/mdssh" "${CONFIG_PATH}" >"${WRAPPER_FILE}"
install -m 0755 "${WRAPPER_FILE}" "${BINARY_LINK}"

if command -v systemctl >/dev/null 2>&1 && [ -d /run/systemd/system ]; then
    log "注册并启动 systemd 服务..."
    # 在主服务启动前放入 override，确保首次启动即携带初始化令牌。
    SERVICE_OVERRIDE_DIR="/etc/systemd/system/${SERVICE_NAME}.d"
    install -d -m 0700 "${SERVICE_OVERRIDE_DIR}"
    install -m 0600 "${SETUP_ENV_FILE}" "${SERVICE_OVERRIDE_DIR}/setup-token.conf"
    (
        cd "${INSTALL_DIR}"
        "${INSTALL_DIR}/mdssh" -c "${CONFIG_PATH}" install
    )
else
    log "未检测到 systemd，已跳过服务注册。"
    log "可手动启动：cd ${INSTALL_DIR} && MDSSH_SETUP_TOKEN=${SETUP_TOKEN} ./mdssh -c ${CONFIG_PATH} web"
fi

log "安装完成。"
log "程序目录：${INSTALL_DIR}"
log "配置文件：${CONFIG_PATH}"
log "初始化令牌：${SETUP_TOKEN}"
log "首次使用请访问：http://服务器地址:9696/setup?token=${SETUP_TOKEN}"
log "查看安全登录地址：${BINARY_LINK} default"

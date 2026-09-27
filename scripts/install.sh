#!/usr/bin/env bash
set -euo pipefail

# --------------------------------------------------------------------------
# Configuration
# --------------------------------------------------------------------------
REPO="${CBA_REPO:-asifali411/cba}"
BIN_NAME="cba"
VERSION="${CBA_VERSION:-latest}"
INSTALL_DIR="${CBA_INSTALL_DIR:-}"

# --------------------------------------------------------------------------
# Helpers
# --------------------------------------------------------------------------
info()  { printf '\033[1;34m=>\033[0m %s\n' "$*"; }
warn()  { printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2; }
error() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || error "required command '$1' not found in PATH"
}

# --------------------------------------------------------------------------
# Parse arguments
# --------------------------------------------------------------------------
while getopts ":v:d:h" opt; do
  case "$opt" in
    v) VERSION="$OPTARG" ;;
    d) INSTALL_DIR="$OPTARG" ;;
    h)
      grep '^#' "$0" | sed -e 's/^#!.*//' -e 's/^# \{0,1\}//'
      exit 0
      ;;
    \?) error "invalid option: -$OPTARG" ;;
  esac
done

# --------------------------------------------------------------------------
# Detect platform
# --------------------------------------------------------------------------
need_cmd uname
need_cmd curl
need_cmd tar
need_cmd mktemp

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  PLATFORM="linux" ;;
  Darwin) PLATFORM="macos" ;;
  *) error "unsupported OS: $OS" ;;
esac

case "$ARCH" in
  x86_64|amd64)   ARCH_TAG="x86_64" ;;
  arm64|aarch64)  ARCH_TAG="aarch64" ;;
  *) error "unsupported architecture: $ARCH" ;;
esac

TARGET_NAME="${PLATFORM}-${ARCH_TAG}"
ARCHIVE="cba-${TARGET_NAME}.tar.gz"
CHECKSUM_FILE="${ARCHIVE}.sha256"

info "Detected platform: $TARGET_NAME"

# --------------------------------------------------------------------------
# Resolve download URLs
# --------------------------------------------------------------------------
if [ "$VERSION" = "latest" ]; then
  BASE_URL="https://github.com/${REPO}/releases/latest/download"
  info "Installing latest release of ${REPO}"
else
  BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"
  info "Installing ${REPO} ${VERSION}"
fi

ARCHIVE_URL="${BASE_URL}/${ARCHIVE}"
CHECKSUM_URL="${BASE_URL}/${CHECKSUM_FILE}"

# --------------------------------------------------------------------------
# Determine install directory
# --------------------------------------------------------------------------
if [ -z "$INSTALL_DIR" ]; then
  if [ -w "/usr/local/bin" ] 2>/dev/null; then
    INSTALL_DIR="/usr/local/bin"
  else
    INSTALL_DIR="${HOME}/.local/bin"
  fi
fi

mkdir -p "$INSTALL_DIR"

# --------------------------------------------------------------------------
# Download, verify, and install
# --------------------------------------------------------------------------
WORKDIR="$(mktemp -d)"
trap 'rm -rf "$WORKDIR"' EXIT

info "Downloading ${ARCHIVE_URL}"
if ! curl -fsSL -o "${WORKDIR}/${ARCHIVE}" "$ARCHIVE_URL"; then
  error "failed to download archive. Does release '${VERSION}' contain an asset for ${TARGET_NAME}?"
fi

info "Downloading checksum"
if curl -fsSL -o "${WORKDIR}/${CHECKSUM_FILE}" "$CHECKSUM_URL"; then
  info "Verifying checksum"
  (
    cd "$WORKDIR"
    if command -v sha256sum >/dev/null 2>&1; then
      sha256sum -c "$CHECKSUM_FILE" --ignore-missing >/dev/null 2>&1 \
        || { shasum -a 256 -c "$CHECKSUM_FILE" 2>/dev/null; } \
        || {
          EXPECTED="$(awk '{print $1}' "$CHECKSUM_FILE")"
          ACTUAL="$(sha256sum "$ARCHIVE" | awk '{print $1}')"
          [ "$EXPECTED" = "$ACTUAL" ] || error "checksum verification failed"
        }
    else
      EXPECTED="$(awk '{print $1}' "$CHECKSUM_FILE")"
      ACTUAL="$(shasum -a 256 "$ARCHIVE" | awk '{print $1}')"
      [ "$EXPECTED" = "$ACTUAL" ] || error "checksum verification failed"
    fi
  )
  info "Checksum OK"
else
  warn "checksum file not found, skipping verification"
fi

info "Extracting archive"
tar -xzf "${WORKDIR}/${ARCHIVE}" -C "$WORKDIR"

[ -f "${WORKDIR}/${BIN_NAME}" ] || error "binary '${BIN_NAME}' not found in archive"

chmod +x "${WORKDIR}/${BIN_NAME}"
mv "${WORKDIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"

info "Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

# --------------------------------------------------------------------------
# PATH check
# --------------------------------------------------------------------------
case ":$PATH:" in
  *":${INSTALL_DIR}:"*) ;;
  *)
    warn "${INSTALL_DIR} is not in your PATH."
    warn "Add this to your shell profile (e.g. ~/.bashrc, ~/.zshrc):"
    warn "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    ;;
esac

if command -v "$BIN_NAME" >/dev/null 2>&1; then
  info "Done. Run '${BIN_NAME} --version' to verify."
else
  info "Done. Restart your shell or update PATH, then run '${BIN_NAME} --version' to verify."
fi
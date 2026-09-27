#!/usr/bin/env bash
set -euo pipefail

# --------------------------------------------------------------------------
# Configuration
# --------------------------------------------------------------------------
BIN_NAME="cba"
INSTALL_DIR="${CBA_INSTALL_DIR:-}"
FORCE=0

# --------------------------------------------------------------------------
# Helpers
# --------------------------------------------------------------------------
info()  { printf '\033[1;34m=>\033[0m %s\n' "$*"; }
warn()  { printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2; }
error() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

usage() {
  cat <<EOF
Usage: $(basename "$0") [-d install_dir] [-y] [-h]

  -d install_dir   Directory to remove '${BIN_NAME}' from (skips auto-detection)
  -y               Don't prompt for confirmation
  -h               Show this help message
EOF
}

# --------------------------------------------------------------------------
# Parse arguments
# --------------------------------------------------------------------------
while getopts ":d:yh" opt; do
  case "$opt" in
    d) INSTALL_DIR="$OPTARG" ;;
    y) FORCE=1 ;;
    h) usage; exit 0 ;;
    \?) error "invalid option: -$OPTARG" ;;
  esac
done

# --------------------------------------------------------------------------
# Locate the installed binary
# --------------------------------------------------------------------------
CANDIDATES=()

if [ -n "$INSTALL_DIR" ]; then
  CANDIDATES+=("${INSTALL_DIR}/${BIN_NAME}")
else
  # Prefer whatever is actually on PATH, then fall back to common locations.
  if command -v "$BIN_NAME" >/dev/null 2>&1; then
    CANDIDATES+=("$(command -v "$BIN_NAME")")
  fi
  CANDIDATES+=(
    "/usr/local/bin/${BIN_NAME}"
    "${HOME}/.local/bin/${BIN_NAME}"
  )
fi

FOUND=""
for path in "${CANDIDATES[@]}"; do
  if [ -f "$path" ]; then
    FOUND="$path"
    break
  fi
done

if [ -z "$FOUND" ]; then
  error "could not find an installed '${BIN_NAME}' binary. Use -d to specify its directory."
fi

info "Found ${BIN_NAME} at: ${FOUND}"

# --------------------------------------------------------------------------
# Confirm and remove
# --------------------------------------------------------------------------
if [ "$FORCE" -ne 1 ]; then
  printf 'Remove this file? [y/N] '
  read -r REPLY
  case "$REPLY" in
    [yY]|[yY][eE][sS]) ;;
    *) info "Aborted."; exit 0 ;;
  esac
fi

if [ -w "$(dirname "$FOUND")" ]; then
  rm -f "$FOUND"
else
  warn "no write permission for $(dirname "$FOUND"), retrying with sudo"
  sudo rm -f "$FOUND"
fi

if [ -f "$FOUND" ]; then
  error "failed to remove ${FOUND}"
fi

info "Uninstalled ${BIN_NAME} from ${FOUND}"

# --------------------------------------------------------------------------
# Note about PATH entries left in shell profiles
# --------------------------------------------------------------------------
warn "If the installer added a PATH export for ${BIN_NAME} to your shell profile"
warn "(e.g. ~/.bashrc, ~/.zshrc), you may want to remove that line manually."
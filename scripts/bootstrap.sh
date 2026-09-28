#!/usr/bin/env bash
# Setheum bootstrap for Linux distributions and macOS.
#
# Detects the tools this repository needs and installs whatever is missing.
# Designed to be idempotent and safe to re-run.
#
# Usage:
#   bash scripts/bootstrap.sh
#   bash scripts/bootstrap.sh --check
#
# Invoked automatically by:  mise run init
#
# Supported Linux families: debian/ubuntu, arch, fedora, rhel/centos/rocky,
# opensuse/sles. Other distributions receive manual instructions.

set -o pipefail

CHECK=0
for arg in "$@"; do
    case "$arg" in
        --check) CHECK=1 ;;
        -h|--help)
            sed -n '2,16p' "$0" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
    esac
done

# ---------------------------------------------------------------- output helpers
step() { printf '\n== %s ==\n' "$1"; }
ok()   { printf '  [ok]      %s\n' "$1"; }
miss() { printf '  [missing] %s\n' "$1"; }
note() { printf '  [note]    %s\n' "$1"; }
warn() { printf '  [warn]    %s\n' "$1"; }
act()  { printf '  [install] %s\n' "$1"; }

have() { command -v "$1" >/dev/null 2>&1; }

MISSING=()

# ---------------------------------------------------------------- OS detection
OS="$(uname -s)"
DISTRO=""
DISTRO_LIKE=""

if [ "$OS" = "Linux" ] && [ -r /etc/os-release ]; then
    # shellcheck disable=SC1091
    . /etc/os-release
    DISTRO="${ID:-}"
    DISTRO_LIKE="${ID_LIKE:-}"
fi

step 'Operating system'
note "kernel: $OS"
if [ "$OS" = "Darwin" ]; then
    note "macOS $(sw_vers -productVersion 2>/dev/null)"
elif [ "$OS" = "Linux" ]; then
    note "distro: ${DISTRO:-unknown} ${VERSION_ID:-} (like: ${DISTRO_LIKE:-none})"
fi
[ "$CHECK" = 1 ] && note 'check-only mode: nothing will be installed'

SUDO=""
if [ "$(id -u)" -ne 0 ] && have sudo; then
    SUDO="sudo"
fi

run() {
    if [ "$CHECK" = 1 ]; then
        note "would run: $*"
    else
        note "running: $*"
        "$@"
    fi
}

# ---------------------------------------------------------------- package manager
install_linux_packages() {
    if [ "$CHECK" = 1 ]; then
        note "would install packages: $*"
        return 0
    fi
    case "$DISTRO $DISTRO_LIKE" in
        *debian*|*ubuntu*)
            $SUDO apt-get update && $SUDO apt-get install -y "$@" ;;
        *arch*)
            $SUDO pacman -Sy --needed --noconfirm "$@" ;;
        *rhel*|*fedora*|*centos*)
            if have dnf; then $SUDO dnf install -y "$@"; else $SUDO yum install -y "$@"; fi ;;
        *suse*|*opensuse*)
            $SUDO zypper --non-interactive install "$@" ;;
        *)
            warn "unsupported distribution '${DISTRO:-unknown}'; install manually: $*"
            return 1 ;;
    esac
}

install_macos_packages() {
    if ! have brew; then
        warn 'Homebrew not found; install it from https://brew.sh and re-run'
        return 1
    fi
    if [ "$CHECK" = 1 ]; then
        note "would install packages: $*"
        return 0
    fi
    brew install "$@"
}

# ---------------------------------------------------------------- native tooling
step 'Native build tooling'

APT_PKGS="build-essential clang llvm libclang-dev protobuf-compiler cmake nasm pkg-config libssl-dev git curl"
PACMAN_PKGS="base-devel clang llvm protobuf cmake nasm pkgconf openssl git curl"
DNF_PKGS="clang llvm clang-devel protobuf-compiler cmake nasm pkgconfig openssl-devel git curl"
ZYPPER_PKGS="clang llvm-devel protobuf-devel cmake nasm pkg-config libopenssl-devel git curl"
BREW_PKGS="llvm protobuf cmake nasm pkg-config openssl git"

need_native=0
for c in clang protoc cmake nasm perl; do
    if have "$c"; then
        ok "$c present"
    else
        miss "$c"
        need_native=1
    fi
done

if [ "$need_native" = 1 ]; then
    if [ "$OS" = "Darwin" ]; then
        install_macos_packages $BREW_PKGS
        LLVM_PREFIX="$(brew --prefix llvm 2>/dev/null)"
        if [ -n "$LLVM_PREFIX" ] && [ -d "$LLVM_PREFIX/bin" ]; then
            note "add to your shell profile:"
            note "  export PATH=\"$LLVM_PREFIX/bin:\$PATH\""
            note "  export LIBCLANG_PATH=\"$LLVM_PREFIX/lib\""
        fi
    else
        case "$DISTRO $DISTRO_LIKE" in
            *debian*|*ubuntu*)  install_linux_packages $APT_PKGS ;;
            *arch*)             install_linux_packages $PACMAN_PKGS ;;
            *rhel*|*fedora*|*centos*) install_linux_packages $DNF_PKGS ;;
            *suse*|*opensuse*)  install_linux_packages $ZYPPER_PKGS ;;
            *) warn "cannot determine packages for '${DISTRO:-unknown}'" ;;
        esac
    fi
else
    ok 'native build tooling complete'
fi

# ---------------------------------------------------------------- rust toolchains
step 'Rust toolchains and WASM targets'

if ! have rustup; then
    if [ "$CHECK" = 1 ]; then
        miss 'rustup'
        MISSING+=('rustup')
    else
        act 'rustup via rustup.rs'
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # shellcheck disable=SC1091
        [ -r "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
    fi
else
    ok 'rustup present'
fi

if have rustup; then
    toolchains="$(rustup toolchain list 2>/dev/null | awk '{print $1}')"
    for spec in '1.88.0|rustfmt,clippy,rust-src|wasm32-unknown-unknown wasm32v1-none' \
                'nightly-2024-02-14|rustfmt,clippy,rust-src|wasm32-unknown-unknown'; do
        tc="${spec%%|*}"
        rest="${spec#*|}"
        comps="${rest%%|*}"
        targets="${rest#*|}"
        if printf '%s\n' "$toolchains" | grep -qx "$tc"; then
            ok "toolchain $tc"
        elif [ "$CHECK" = 1 ]; then
            miss "toolchain $tc"
            MISSING+=("rust $tc")
        else
            run rustup toolchain install "$tc" --profile minimal --component "$comps"
        fi
        if [ "$CHECK" != 1 ] && have rustup; then
            # shellcheck disable=SC2086
            rustup target add $targets --toolchain "$tc" >/dev/null 2>&1 || true
        fi
    done
fi

# ---------------------------------------------------------------- mise
step 'mise (task runner / tool manager)'
if have mise; then
    ok "mise $(mise --version 2>/dev/null | awk '{print $1}')"
elif [ "$CHECK" = 1 ]; then
    miss 'mise'
    MISSING+=('mise')
else
    act 'mise via mise.run'
    curl https://mise.run | sh
    note 'ensure ~/.local/bin is on PATH'
fi

# ---------------------------------------------------------------- summary
step 'Summary'
if [ "${#MISSING[@]}" -eq 0 ]; then
    ok 'all required tooling is present'
    exit 0
fi
warn "missing / manual action required: ${MISSING[*]}"
exit 1

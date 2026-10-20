#!/bin/sh
set -eu

# the AppImage unpacks itself in the cool like /multi-game so i changed it the with the -git so it would not have like problems and stuff.
INSTALL_DIR="$HOME/.local/share/multi-game-git"

download(){
    echo "Downloading $1"
}

has(){
    command -v "$1" >/dev/null 2>&1
}
log(){
    echo "$1"
}

main(){
    echo "installing multi-game into $INSTALL_DIR"

    os=$(uname -s)

    case "$os" in
        Linux)
            echo "Linux detected"
            ;;
        Darwin)
            echo "MacOS detected"
            ;;
        FreeBSD|DragonFly)
            echo "you are on $os"
            ;;
        OpenBSD)
            echo "OpenBSD detected"
            ;;
        NetBSD)
            echo "NetBSD detected"
            ;;
         *)
            echo "Unsupported OS: $os"
            exit 1
            ;;
    esac
    if has pacman; then
        PM="pacman"
    elif has apt-get; then
        PM="apt-get"
    elif has dnf; then
        PM="dnf"
    elif has zypper; then
        PM="zypper"
    elif has apk; then
        PM="apk"
    elif has xbps-install; then
        PM="xbps-install"
    elif has emerge; then
        PM="emerge"
    elif has eopkg; then
        PM="eopkg"
    elif has pkg; then
        PM="pkg"
    elif has pkg_add; then
        PM="pkg_add"
    elif has pkgin; then
        PM="pkgin"
    elif has nix-shell; then
        PM="nix"
    elif has guix; then
        PM="guix"
    else
        # only a problem if something is missing that gets checked further down
        PM=""
    fi

    missing=""
    
    has git || missing="$missing git"
    if ! has java || ! has javac; then
        missing="$missing java"
    fi
    has cargo || missing="$missing cargo"

    if has python3; then
        PY="python3"
    elif has python; then
        PY="python"
    else
        PY=""
        missing="$missing python"
    fi

    if [ -z "$PY" ] || ! "$PY" -c "import pygame" >/dev/null 2>&1; then
        missing="$missing pygame"
    fi

    log "missing:$missing"

    if has sudo; then
        SUDO="sudo"
    elif has doas; then
        SUDO="doas"
    elif [ "$(id -u)" -eq 0 ]; then
        SUDO=""
    elif [ -n "$missing" ]; then
        log "could not find a supported privilege escalation tool"
        log "please install these yourself: git, python3, pygame, java, rust"
        log "then run this installer again"
        exit 1
    fi

    if [ -z "$missing" ]; then
        log "everything is already installed"
    elif [ "$PM" = "pacman" ]; then
        command -v $SUDO >/dev/null 2>&1 || { log "sudo is not installed, please install it and rerun" >&2; exit 1; }
        if ! has python3; then
            log "python3 is not installed, installing it"
            $SUDO pacman -S --needed --noconfirm python
        fi
        if ! has java || ! has javac; then
            log "java is not installed, installing it"
            $SUDO pacman -S --needed --noconfirm jdk-openjdk
        fi
        if ! has cargo; then
            log "cargo is not installed, installing it"
            $SUDO pacman -S --needed --noconfirm rust
        fi
        if ! "$PY" -c "import pygame" >/dev/null 2>&1; then
            log "pygame is not installed, installing it"
            $SUDO pacman -S --needed --noconfirm python-pygame
        fi

    elif [ "$PM" = "apt-get" ]; then
        command -v $SUDO >/dev/null 2>&1 || { log "sudo is not installed, please install it and rerun" >&2; exit 1; }
        if ! has python3; then
            log "python3 is not installed, installing it"
            $SUDO apt-get update
            $SUDO apt-get install -y python3
        fi
        if ! has java || ! has javac; then
            log "java is not installed, installing it"
            $SUDO apt-get update
            $SUDO apt-get install -y default-jdk
        fi
        if ! has cargo; then
            log "cargo is not installed, installing it"
            $SUDO apt-get update
            $SUDO apt-get install -y cargo
        fi
        if ! "$PY" -c "import pygame" >/dev/null 2>&1; then
            log "pygame is not installed, installing it"
            $SUDO apt-get update
            $SUDO apt-get install -y python3-pygame
        fi
    
    else
        log "please install these yourself: git, python3, pygame, java, rust"
        log "then run this installer again"
        exit 1
    fi

    # check that gits installed cus we need to 
    if ! has git; then
        log "git is not installed, please install it and rerun" >&2
        exit 1
    fi

    if [ -e "$INSTALL_DIR/.git" ]; then
        log "updating multi-game"
        git -C "$INSTALL_DIR" pull --ff-only
    elif [ -e "$INSTALL_DIR" ]; then
        log "$INSTALL_DIR exists but isn't a multi-game checkout remove it and rerun" >&2
        exit 1
    else
        log "cloning multi-game"
        git clone https://github.com/Ben8282/multi-game.git "$INSTALL_DIR"
    fi

    # compile the Coooool code
    log "building multi-game, the first build can take a few minutes"
    cargo build --release --locked --manifest-path "$INSTALL_DIR/Cargo.toml"

    # make the multi-game command so you can type multi-game in your terminal anywhere to play
    mkdir -p "$HOME/.local/bin"
    cat > "$HOME/.local/bin/multi-game" <<EOF
#!/bin/sh
cd "$INSTALL_DIR" || exit 1
./target/release/multi-game
EOF
    chmod +x "$HOME/.local/bin/multi-game"
}
main
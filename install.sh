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

# asks gentoo users if they want prebuilt binary packages or to compile from source
# keeps asking until they type 1 or 0, then saves it in USE_BIN (1 = binary, 0 = source)
ask_bin(){
    while true; do
        printf "do you want binary packages (1) or to compile from source (0)? "
        read -r answer </dev/tty
        case "$answer" in
            1|0)
                USE_BIN="$answer"
                return
                ;;
            *)
                log "please type 1 or 0"
                ;;
        esac
    done
}

main(){
    trap 'status=$?; [ "$status" -eq 0 ] || log "install failed (exit code $status), see the error above"' EXIT
    echo "installing multi-game into $INSTALL_DIR"

    os=$(uname -s)

    case "$os" in
        Linux)
            echo "Linux detected"
            ;;
        Darwin)
            echo "MacOS detected"
            PATH="/opt/homebrew/bin:$PATH"
            if ! has brew; then
                log "brew is not installed, installing it"
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" </dev/tty
            fi
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
    elif has brew; then
        PM="brew"
    elif has nix-shell; then
        PM="nix"
    elif has guix; then
        PM="guix"
    else
        # only a problem if something is missing that gets checked further down
        PM=""
    fi

    missing=""
    
    # run git instead of just checking that it exists cus macOS has a fake one that exists even when git isn't installed
    git --version >/dev/null 2>&1 || missing="$missing git"
    # run java and javac instead of just checking that they exist because macOS has fake ones that exist even when java isn't installed
    if ! java -version >/dev/null 2>&1 || ! javac -version >/dev/null 2>&1; then
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

    if { [ -z "$PY" ] || ! "$PY" -c "import pygame" >/dev/null 2>&1; } &&
        ! "$INSTALL_DIR/.venv/bin/python3" -c "import pygame" >/dev/null 2>&1; then
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
        if ! has git; then
            log "git is not installed, installing it"
            $SUDO pacman -S --needed --noconfirm git
        fi
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
        if ! has git; then
            log "git is not installed, installing it"
            $SUDO apt-get update
            $SUDO apt-get install -y git
        fi
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
        # use rustup instead of apt cus debian and ubuntu ship a rust thats too old for edition 2024 (needs 1.85+)
        if ! has cargo; then
            log "cargo is not installed, installing it with rustup"
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            . "$HOME/.cargo/env"
        fi
        if ! "$PY" -c "import pygame" >/dev/null 2>&1; then
            log "pygame is not installed, installing it"
            $SUDO apt-get update
            $SUDO apt-get install -y python3-pygame
        fi

    elif [ "$PM" = "brew" ]; then
        # run git instead of just checking it exists cus macOS has a fake one like java
        if ! git --version >/dev/null 2>&1; then
            log "git is not installed, installing it"
            brew install git
        fi
        if ! java -version >/dev/null 2>&1 || ! javac -version >/dev/null 2>&1; then
            log "java is not installed, installing it"
            brew install --cask temurin
        fi
        if ! has cargo; then
            log "cargo is not installed, installing it"
            brew install rust
        fi
        if [ ! -x "$(brew --prefix)/bin/python3" ]; then
            log "brew's python is not installed, installing it"
            brew install python
        fi
    
    elif [ "$PM" = "emerge" ]; then
        command -v $SUDO >/dev/null 2>&1 || { log "sudo is not installed, please install it and rerun" >&2; exit 1; }
        # only java and rust have binary versions so only ask if one of them is missing
        if ! has java || ! has javac || ! has cargo; then
            ask_bin
        fi
        # --ask=n so emerge just installs without asking and we dont just do nothing in case --ask is turned on in make.conf
        if ! has git; then
            log "git is not installed, installing it"
            $SUDO emerge --noreplace --ask=n dev-vcs/git
        fi
        if ! has python3; then
            log "python3 is not installed, installing it"
            $SUDO emerge --noreplace --ask=n dev-lang/python
            PY="python3"
        fi
        if ! has java || ! has javac; then
            log "java is not installed, installing it"
            if [ "$USE_BIN" = 1 ]; then
                $SUDO emerge --noreplace --ask=n dev-java/openjdk-bin
            else
                $SUDO emerge --noreplace --ask=n dev-java/openjdk
            fi
        fi
        if ! has cargo; then
            log "cargo is not installed, installing it"
            if [ "$USE_BIN" = 1 ]; then
                $SUDO emerge --noreplace --ask=n dev-lang/rust-bin
            else
                $SUDO emerge --noreplace --ask=n dev-lang/rust
            fi
        fi

    else
        log "please install these yourself: git, python3, pygame, java, rust"
        log "then run this installer again"
        exit 1
    fi

    # check that gits installed cus we need to
    if ! git --version >/dev/null 2>&1; then
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

    if [ "$PM" = "brew" ] || [ "$PM" = "emerge" ]; then
        if [ "$PM" = "brew" ]; then
            VENV_PY="$(brew --prefix)/bin/python3"
        else
            VENV_PY="$PY"
        fi
        if ! "$VENV_PY" -c "import pygame" >/dev/null 2>&1 &&
            ! "$INSTALL_DIR/.venv/bin/python3" -c "import pygame" >/dev/null 2>&1; then
            log "pygame is not installed, installing pygame-ce"
            "$VENV_PY" -m venv "$INSTALL_DIR/.venv"
            "$INSTALL_DIR/.venv/bin/python3" -m pip install pygame-ce
        fi
    fi

    # compile the Coooool code
    log "building multi-game, the first build can take a few minutes"
    cargo build --release --locked --manifest-path "$INSTALL_DIR/Cargo.toml"

    # make the multi-game command so you can type multi-game in your terminal anywhere to play
    mkdir -p "$HOME/.local/bin"
    cat > "$HOME/.local/bin/multi-game" <<EOF
#!/bin/sh
cd "$INSTALL_DIR" || exit 1
PATH="$INSTALL_DIR/.venv/bin:\$PATH"
./target/release/multi-game
EOF
    chmod +x "$HOME/.local/bin/multi-game"
        if [ "$os" = "Darwin" ]; then
            log "adding a multi-game shortcut to /usr/local/bin, this needs your password"
            sudo mkdir -p /usr/local/bin
            sudo ln -sf "$HOME/.local/bin/multi-game" /usr/local/bin/multi-game
        fi
    log "done! type multi-game to play"
}
main
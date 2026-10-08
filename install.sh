#!/bin/sh
set -eu

INSTALL_DIR="$HOME/.local/share/multi-game"

download(){
    echo "Downloading $1"
}

has(){
    command -v "$1" >/dev/null 2>&1
}
main(){
    echo "installing multi-game into $INSTALL_DIR"

    os=$(uname -s)
    if has git; then
        echo "git is installed, cloning repo"
    else
        echo "git is not installed"
    fi
}